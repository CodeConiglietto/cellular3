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
    fn generate_rng<R: Rng + ?Sized>(rng: &mut R, state: mutagen::State) -> Self {
        Boolean { value: rng.gen() }
    }
}

impl Mutatable for Boolean {
    fn mutate_rng<R: Rng + ?Sized>(&mut self, rng: &mut R, state: mutagen::State) {
        *self = Self::generate_rng(rng, state);
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Byte {
    pub value: u8,
}

impl Byte {
    pub fn new(value: u8) -> Self {
        Self { value }
    }

    pub fn into_inner(self) -> u8 {
        self.value
    }
}

impl Generatable for Byte {
    fn generate_rng<R: Rng + ?Sized>(rng: &mut R, state: mutagen::State) -> Self {
        Byte {
            value: rng.gen::<u8>(),
        }
    }
}

impl Mutatable for Byte {
    fn mutate_rng<R: Rng + ?Sized>(&mut self, rng: &mut R, state: mutagen::State) {
        *self = Self::generate_rng(rng, state);
    }
}
