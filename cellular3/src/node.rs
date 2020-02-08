pub mod color_nodes;
pub mod noise_nodes;
pub mod primitive_nodes;
pub mod coord_map_nodes;

use crate::updatestate::UpdateState;

pub trait Node {
    type Output;

    fn compute(&self, state: UpdateState) -> Self::Output;
}
