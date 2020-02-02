use crate::colors::*;
use crate::constants::*;
use ndarray::prelude::*;
use noise::{NoiseFn, OpenSimplex, Worley, RangeFunction};

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
    SimplexNoise {noise: OpenSimplex},
    Worley {noise: Worley},
    ComboNoise {n1: OpenSimplex, n2: Worley},
    // DecomposeToComponents{
    //     //r: 
    // }
}

impl Node for PalletteColorNodes {
    type Output = PalletteColor;

    fn compute(&self, x: usize, y: usize, t: f64) -> PalletteColor {
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
                let noise_value = (noise.get([x as f64 * 0.025, y as f64 * 0.025, t as f64 * 0.1])) + 0.5;
                PalletteColor::from_index((noise_value * (MAX_COLORS - 1) as f64) as usize)}
            PalletteColorNodes::Worley { noise } => { 
                let noise_value = (noise.get([x as f64 * 0.025, y as f64 * 0.025, t as f64 * 0.1]).abs());
                PalletteColor::from_index((noise_value * (MAX_COLORS - 1) as f64) as usize)}
            PalletteColorNodes::ComboNoise { n1, n2 } => { 
                let noise_value1 = n1.get([x as f64 * 0.025, y as f64 * 0.025, t as f64 * 0.1]) + 0.5;
                let noise_value2 = n2.get([x as f64 * 0.025, y as f64 * 0.025, t as f64 * 0.1]).abs();

                PalletteColor::from_index((noise_value1 * noise_value2 * (MAX_COLORS - 1) as f64) as usize)
            }
        }
    }
}

// pub enum RangeFunctionNodes {
//     RangeFunction { function: RangeFunction },    
// }

// pub enum NeighbourhoodNodes {
    
// }

// pub enum SignedNormalisedFloatNodes {
//     Sin,
//     Tan,
//     Cos,
//     Arctan,
//     Cosec,
//     Cot,
//     Random,
//     Constant,
// }

// pub enum UnsignedNormalisedFloatNodes {
//     Constant,
//     Random,
// }

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
