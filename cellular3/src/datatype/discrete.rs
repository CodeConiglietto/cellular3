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
    fn generate_rng<R: Rng + ?Sized>(_rng: &mut R) -> Self {
        Boolean {
            value: thread_rng().gen::<bool>(),
        }
    }
}

impl Mutatable for Boolean {
    fn mutate_rng<R: Rng + ?Sized>(&mut self, _rng: &mut R) {
        *self = Self::generate();
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Byte {
    pub value: usize,
}

impl Byte {
    pub fn new(value: usize) -> Self {
        assert!(value <= BYTE_MAX_VALUE);

        Self{value: value}
    }

    pub fn into_inner(self) -> usize {
        self.value
    }
}

impl Generatable for Byte {
    fn generate_rng<R: Rng + ?Sized>(_rng: &mut R) -> Self {
        Byte {
            value: thread_rng().gen::<usize>() % BYTE_POSSIBLE_VALUES,
        }
    }
}

impl Mutatable for Byte {
    fn mutate_rng<R: Rng + ?Sized>(&mut self, _rng: &mut R) {
        *self = Self::generate();
    }
}