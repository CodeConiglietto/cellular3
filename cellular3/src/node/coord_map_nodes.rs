use crate::{
    datatype::continuous::*,
    node::{continuous_nodes::*, discrete_nodes::*, mutagen_functions::*, Node},
    updatestate::{CoordinateSet, UpdateState},
};
use mutagen::{Generatable, Mutatable};
use nalgebra::{geometry::Rotation2, geometry::Point2};

#[derive(Generatable, Mutatable, Debug)]
#[mutagen(mut_reroll = 0.1)]
pub enum CoordMapNodes {
    #[mutagen(gen_weight = branch_node_weight)]
    Shift {
        x: Box<SNFloatNodes>,
        y: Box<SNFloatNodes>,
    },

    #[mutagen(gen_weight = branch_node_weight)]
    Scale {
        x: Box<SNFloatNodes>,
        y: Box<SNFloatNodes>,
    },

    #[mutagen(gen_weight = pipe_node_weight)]
    Rotation { angle: Box<AngleNodes> },

    #[mutagen(gen_weight = leaf_node_weight)]
    ToPolar,

    #[mutagen(gen_weight = leaf_node_weight)]
    FromPolar,
    #[mutagen(gen_weight = branch_node_weight)]
    IfElse {
        predicate: Box<BooleanNodes>,
        child_a: Box<Self>,
        child_b: Box<Self>,
    },
}

impl Node for CoordMapNodes {
    type Output = CoordinateSet;

    fn compute(&self, state: UpdateState) -> Self::Output {
        use CoordMapNodes::*;

        match self {
            Shift { x, y } => state.coordinate_set.get_coord_shifted(
                x.compute(state),
                y.compute(state),
                SNFloat::new(0.0),
            ),
            Scale { x, y } => state.coordinate_set.get_coord_scaled(
                x.compute(state),
                y.compute(state),
                SNFloat::new(1.0),
            ),
            Rotation { angle } => { 
                let new_pos = Rotation2::new(angle.compute(state).into_inner()).transform_point(&Point2::new(state.coordinate_set.x.into_inner(), state.coordinate_set.y.into_inner()));

                CoordinateSet {
                    x: SNFloat::new(0.0).circular_add_f32(new_pos.x), 
                    y: SNFloat::new(0.0).circular_add_f32(new_pos.y), 
                    t: state.coordinate_set.t
                }
            },
            ToPolar => {
                let state_x = state.coordinate_set.x.into_inner();
                let state_y = state.coordinate_set.y.into_inner();

                CoordinateSet {
                    //Represents the angle from 0.0..2PI
                    //atan2(y, x) is correct, but it's more visually appealing to have the axis of symmetry along the vertical axis
                    //Sorry if this makes me a bad person :<
                    x: Angle::new(f32::atan2(-state_x, state_y)).to_signed(),
                    //Represents the radius between 0.0..1.0
                    y: SNFloat::new(
                        f32::sqrt(
                            state.coordinate_set.x.into_inner().powf(2.0)
                                + state.coordinate_set.y.into_inner().powf(2.0),
                        )
                        .min(1.0),
                    ),
                    t: state.coordinate_set.t,
                }
            }
            FromPolar => CoordinateSet {
                x: SNFloat::new(
                    state.coordinate_set.y.into_inner()
                        * f32::cos(state.coordinate_set.x.into_inner()),
                ),
                y: SNFloat::new(
                    state.coordinate_set.y.into_inner()
                        * f32::sin(state.coordinate_set.x.into_inner()),
                ),
                t: state.coordinate_set.t,
            },
            IfElse {
                predicate,
                child_a,
                child_b,
            } => {
                if predicate.compute(state).into_inner() {
                    child_a.compute(state)
                } else {
                    child_b.compute(state)
                }
            }
        }
    }
}
