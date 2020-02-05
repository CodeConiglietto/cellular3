use mutagen::{Generatable, Mutatable};
use std::f32::consts::PI;

use rand::prelude::*;

#[derive(Clone, Copy, Debug)]
pub struct UNFloat {
    value: f32,
}

impl UNFloat {
    pub fn new_unchecked(value: f32) -> Self {
        Self { value }
    }

    pub fn new(value: f32) -> Self {
        assert!(value >= 0.0, "value: {}", value);
        assert!(value <= 1.0, "value: {}", value);

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
    fn generate_rng<R: Rng + ?Sized>(rng: &mut R) -> Self {
        Self::new_unchecked(thread_rng().gen_range(0.0, 1.0))
    }
}

impl Mutatable for UNFloat {
    fn mutate_rng<R: Rng + ?Sized>(&mut self, rng: &mut R) {
        *self = Self::generate();
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
        assert!(value >= -1.0);
        assert!(value <= 1.0);

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
}

impl Generatable for SNFloat {
    fn generate_rng<R: Rng + ?Sized>(rng: &mut R) -> Self {
        Self::new_unchecked(thread_rng().gen_range(-1.0, 1.0))
    }
}

impl Mutatable for SNFloat {
    fn mutate_rng<R: Rng + ?Sized>(&mut self, rng: &mut R) {
        *self = Self::generate_rng(rng);
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
        SNFloat::new_from_range(self.value, -1.0 * PI, PI)
    }

    pub fn to_unsigned(self) -> UNFloat {
        UNFloat::new_from_range(self.value, 0.0, 2.0 * PI)
    }
}

impl Generatable for Angle {
    fn generate_rng<R: Rng + ?Sized>(rng: &mut R) -> Self {
        Angle::new_unchecked(rng.gen_range(0.0, 2.0 * PI))
    }
}

impl Mutatable for Angle {
    fn mutate_rng<R: Rng + ?Sized>(&mut self, rng: &mut R) {
        *self = Self::generate_rng(rng);
    }
}

#[inline(always)]
fn map_range(value: f32, from: (f32, f32), to: (f32, f32)) -> f32 {
    let (from_min, from_max) = from;
    let (to_min, to_max) = to;

    assert!(
        from_min < from_max,
        "from_min: {}, from_max: {}",
        from_min,
        from_max
    );
    assert!(
        from_min <= value,
        "from_min: {}, value: {}",
        from_min,
        value
    );
    assert!(
        value <= from_max,
        "value: {}, from_max: {}",
        value,
        from_max
    );
    assert!(to_min < to_max, "to_min: {}, to_max: {}", to_min, to_max);

    let out = ((value - from_min) / (from_max - from_min)) * (to_max - to_min) + to_min;

    debug_assert!(to_min <= out);
    debug_assert!(out <= to_max);

    out
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
