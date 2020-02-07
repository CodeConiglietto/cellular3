use crate::
{
    updatestate::{UpdateState, CoordinateSet},
    node::{Node, primitive_nodes::*},
};
use mutagen::{Generatable, Mutatable};

#[derive(Generatable, Mutatable, Debug)]
#[mutagen(mut_reroll = 0.1)]
pub enum CoordMapNodes{
    ShiftX { child: Box<SNFloatNodes> },
    ShiftY { child: Box<SNFloatNodes> },
    ShiftT { child: Box<SNFloatNodes> },
    ScaleX { child: Box<UNFloatNodes> },
    ScaleY { child: Box<UNFloatNodes> },
    ScaleT { child: Box<UNFloatNodes> },
}

impl Node for CoordMapNodes{
    type Output = CoordinateSet;

    fn compute(&self, state: UpdateState) -> Self::Output {
        use CoordMapNodes::*;

        match self{
            ShiftX { child } => state.coordinate_set.get_coord_shifted(child.compute(state).into_inner(), 0.0, 0.0),
            ShiftY { child } => state.coordinate_set.get_coord_shifted(0.0, child.compute(state).into_inner(), 0.0),
            ShiftT { child } => state.coordinate_set.get_coord_shifted(0.0, 0.0, child.compute(state).into_inner()),
            ScaleX { child } => state.coordinate_set.get_coord_shifted(child.compute(state).into_inner(), 0.0, 0.0),
            ScaleY { child } => state.coordinate_set.get_coord_shifted(0.0, child.compute(state).into_inner(), 0.0),
            ScaleT { child } => state.coordinate_set.get_coord_shifted(0.0, 0.0, child.compute(state).into_inner()),
        }
    }
}
