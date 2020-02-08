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
    pub x: SNFloat,
    pub y: SNFloat,
    //current game sync tic
    pub t: f32,
}

impl CoordinateSet {
    pub fn get_coord_shifted(self, shift_x: SNFloat, shift_y: SNFloat, shift_t: SNFloat) -> Self {
        CoordinateSet {
            x: SNFloat::new(self.x.into_inner() + shift_x.into_inner()),
            y: SNFloat::new(self.y.into_inner() + shift_y.into_inner()),
            t: self.t + shift_t.into_inner(),
        }
    }
    
    pub fn get_coord_scaled(self, scale_x: SNFloat, scale_y: SNFloat, scale_t: SNFloat) -> Self {
        CoordinateSet {
            x: SNFloat::new(self.x.into_inner() * scale_x.into_inner()),
            y: SNFloat::new(self.y.into_inner() * scale_y.into_inner()),
            t: self.t * scale_t.into_inner(),
        }
    }

    pub fn get_byte_t(&self) -> Byte {
        Byte::new((self.t as u64 % BYTE_POSSIBLE_VALUES as u64) as u8)
    }

    pub fn get_unfloat_t(&self) -> UNFloat {
        UNFloat::new(self.get_byte_t().into_inner() as f32 / BYTE_POSSIBLE_VALUES as f32)
    }
}
