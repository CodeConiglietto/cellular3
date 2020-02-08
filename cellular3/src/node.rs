pub mod color_nodes;
pub mod coord_map_nodes;
pub mod noise_nodes;
pub mod primitive_nodes;

use crate::updatestate::UpdateState;

pub trait Node {
    type Output;

    fn compute(&self, state: UpdateState) -> Self::Output;
}

mod mutagen_functions {
    use mutagen::State;

    use crate::constants::*;

    pub fn leaf_node_weight(state: &mutagen::State) -> f64 {
        if state.depth == 0 {
            0.0
        } else {
            1.0
        }
    }

    pub fn pipe_node_weight(state: &mutagen::State) -> f64 {
        if state.depth == 0 {
            0.0
        } else {
            1.0 - usize::min(state.depth, MAX_TREE_DEPTH) as f64 / MAX_TREE_DEPTH as f64
        }
    }

    pub fn branch_node_weight(state: &mutagen::State) -> f64 {
        1.0 - usize::min(state.depth, MAX_TREE_DEPTH / 2) as f64 / MAX_TREE_DEPTH as f64
    }
}
