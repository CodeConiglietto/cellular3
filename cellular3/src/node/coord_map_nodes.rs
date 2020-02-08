use crate::{
    node::{primitive_nodes::*, Node},
    updatestate::{CoordinateSet, UpdateState},
    datatype::continuous::*,
};
use mutagen::{Generatable, Mutatable};

#[derive(Generatable, Mutatable, Debug)]
#[mutagen(mut_reroll = 0.1)]
pub enum CoordMapNodes {
    Shift { x: Box<SNFloatNodes>,  y: Box<SNFloatNodes> },
    Scale { x: Box<SNFloatNodes>,  y: Box<SNFloatNodes> },
    ToPolar,
    FromPolar,
}

impl Node for CoordMapNodes {
    type Output = CoordinateSet;

    fn compute(&self, state: UpdateState) -> Self::Output {
        use CoordMapNodes::*;

        match self {
            Shift { x, y } => {
                state
                    .coordinate_set
                    .get_coord_shifted(
                        x.compute(state), 
                        y.compute(state), 
                        SNFloat::new(0.0))
            },
            Scale { x, y } => {
                state
                    .coordinate_set
                    .get_coord_scaled(
                        x.compute(state), 
                        y.compute(state), 
                        SNFloat::new(1.0))
            },
            ToPolar => 
                CoordinateSet
                {
                    //Represents the angle from 0.0..1.0
                    x: SNFloat::new(f32::atan(state.coordinate_set.y.into_inner() / state.coordinate_set.x.into_inner())), 
                    //Represents the radius between 0.0..1.0
                    y: SNFloat::new(f32::sqrt(
                        state.coordinate_set.x.into_inner().powf(2.0) + 
                        state.coordinate_set.y.into_inner().powf(2.0))),
                    t: state.coordinate_set.t
                },
            FromPolar => 
                CoordinateSet
                {
                    x: SNFloat::new(state.coordinate_set.y.into_inner() * f32::cos(state.coordinate_set.x.into_inner())), 
                    y: SNFloat::new(state.coordinate_set.y.into_inner() * f32::sin(state.coordinate_set.x.into_inner())), 
                    t: state.coordinate_set.t
                }
        }
    }
}
