use crate::{
    datatype::continuous::*,
    node::{mutagen_functions::*, primitive_nodes::*, Node},
    updatestate::{CoordinateSet, UpdateState},
};
use mutagen::{Generatable, Mutatable};

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

    #[mutagen(gen_weight = leaf_node_weight)]
    ToPolar,

    #[mutagen(gen_weight = leaf_node_weight)]
    FromPolar,
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
            ToPolar => 
            {
                let state_x = state.coordinate_set.x.into_inner();
                let state_y = state.coordinate_set.y.into_inner();

                CoordinateSet
                {
                    //Represents the angle from 0.0..1.0
                    //atan2(y, x) is correct. Don't ask me why.
                    x: Angle::new(
                        f32::atan2(
                            state_y, 
                            state_x)).to_signed(), 
                    //Represents the radius between 0.0..1.0
                    y: SNFloat::new(f32::sqrt(
                        state.coordinate_set.x.into_inner().powf(2.0) + 
                        state.coordinate_set.y.into_inner().powf(2.0)).min(1.0)),
                    t: state.coordinate_set.t
                }},
            FromPolar => 
                CoordinateSet
                {
                    x: SNFloat::new((state.coordinate_set.y.into_inner() * f32::cos(state.coordinate_set.x.into_inner())).min(1.0).max(-1.0)), 
                    y: SNFloat::new((state.coordinate_set.y.into_inner() * f32::sin(state.coordinate_set.x.into_inner())).min(1.0).max(-1.0)), 
                    t: state.coordinate_set.t
                }
        }
    }
}
