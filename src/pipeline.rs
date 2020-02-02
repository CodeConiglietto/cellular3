use crate::{colors::PalletteColor, nodes::Node};
use ndarray::prelude::*;

pub struct Pipeline {
    pub root_node: Box<dyn Node<Output = PalletteColor> + Sync>,
}

pub fn run_pipeline(pipeline: Pipeline, cell_array: &mut Array2<PalletteColor>) {
    let cell_array_width = cell_array.dim().0;
    let cell_array_height = cell_array.dim().1;

    for x in 0..cell_array_width {
        for y in 0..cell_array_height {
            cell_array[[x, y]] = pipeline.root_node.compute(x, y, 10.0);
        }
    }
}
