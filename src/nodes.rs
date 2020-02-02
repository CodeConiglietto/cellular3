use crate::{
    colors::*,
    constants::*,
    datatypes::{Angle, SignedFloatNormalised, UnsignedFloatNormalised},
};
use ndarray::prelude::*;
use noise::{NoiseFn, OpenSimplex, RangeFunction, Worley};
use rand::prelude::*;

pub trait Node {
    type Output;

    fn compute(&self, x: usize, y: usize, t: f64) -> Self::Output;
}

pub enum PalletteColorNodes {
    //Red,
    Modulus {
        x_mod: usize,
        y_mod: usize,
        x_offset: usize,
        y_offset: usize,
        color_table: Array2<PalletteColor>,
    },
    SimplexNoise {
        noise: OpenSimplex,
    },
    Worley {
        noise: Worley,
    },
    ComboNoise {
        n1: OpenSimplex,
        n2: Worley,
    },
    // DecomposeToComponents{
    //     //r:
    // }
}

impl Node for PalletteColorNodes {
    type Output = PalletteColor;

    fn compute(&self, x: usize, y: usize, t: f64) -> Self::Output {
        match self {
            //ColorNodes::Red => PalletteColor::Red,
            PalletteColorNodes::Modulus {
                x_mod,
                y_mod,
                x_offset,
                y_offset,
                color_table,
            } => {
                let x_index = if (x + x_offset) % x_mod == 0 { 1 } else { 0 };
                let y_index = if (y + y_offset) % y_mod == 0 { 1 } else { 0 };

                color_table[[x_index, y_index]]
            }
            PalletteColorNodes::SimplexNoise { noise } => {
                let noise_value =
                    (noise.get([x as f64 * 0.025, y as f64 * 0.025, t as f64 * 0.1])) + 0.5;
                PalletteColor::from_index((noise_value * (MAX_COLORS - 1) as f64) as usize)
            }
            PalletteColorNodes::Worley { noise } => {
                let noise_value = noise
                    .get([x as f64 * 0.025, y as f64 * 0.025, t as f64 * 0.1])
                    .abs();
                PalletteColor::from_index((noise_value * (MAX_COLORS - 1) as f64) as usize)
            }
            PalletteColorNodes::ComboNoise { n1, n2 } => {
                let noise_value1 =
                    n1.get([x as f64 * 0.025, y as f64 * 0.025, t as f64 * 0.1]) + 0.5;
                let noise_value2 = n2
                    .get([x as f64 * 0.025, y as f64 * 0.025, t as f64 * 0.1])
                    .abs();

                PalletteColor::from_index(
                    (noise_value1 * noise_value2 * (MAX_COLORS - 1) as f64) as usize,
                )
            }
        }
    }
}

// pub enum RangeFunctionNodes {
//     RangeFunction { function: RangeFunction },
// }

// pub enum NeighbourhoodNodes {

// }

pub enum FloatNodes {
    Tan { child: Box<AngleNodes> },
    Constant { value: f32 },
}

impl Node for FloatNodes {
    type Output = f32;

    fn compute(&self, x: usize, y: usize, t: f64) -> Self::Output {
        use FloatNodes::*;

        match self {
            Tan { child } => f32::tan(child.compute(x, y, t).into_inner()),
            Constant { value } => *value,
        }
    }
}

pub enum AngleNodes {
    ArcSin {
        child: Box<SignedFloatNormalisedNodes>,
    },
    ArcCos {
        child: Box<SignedFloatNormalisedNodes>,
    },
    ArcTan {
        child: Box<FloatNodes>,
    },
    Random,
    Constant {
        value: Angle,
    },
    FromSignedFloatNormalised {
        child: Box<SignedFloatNormalisedNodes>,
    },
    FromUnsignedFloatNormalised {
        child: Box<UnsignedFloatNormalisedNodes>,
    },
}

impl Node for AngleNodes {
    type Output = Angle;

    fn compute(&self, x: usize, y: usize, t: f64) -> Self::Output {
        use AngleNodes::*;

        match self {
            ArcSin { child } => Angle::new(f32::asin(child.compute(x, y, t).into_inner())),
            ArcCos { child } => Angle::new(f32::acos(child.compute(x, y, t).into_inner())),
            ArcTan { child } => Angle::new(f32::atan(child.compute(x, y, t))),
            Random => Angle::random(),
            Constant { value } => *value,
            FromSignedFloatNormalised { child } => child.compute(x, y, t).to_angle(),
            FromUnsignedFloatNormalised { child } => child.compute(x, y, t).to_angle(),
        }
    }
}

pub enum SignedFloatNormalisedNodes {
    Sin {
        child: Box<AngleNodes>,
    },
    Cos {
        child: Box<AngleNodes>,
    },
    Random,
    Constant {
        value: SignedFloatNormalised,
    },
    FromAngle {
        child: Box<AngleNodes>,
    },
    FromUnsignedFloatNormalised {
        child: Box<UnsignedFloatNormalisedNodes>,
    },
}

impl Node for SignedFloatNormalisedNodes {
    type Output = SignedFloatNormalised;

    fn compute(&self, x: usize, y: usize, t: f64) -> Self::Output {
        use SignedFloatNormalisedNodes::*;

        match self {
            Sin { child } => {
                SignedFloatNormalised::new(f32::sin(child.compute(x, y, t).into_inner()))
            }
            Cos { child } => {
                SignedFloatNormalised::new(f32::cos(child.compute(x, y, t).into_inner()))
            }
            Random => SignedFloatNormalised::random(),
            FromAngle { child } => child.compute(x, y, t).to_signed(),
            FromUnsignedFloatNormalised { child } => child.compute(x, y, t).to_signed(),
            Constant { value } => *value,
        }
    }
}

pub enum UnsignedFloatNormalisedNodes {
    Random,
    Constant {
        value: UnsignedFloatNormalised,
    },
    FromAngle {
        child: Box<AngleNodes>,
    },
    FromSignedFloatNormalised {
        child: Box<SignedFloatNormalisedNodes>,
    },
}

impl Node for UnsignedFloatNormalisedNodes {
    type Output = UnsignedFloatNormalised;

    fn compute(&self, x: usize, y: usize, t: f64) -> Self::Output {
        use UnsignedFloatNormalisedNodes::*;

        match self {
            Random => UnsignedFloatNormalised::random(),
            Constant { value } => *value,
            FromAngle { child } => child.compute(x, y, t).to_unsigned(),
            FromSignedFloatNormalised { child } => child.compute(x, y, t).to_unsigned(),
        }
    }
}

// pub enum GGColorNodes {
//     DecomposeToComponents { r: f32, g: f32, b: f32 },
// }

// pub enum UnsignedIntNodes {
//     Constant,
//     Random,
//     CurrentTic,
// }

// pub enum CoordinateTranslationNodes {
//     ShiftBy,
// }

// pub enum BooleanNodes {
//     And {a: bool, b: bool},
//     Or {a: bool, b: bool},
//     Xor {a: bool, b: bool},
//     Not {a: bool},
//     Random,
// }
