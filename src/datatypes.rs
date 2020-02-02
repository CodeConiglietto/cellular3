use std::f32::consts::PI;

use rand::prelude::*;

#[derive(Clone, Copy, Debug)]
pub struct UnsignedFloatNormalised {
    value: f32,
}

impl UnsignedFloatNormalised {
    pub fn new_unchecked(value: f32) -> Self {
        Self { value }
    }

    pub fn new(value: f32) -> Self {
        assert!(value >= 0.0);
        assert!(value <= 1.0);

        Self::new_unchecked(value)
    }

    pub fn new_from_range(value: f32, min: f32, max: f32) -> Self {
        Self::new_unchecked(map_range(value, (min, max), (0.0, 1.0)))
    }

    pub fn random() -> Self {
        Self::new_unchecked(thread_rng().gen_range(0.0, 1.0))
    }

    pub fn into_inner(self) -> f32 {
        self.value
    }

    pub fn to_angle(self) -> Angle {
        Angle::new_from_range(self.value, 0.0, 1.0)
    }

    pub fn to_signed(self) -> SignedFloatNormalised {
        SignedFloatNormalised::new_from_range(self.value, 0.0, 1.0)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct SignedFloatNormalised {
    value: f32,
}

impl SignedFloatNormalised {
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

    pub fn random() -> Self {
        Self::new_unchecked(thread_rng().gen_range(-1.0, 1.0))
    }

    pub fn into_inner(self) -> f32 {
        self.value
    }

    pub fn to_angle(self) -> Angle {
        Angle::new_from_range(self.value, -1.0, 1.0)
    }

    pub fn to_unsigned(self) -> UnsignedFloatNormalised {
        UnsignedFloatNormalised::new_from_range(self.value, -1.0, 1.0)
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
        let normalised = value - 2.0 * PI * (value / 2.0 * PI).floor();

        debug_assert!(normalised >= 0.0);
        debug_assert!(normalised < 2.0 * PI);

        Self::new_unchecked(normalised)
    }

    pub fn new_from_range(value: f32, min: f32, max: f32) -> Self {
        Self::new_unchecked(map_range(value, (min, max), (0.0, 2.0 * PI)))
    }

    pub fn random() -> Self {
        Self::new_unchecked(thread_rng().gen_range(0.0, 2.0 * PI))
    }

    pub fn into_inner(self) -> f32 {
        self.value
    }

    pub fn to_signed(self) -> SignedFloatNormalised {
        SignedFloatNormalised::new_from_range(self.value, 0.0, 2.0 * PI)
    }

    pub fn to_unsigned(self) -> UnsignedFloatNormalised {
        UnsignedFloatNormalised::new_from_range(self.value, 0.0, 2.0 * PI)
    }
}

#[inline(always)]
fn map_range(value: f32, from: (f32, f32), to: (f32, f32)) -> f32 {
    let (from_min, from_max) = from;
    let (to_min, to_max) = to;

    assert!(from_min < from_max);
    assert!(from_min <= value);
    assert!(value <= from_max);
    assert!(to_min < to_max);

    let out = ((value - from_min) / (from_max - from_min)) * (to_max - to_min) + to_min;

    debug_assert!(to_min <= out);
    debug_assert!(out <= to_max);

    out
}