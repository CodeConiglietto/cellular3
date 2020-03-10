use mutagen::{Generatable, Mutatable};
use std::f32::consts::PI;

use rand::prelude::*;

use crate::util::*;

#[derive(Clone, Copy, Debug)]
pub struct UNFloat {
    value: f32,
}

impl UNFloat {
    pub fn new_unchecked(value: f32) -> Self {
        Self { value }
    }

    pub fn new(value: f32) -> Self {
        assert!(
            value >= 0.0 && value <= 1.0,
            "Invalid UNFloat value: {}",
            value
        );
        Self::new_unchecked(value)
    }

    pub fn new_from_range(value: f32, min: f32, max: f32) -> Self {
        Self::new_unchecked(map_range(value, (min, max), (0.0, 1.0)))
    }

    pub fn into_inner(self) -> f32 {
        self.value
    }

    pub fn to_angle(self) -> Angle {
        Angle::new_from_range(self.value, 0.0, 1.0)
    }

    pub fn to_signed(self) -> SNFloat {
        SNFloat::new_from_range(self.value, 0.0, 1.0)
    }
}

impl Generatable for UNFloat {
    fn generate_rng<R: Rng + ?Sized>(rng: &mut R, _state: mutagen::State) -> Self {
        Self::new_unchecked(rng.gen_range(0.0, 1.0))
    }
}

impl Mutatable for UNFloat {
    fn mutate_rng<R: Rng + ?Sized>(&mut self, rng: &mut R, state: mutagen::State) {
        *self = Self::generate_rng(rng, state);
    }
}

#[derive(Clone, Copy, Debug)]
pub struct SNFloat {
    value: f32,
}

impl SNFloat {
    pub fn new_unchecked(value: f32) -> Self {
        Self { value }
    }

    pub fn new(value: f32) -> Self {
        assert!(
            value >= -1.0 && value <= 1.0,
            "Invalid SNFloat value: {}",
            value
        );

        Self::new_unchecked(value)
    }

    pub fn new_from_range(value: f32, min: f32, max: f32) -> Self {
        Self::new_unchecked(map_range(value, (min, max), (-1.0, 1.0)))
    }

    pub fn into_inner(self) -> f32 {
        self.value
    }

    pub fn to_angle(self) -> Angle {
        Angle::new_from_range(self.value, -1.0, 1.0)
    }

    pub fn to_unsigned(self) -> UNFloat {
        UNFloat::new_from_range(self.value, -1.0, 1.0)
    }

    pub fn circular_add(self, other: SNFloat) -> SNFloat {
        self.circular_add_f32(other.into_inner())
    }

    //TODO: There is a bug here, the circular add will probably reset to 0 when overflowed instead of -1 or 1
    pub fn circular_add_f32(self, other: f32) -> SNFloat {
        let total = self.into_inner() + other;
        let sign = total.signum();
        SNFloat::new((total.abs() - total.abs().floor()) * sign)
    }
}

impl Generatable for SNFloat {
    fn generate_rng<R: Rng + ?Sized>(rng: &mut R, _state: mutagen::State) -> Self {
        Self::new_unchecked(rng.gen_range(-1.0, 1.0))
    }
}

impl Mutatable for SNFloat {
    fn mutate_rng<R: Rng + ?Sized>(&mut self, rng: &mut R, state: mutagen::State) {
        *self = Self::generate_rng(rng, state);
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Angle {
    value: f32,
}

impl Angle {
    pub fn new_unchecked(value: f32) -> Self {
        Self { value }
    }

    pub fn new(value: f32) -> Self {
        let normalised = value - 2.0 * PI * (value / (2.0 * PI)).floor();

        debug_assert!(
            normalised >= 0.0 && normalised < 2.0 * PI,
            "Failed to normalize angle: {} -> {}",
            value,
            normalised,
        );

        Self::new_unchecked(normalised)
    }

    pub fn new_from_range(value: f32, min: f32, max: f32) -> Self {
        Self::new_unchecked(map_range(value, (min, max), (0.0, 2.0 * PI)))
    }

    pub fn into_inner(self) -> f32 {
        self.value
    }

    pub fn to_signed(self) -> SNFloat {
        SNFloat::new_from_range(self.value, 0.0, 2.0 * PI)
    }

    pub fn to_unsigned(self) -> UNFloat {
        UNFloat::new_from_range(self.value, 0.0, 2.0 * PI)
    }
}

impl Generatable for Angle {
    fn generate_rng<R: Rng + ?Sized>(rng: &mut R, _state: mutagen::State) -> Self {
        Angle::new_unchecked(rng.gen_range(0.0, 2.0 * PI))
    }
}

impl Mutatable for Angle {
    fn mutate_rng<R: Rng + ?Sized>(&mut self, rng: &mut R, state: mutagen::State) {
        *self = Self::generate_rng(rng, state);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_angles() {
        for i in 0..100_000 {
            Angle::new(i as f32);
        }
    }
}
