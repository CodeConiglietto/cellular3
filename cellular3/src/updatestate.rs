use crate::colors::*;
use ndarray::prelude::*;

#[derive(Clone, Copy, Debug)]
pub struct UpdateState<'a> {
    //coordinates of update position
    pub x: usize,
    pub y: usize,
    //current gametic
    pub t: i32,
    //cell array to read from
    pub cell_array: ArrayView2<'a, FloatColor>,
}
