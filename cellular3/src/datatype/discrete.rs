use std::num::Wrapping;

use mutagen::{Generatable, Mutatable};

use crate::constants::*;
use rand::prelude::*;

#[derive(Clone, Copy, Debug)]
pub struct Boolean {
    pub value: bool,
}

impl Boolean {
    pub fn into_inner(self) -> bool {
        self.value
    }
}

impl Generatable for Boolean {
    fn generate_rng<R: Rng + ?Sized>(rng: &mut R, _state: mutagen::State) -> Self {
        Boolean { value: rng.gen() }
    }
}

impl Mutatable for Boolean {
    fn mutate_rng<R: Rng + ?Sized>(&mut self, rng: &mut R, state: mutagen::State) {
        *self = Self::generate_rng(rng, state);
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Nibble {
    pub value: u8,
}

impl Nibble {
    pub fn new(value: u8) -> Self {
        Self {
            value: value % CONSTS.nibble_possible_values,
        }
    }

    pub fn into_inner(self) -> u8 {
        self.value
    }

    pub fn add(self, other: Self) -> Self {
        Self::new(self.value + other.value)
    }

    pub fn divide(self, other: Self) -> Self {
        if other.value == 0 {
            Self::new(other.value)
        } else {
            Self::new(self.value / other.value)
        }
    }

    pub fn multiply(self, other: Self) -> Self {
        Self::new(self.value * other.value)
    }

    pub fn modulus(self, other: Self) -> Self {
        if other.value == 0 {
            Self::new(other.value)
        } else {
            Self::new(self.value % other.value)
        }
    }
}

impl Generatable for Nibble {
    fn generate_rng<R: Rng + ?Sized>(rng: &mut R, _state: mutagen::State) -> Self {
        Nibble::new(rng.gen())
    }
}

impl Mutatable for Nibble {
    fn mutate_rng<R: Rng + ?Sized>(&mut self, rng: &mut R, state: mutagen::State) {
        *self = Self::generate_rng(rng, state);
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Byte {
    pub value: Wrapping<u8>,
}

impl Byte {
    pub fn new(value: u8) -> Self {
        Self {
            value: Wrapping(value),
        }
    }

    pub fn into_inner(self) -> u8 {
        self.value.0
    }

    pub fn add(self, other: Self) -> Self {
        Self::new((self.value + other.value).0)
    }

    pub fn divide(self, other: Self) -> Self {
        if other.value.0 == 0 {
            Self::new(other.value.0)
        } else {
            Self::new((self.value / other.value).0)
        }
    }

    pub fn multiply(self, other: Self) -> Self {
        Self::new((self.value * other.value).0)
    }

    pub fn modulus(self, other: Self) -> Self {
        if other.value.0 == 0 {
            Self::new(other.value.0)
        } else {
            Self::new((self.value % other.value).0)
        }
    }
}

impl Generatable for Byte {
    fn generate_rng<R: Rng + ?Sized>(rng: &mut R, _state: mutagen::State) -> Self {
        Byte { value: rng.gen() }
    }
}

impl Mutatable for Byte {
    fn mutate_rng<R: Rng + ?Sized>(&mut self, rng: &mut R, state: mutagen::State) {
        *self = Self::generate_rng(rng, state);
    }
}

#[derive(Clone, Copy, Debug)]
pub struct UInt {
    pub value: Wrapping<u32>,
}

impl UInt {
    pub fn new(value: u32) -> Self {
        Self {
            value: Wrapping(value),
        }
    }

    pub fn into_inner(self) -> u32 {
        self.value.0
    }

    pub fn add(self, other: Self) -> Self {
        Self::new((self.value + other.value).0)
    }

    pub fn divide(self, other: Self) -> Self {
        if other.value.0 == 0 {
            Self::new(other.value.0)
        } else {
            Self::new((self.value / other.value).0)
        }
    }

    pub fn multiply(self, other: Self) -> Self {
        Self::new((self.value * other.value).0)
    }

    pub fn modulus(self, other: Self) -> Self {
        if other.value.0 == 0 {
            Self::new(other.value.0)
        } else {
            Self::new((self.value % other.value).0)
        }
    }
}

impl Generatable for UInt {
    fn generate_rng<R: Rng + ?Sized>(rng: &mut R, _state: mutagen::State) -> Self {
        UInt { value: rng.gen() }
    }
}

impl Mutatable for UInt {
    fn mutate_rng<R: Rng + ?Sized>(&mut self, rng: &mut R, state: mutagen::State) {
        *self = Self::generate_rng(rng, state);
    }
}

#[derive(Clone, Copy, Debug)]
pub struct SInt {
    pub value: Wrapping<i32>,
}

impl SInt {
    pub fn new(value: i32) -> Self {
        Self {
            value: Wrapping(value),
        }
    }

    pub fn into_inner(self) -> i32 {
        self.value.0
    }

    pub fn add(self, other: Self) -> Self {
        Self::new((self.value + other.value).0)
    }

    pub fn divide(self, other: Self) -> Self {
        if other.value.0 == 0 {
            Self::new(other.value.0)
        } else {
            Self::new((self.value / other.value).0)
        }
    }

    pub fn multiply(self, other: Self) -> Self {
        Self::new((self.value * other.value).0)
    }

    pub fn modulus(self, other: Self) -> Self {
        if other.value.0 == 0 {
            Self::new(other.value.0)
        } else {
            Self::new((self.value % other.value).0)
        }
    }
}

impl Generatable for SInt {
    fn generate_rng<R: Rng + ?Sized>(rng: &mut R, _state: mutagen::State) -> Self {
        SInt { value: rng.gen() }
    }
}

impl Mutatable for SInt {
    fn mutate_rng<R: Rng + ?Sized>(&mut self, rng: &mut R, state: mutagen::State) {
        *self = Self::generate_rng(rng, state);
    }
}
