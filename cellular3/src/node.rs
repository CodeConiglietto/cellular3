pub mod color_blend_nodes;
pub mod color_nodes;
pub mod continuous_nodes;
pub mod coord_map_nodes;
pub mod discrete_nodes;
pub mod noise_nodes;

use crate::updatestate::UpdateState;

pub trait Node {
    type Output;

    fn compute(&self, state: UpdateState) -> Self::Output;
}

mod mutagen_functions {
    use crate::{constants::*, util::*};

    pub fn leaf_node_weight(state: &mutagen::State) -> f64 {
        if state.depth < MIN_LEAF_DEPTH || state.depth > MAX_LEAF_DEPTH {
            0.0
        } else {
            map_range(
                state.depth as f32,
                (MIN_LEAF_DEPTH as f32, MAX_LEAF_DEPTH as f32),
                (0.0, 1.0),
            ) as f64
        }
    }

    pub fn pipe_node_weight(state: &mutagen::State) -> f64 {
        if state.depth < MIN_PIPE_DEPTH || state.depth > MAX_PIPE_DEPTH {
            0.0
        } else {
            1.0 - map_range(
                state.depth as f32,
                (MIN_PIPE_DEPTH as f32, MAX_PIPE_DEPTH as f32),
                (0.0, 1.0),
            ) as f64
        }
    }

    pub fn branch_node_weight(state: &mutagen::State) -> f64 {
        if state.depth < MIN_BRANCH_DEPTH || state.depth > MAX_BRANCH_DEPTH {
            0.0
        } else {
            1.0 - map_range(
                state.depth as f32,
                (MIN_BRANCH_DEPTH as f32, MAX_BRANCH_DEPTH as f32),
                (0.0, 1.0),
            ) as f64
        }
    }
}
