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
    fn generate_rng<R: Rng + ?Sized>(rng: &mut R, _state: mutagen::State) -> Self {
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

    pub fn add(self, other: Self) -> Self {
        Self::new(self.into_inner() + other.into_inner())
    }

    pub fn divide(self, other: Self) -> Self {
        let other_val = other.into_inner();
        
        if other_val == 0
        {
            Self::new(other_val)
        }else{
            Self::new(self.into_inner() / other.into_inner())
        }
    }

    pub fn multiply(self, other: Self) -> Self {
        Self::new(self.into_inner() * other.into_inner())
    }

    pub fn modulus(self, other: Self) -> Self {
        let other_val = other.into_inner();
        
        if other_val == 0
        {
            Self::new(other_val)
        }else{
            Self::new(self.into_inner() % other.into_inner())
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
    pub value: u32,
}

impl UInt {
    pub fn new(value: u32) -> Self{
        Self { value: value }
    }

    pub fn into_inner(self) -> u32 {
        self.value
    }

    pub fn add(self, other: Self) -> Self {
        Self::new(self.into_inner() + other.into_inner())
    }

    pub fn divide(self, other: Self) -> Self {
        let other_val = other.into_inner();
        
        if other_val == 0
        {
            Self::new(other_val)
        }else{
            Self::new(self.into_inner() / other.into_inner())
        }
    }

    pub fn multiply(self, other: Self) -> Self {
        Self::new(self.into_inner() * other.into_inner())
    }

    pub fn modulus(self, other: Self) -> Self {
        let other_val = other.into_inner();
        
        if other_val == 0
        {
            Self::new(other_val)
        }else{
            Self::new(self.into_inner() / other.into_inner())
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
    pub value: i32,
}

impl SInt {
    pub fn new(value: i32) -> Self{
        Self { value: value }
    }

    pub fn into_inner(self) -> i32 {
        self.value
    }

    pub fn add(self, other: Self) -> Self {
        Self::new(self.into_inner() + other.into_inner())
    }

    pub fn divide(self, other: Self) -> Self {
        let other_val = other.into_inner();
        
        if other_val == 0
        {
            Self::new(other_val)
        }else{
            Self::new(self.into_inner() / other.into_inner())
        }
    }

    pub fn multiply(self, other: Self) -> Self {
        Self::new(self.into_inner() * other.into_inner())
    }

    pub fn modulus(self, other: Self) -> Self {
        let other_val = other.into_inner();
        
        if other_val == 0
        {
            Self::new(other_val)
        }else{
            Self::new(self.into_inner() / other.into_inner())
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