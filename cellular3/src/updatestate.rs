use crate::{
    constants::*,
    datatype::{colors::FloatColor, continuous::*, discrete::*},
};
use ndarray::prelude::*;

#[derive(Clone, Copy, Debug)]
pub struct UpdateState<'a> {
    //the set of coordinates for the update
    pub coordinate_set: CoordinateSet,
    //cell array to read from
    pub cell_array: ArrayView2<'a, FloatColor>,
}

#[derive(Clone, Copy, Debug)]
pub struct CoordinateSet {
    //coordinates of update position
    //Needs to be floating point to allow for proper scaling
    pub x: f32,
    pub y: f32,
    //current game sync tic
    pub t: f32,
}

impl CoordinateSet {
    pub fn get_coord_shifted(self, shift_x: f32, shift_y: f32, shift_t: f32) -> Self {
        CoordinateSet {
            x: self.x + shift_x,
            y: self.y + shift_y,
            t: self.t + shift_t,
        }
    }

    pub fn get_byte_t(&self) -> Byte {
        Byte::new(self.t as usize % BYTE_POSSIBLE_VALUES)
    }

    pub fn get_unfloat_t(&self) -> UNFloat {
        UNFloat::new(self.get_byte_t().into_inner() as f32 / BYTE_POSSIBLE_VALUES as f32)
    }
}
