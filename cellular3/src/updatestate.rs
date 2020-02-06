use crate::datatype::colors::FloatColor;
use ndarray::prelude::*;

#[derive(Clone, Copy, Debug)]
pub struct UpdateState<'a> {
    //coordinates of update position
    pub x: f32,
    pub y: f32,
    //current gametic
    pub t: f32,
    //cell array to read from
    pub cell_array: ArrayView2<'a, FloatColor>,
}
