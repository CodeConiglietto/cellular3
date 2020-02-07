pub mod color_nodes;
pub mod noise_nodes;
pub mod primitive_nodes;
pub mod state_nodes;

use crate::updatestate::UpdateState;

pub trait Node {
    type Output;

    fn compute(&self, state: UpdateState) -> Self::Output;
}
