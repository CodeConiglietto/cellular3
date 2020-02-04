use crate::{
    colors::*,
    constants::*,
    updatestate::*,
    noisedatatypes::*,
    datatypes::{Angle, SNFloat, UNFloat},
};
use mutagen::Generatable;
use ndarray::prelude::*;
use noise::{
    BasicMulti, Billow, Checkerboard, Fbm, HybridMulti, NoiseFn, OpenSimplex, RangeFunction,
    RidgedMulti, SuperSimplex, Value, Worley,
};
use palette::{encoding::srgb::Srgb, rgb::Rgb, Hsv, RgbHue};
//use rand::prelude::*;

pub trait Node {
    type Output;

    fn compute(&self, state: UpdateState) -> Self::Output;
}

#[derive(Generatable)]
pub enum FloatColorNodes {
    Grayscale {
        child: Box<UNFloatNodes>,
    },
    RGB {
        r: Box<UNFloatNodes>,
        g: Box<UNFloatNodes>,
        b: Box<UNFloatNodes>,
    },
    HSV {
        h: Box<UNFloatNodes>,
        s: Box<UNFloatNodes>,
        v: Box<UNFloatNodes>,
    },
    FromCellArray,
}

impl Node for FloatColorNodes {
    type Output = FloatColor;

    fn compute(&self, state: UpdateState) -> Self::Output {
        match self {
            FloatColorNodes::Grayscale { child } => {
                let value = child.compute(state).into_inner() as f32;
                FloatColor {
                    r: value,
                    g: value,
                    b: value,
                    a: 1.0,
                }
            }
            FloatColorNodes::RGB { r, g, b } => FloatColor {
                r: r.compute(state).into_inner() as f32,
                g: g.compute(state).into_inner() as f32,
                b: b.compute(state).into_inner() as f32,
                a: 1.0,
            },
            FloatColorNodes::HSV { h, s, v } => {
                let rgb: Rgb = Hsv::<Srgb, _>::from_components((
                    RgbHue::from_degrees(h.compute(state).into_inner() as f32 * 360.0),
                    s.compute(state).into_inner() as f32,
                    v.compute(state).into_inner() as f32,
                ))
                .into();

                float_color_from_pallette_rgb(rgb)
            }
            FloatColorNodes::FromCellArray => state.cell_array[[state.x, state.y]],
        }
    }
}

#[derive(Generatable)]
pub enum PalletteColorNodes {
    //Red,
    // Modulus {
    //     x_mod: usize,
    //     y_mod: usize,
    //     x_offset: usize,
    //     y_offset: usize,
    //     color_table: Array2<PalletteColor>,
    // },
    FromUNFloat {
        child: UNFloatNodes,
    },
    GiveColor {
        child_a: Box<PalletteColorNodes>,
        child_b: Box<PalletteColorNodes>,
    },
    TakeColor {
        child_a: Box<PalletteColorNodes>,
        child_b: Box<PalletteColorNodes>,
    },
    XorColor {
        child_a: Box<PalletteColorNodes>,
        child_b: Box<PalletteColorNodes>,
    },
    EqColor {
        child_a: Box<PalletteColorNodes>,
        child_b: Box<PalletteColorNodes>,
    }, // DecomposeToComponents{
       //     //r:
       // }
}

impl Node for PalletteColorNodes {
    type Output = PalletteColor;

    fn compute(&self, state: UpdateState) -> Self::Output {
        match self {
            //ColorNodes::Red => PalletteColor::Red,
            // PalletteColorNodes::Modulus {
            //     x_mod,
            //     y_mod,
            //     x_offset,
            //     y_offset,
            //     color_table,
            // } => {
            //     let x_index = if (state.x + x_offset) % x_mod == 0 { 1 } else { 0 };
            //     let y_index = if (state.y + y_offset) % y_mod == 0 { 1 } else { 0 };

            //     color_table[[x_index, y_index]]
            // }
            PalletteColorNodes::FromUNFloat { child } => PalletteColor::from_index(
                (child.compute(state).into_inner() * (MAX_COLORS) as f32) as usize,
            ),
            PalletteColorNodes::GiveColor { child_a, child_b } => PalletteColor::from_components(
                child_a
                    .compute(state)
                    .give_color(child_b.compute(state)),
            ),
            PalletteColorNodes::TakeColor { child_a, child_b } => PalletteColor::from_components(
                child_a
                    .compute(state)
                    .take_color(child_b.compute(state)),
            ),
            PalletteColorNodes::XorColor { child_a, child_b } => PalletteColor::from_components(
                child_a.compute(state).xor_color(child_b.compute(state)),
            ),
            PalletteColorNodes::EqColor { child_a, child_b } => PalletteColor::from_components(
                child_a.compute(state).eq_color(child_b.compute(state)),
            ), // PalletteColorNodes::SimplexNoise { noise } => {
               //     let noise_value =
               //         (noise.get([x as f64 * 0.025, y as f64 * 0.025, t as f64 * 0.1])) + 0.5;
               //     PalletteColor::from_index((noise_value * (MAX_COLORS - 1) as f64) as usize)
               // }
               // PalletteColorNodes::Worley { noise } => {
               //     let noise_value = noise
               //         .get([x as f64 * 0.025, y as f64 * 0.025, t as f64 * 0.1])
               //         .abs();
               //     PalletteColor::from_index((noise_value * (MAX_COLORS - 1) as f64) as usize)
               // }
               // PalletteColorNodes::ComboNoise { n1, n2 } => {
               //     let noise_value1 =
               //         n1.get([x as f64 * 0.025, y as f64 * 0.025, t as f64 * 0.1]) + 0.5;
               //     let noise_value2 = n2
               //         .get([x as f64 * 0.025, y as f64 * 0.025, t as f64 * 0.1])
               //         .abs();

               //     let c1 = PalletteColor::from_index((noise_value1 * (MAX_COLORS - 1) as f64) as usize);
               //     let c2 = PalletteColor::from_index((noise_value2 * (MAX_COLORS - 1) as f64) as usize);

               //     PalletteColor::from_composites(c1.take_color(c2))
               // }
        }
    }
}

// pub enum RangeFunctionNodes {
//     RangeFunction { function: RangeFunction },
// }

// pub enum NeighbourhoodNodes {

// }

// #[derive(Generatable)]
// pub enum FloatNodes {
//     Tan { child: Box<AngleNodes> },
//     Constant { value: f32 },
// }

// impl Node for FloatNodes {
//     type Output = f32;

//     fn compute(&self, state: UpdateState) -> Self::Output {
//         use FloatNodes::*;

//         match self {
//             Tan { child } => f32::tan(child.compute(state).into_inner()),
//             Constant { value } => *value,
//         }
//     }
// }

#[derive(Generatable)]
pub enum AngleNodes {
    ArcSin { theta: Box<SNFloatNodes> },
    ArcCos { theta: Box<SNFloatNodes> },
    //ArcTan { theta: Box<FloatNodes> },
    Random,
    Constant { value: Angle },
    FromSNFloat { child: Box<SNFloatNodes> },
    FromUNFloat { child: Box<UNFloatNodes> },
}

impl Node for AngleNodes {
    type Output = Angle;

    fn compute(&self, state: UpdateState) -> Self::Output {
        use AngleNodes::*;

        match self {
            ArcSin { theta } => Angle::new(f32::asin(theta.compute(state).into_inner())),
            ArcCos { theta } => Angle::new(f32::acos(theta.compute(state).into_inner())),
            //ArcTan { theta } => Angle::new(f32::atan(theta.compute(state))),
            Random => Angle::random(),
            Constant { value } => *value,
            FromSNFloat { child } => child.compute(state).to_angle(),
            FromUNFloat { child } => child.compute(state).to_angle(),
        }
    }
}

#[derive(Generatable)]
pub enum SNFloatNodes {
    Sin {
        child: Box<AngleNodes>,
    },
    Cos {
        child: Box<AngleNodes>,
    },
    Random,
    Constant {
        value: SNFloat,
    },
    FromAngle {
        child: Box<AngleNodes>,
    },
    FromUNFloat {
        child: Box<UNFloatNodes>,
    },
    FbmNoise {
        noise: FractalBrownianNoise,
    },
    OpenSimplexNoise {
        noise: OpenSimplexNoise,
    },
    WorleyNoise {
        noise: WorleyNoise,
    },
    // ValueNoise {
    //     noise: ValueNoise,
    // },
    RidgedMultiNoise {
        noise: RidgedMultiFractalNoise,
    },
    Multiply {
        child_a: Box<SNFloatNodes>,
        child_b: Box<SNFloatNodes>,
    },
    Abs {
        child: Box<SNFloatNodes>,
    },
}

impl Node for SNFloatNodes {
    type Output = SNFloat;

    fn compute(&self, state: UpdateState) -> Self::Output {
        use SNFloatNodes::*;

        match self {
            Sin { child } => SNFloat::new(f32::sin(child.compute(state).into_inner())),
            Cos { child } => SNFloat::new(f32::cos(child.compute(state).into_inner())),
            Random => SNFloat::random(),
            FromAngle { child } => child.compute(state).to_signed(),
            FromUNFloat { child } => child.compute(state).to_signed(),
            Constant { value } => *value,
            OpenSimplexNoise { noise } => {
                SNFloat::new(noise.0.get([state.x as f64 * 0.025, state.y as f64 * 0.025, state.t as f64 * 0.05]) as f32)
            }
            FbmNoise { noise } => {
                SNFloat::new(noise.0.get([state.x as f64 * 0.01, state.y as f64 * 0.01, state.t as f64 * 0.05]) as f32)
            }
            WorleyNoise { noise } => SNFloat::new(
                -1.0 * noise.0
                    .get([state.x as f64 * 0.025, state.y as f64 * 0.025, state.t as f64 * 0.05])
                    .min(0.99) as f32,
            ),
            //Todo scale
            //ValueNoise { noise } => SNFloat::new(noise.0.get([state.x as f64, state.y as f64 as f64, state.t as f64]) as f32),
            RidgedMultiNoise { noise } => SNFloat::new(
                noise
                    .0.get([state.x as f64 * 0.01, state.y as f64 * 0.01, state.t as f64 * 0.05])
                    .min(0.99) as f32,
            ),
            Multiply { child_a, child_b } => SNFloat::new(
                child_a.compute(state).into_inner() * child_b.compute(state).into_inner(),
            ),
            Abs { child } => SNFloat::new(child.compute(state).into_inner().abs()),
        }
    }
}

#[derive(Generatable)]
pub enum UNFloatNodes {
    Random,
    Constant {
        value: UNFloat,
    },
    FromAngle {
        child: Box<AngleNodes>,
    },
    FromSNFloat {
        child: Box<SNFloatNodes>,
    },
    AbsSNFloat {
        child: Box<SNFloatNodes>,
    },
    SquareSNFloat {
        child: Box<SNFloatNodes>,
    },
    Multiply {
        child_a: Box<UNFloatNodes>,
        child_b: Box<UNFloatNodes>,
    },
    InvertNormalised {
        child: Box<UNFloatNodes>,
    },
    ColorAverage {
        child: Box<FloatColorNodes>,
    },
    ColorComponent {
        child: Box<FloatColorNodes>,
    }
}

impl Node for UNFloatNodes {
    type Output = UNFloat;

    fn compute(&self, state: UpdateState) -> Self::Output {
        use UNFloatNodes::*;

        match self {
            Random => UNFloat::random(),
            Constant { value } => *value,
            FromAngle { child } => child.compute(state).to_unsigned(),
            FromSNFloat { child } => child.compute(state).to_unsigned(),
            AbsSNFloat { child } => UNFloat::new(child.compute(state).into_inner().abs()),
            SquareSNFloat { child } => UNFloat::new(child.compute(state).into_inner().powf(2.0)),
            Multiply { child_a, child_b } => UNFloat::new(
                child_a.compute(state).into_inner() * child_b.compute(state).into_inner(),
            ),
            InvertNormalised { child } => UNFloat::new(1.0 - child.compute(state).into_inner()),
            ColorAverage { child } => { let color = child.compute(state); UNFloat::new((color.r + color.g + color.b) / 3.0) },
            ColorComponent { child } => { UNFloat::new(child.compute(state).r) },
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
