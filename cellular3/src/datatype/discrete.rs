use crate::constants::*;
use mutagen::{Generatable, Mutatable};

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
    fn generate_rng<R: Rng + ?Sized>(rng: &mut R) -> Self {
        Boolean { value: rng.gen() }
    }
}

impl Mutatable for Boolean {
    fn mutate_rng<R: Rng + ?Sized>(&mut self, rng: &mut R) {
        *self = Self::generate_rng(rng);
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Byte {
    pub value: usize,
}

impl Byte {
    pub fn new(value: usize) -> Self {
        assert!(value <= BYTE_MAX_VALUE);

        Self { value }
    }

    pub fn into_inner(self) -> usize {
        self.value
    }
}

impl Generatable for Byte {
    fn generate_rng<R: Rng + ?Sized>(rng: &mut R) -> Self {
        Byte {
            value: rng.gen_range(0, BYTE_POSSIBLE_VALUES),
        }
    }
}

impl Mutatable for Byte {
    fn mutate_rng<R: Rng + ?Sized>(&mut self, rng: &mut R) {
        *self = Self::generate_rng(rng);
    }
}
