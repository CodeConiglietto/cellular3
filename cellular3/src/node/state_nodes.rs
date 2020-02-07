// use crate::
// {
//     updatestate::UpdateState,
//     node::{Node, primitive_nodes::*},
// };
// use mutagen::{Generatable, Mutatable};

// #[derive(Generatable, Mutatable, Debug)]
// #[mutagen(mut_reroll = 0.1)]
// pub enum StateNodes{
//     ShiftX { child: Box<SNFloatNodes> },
//     ShiftY { child: Box<SNFloatNodes> },
//     ShiftT { child: Box<SNFloatNodes> },
//     ScaleX { child: Box<UNFloatNodes> },
//     ScaleY { child: Box<UNFloatNodes> },
//     ScaleT { child: Box<UNFloatNodes> },
// }

// impl Node for StateNodes{
//     type Output = UpdateState<'_>;

//     fn compute(&self, state: UpdateState) -> Self::Output {
//         use StateNodes::*;

//         match self{
//             ShiftX { child } => state.get_coord_shifted(shift_x: child.compute(state).into_inner(), shift_y: 0.0, shift_t: 0.0),
//             ShiftY { child } => state.get_coord_shifted(shift_x: 0.0, shift_y: child.compute(state).into_inner(), shift_t: 0.0),
//             ShiftT { child } => state.get_coord_shifted(shift_x: 0.0, shift_y: 0.0, shift_t: child.compute(state).into_inner()),
//             ScaleX { child } => state.get_coord_shifted(shift_x: child.compute(state).into_inner(), shift_y: 0.0, shift_t: 0.0),
//             ScaleY { child } => state.get_coord_shifted(shift_x: 0.0, shift_y: child.compute(state).into_inner(), shift_t: 0.0),
//             ScaleT { child } => state.get_coord_shifted(shift_x: 0.0, shift_y: 0.0, shift_t: child.compute(state).into_inner()),
//         }
//     }
// }
