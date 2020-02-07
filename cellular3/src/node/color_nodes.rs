use palette::{encoding::srgb::Srgb, rgb::Rgb, Hsv, RgbHue};

use crate::{
    constants::MAX_COLORS,
    datatype::{colors::*, image::*},
    node::{primitive_nodes::*, Node, state_nodes::*},
    updatestate::UpdateState,
};
use ndarray::prelude::*;
use mutagen::{Generatable, Mutatable};

#[derive(Generatable, Mutatable, Debug)]
#[mutagen(mut_reroll = 0.1)]
#[allow(dead_code)]
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
    FromPalletteColor {
        child: Box<PalletteColorNodes>,
    },
    FromIntColor {
        child: Box<IntColorNodes>,
    },
    // ModifyState {
    //     child: Box<FloatColorNodes>,
    //     child_state: Box<StateNodes>,
    // }
}

// This function assumes an x and y between the ranges -dim().<dimension>..infinity
fn wrap_point_to_cell_array(
    cell_array: ArrayView2<'_, FloatColor>,
    x: usize,
    y: usize,
) -> (usize, usize) {
    let width = cell_array.dim().0 as usize;
    let height = cell_array.dim().1 as usize;

    ((x % width + width) % width, (y % height + height) % height)
}

impl Node for FloatColorNodes {
    type Output = FloatColor;

    fn compute(&self, state: UpdateState) -> Self::Output {
        use FloatColorNodes::*;

        match self {
            Grayscale { child } => {
                let value = child.compute(state).into_inner() as f32;
                FloatColor {
                    r: value,
                    g: value,
                    b: value,
                    a: 1.0,
                }
            }
            RGB { r, g, b } => FloatColor {
                r: r.compute(state).into_inner() as f32,
                g: g.compute(state).into_inner() as f32,
                b: b.compute(state).into_inner() as f32,
                a: 1.0,
            },
            HSV { h, s, v } => {
                let rgb: Rgb = Hsv::<Srgb, _>::from_components((
                    RgbHue::from_degrees(h.compute(state).into_inner() as f32 * 360.0),
                    s.compute(state).into_inner() as f32,
                    v.compute(state).into_inner() as f32,
                ))
                .into();

                float_color_from_pallette_rgb(rgb)
            }
            FromCellArray => {
                let (x, y) = 
                    wrap_point_to_cell_array(
                        state.cell_array.view(), 
                        state.coordinate_set.x as usize, 
                        state.coordinate_set.y as usize);
                state.cell_array[[x as usize, y as usize]]
            },
            FromPalletteColor { child } => FloatColor::from(child.compute(state)),
            // ModifyState { child, child_state } => child.compute(child_state.compute(state).into_inner()),
            FromIntColor { child } => FloatColor::from(child.compute(state)),
        }
    }
}

#[derive(Generatable, Mutatable, Debug)]
#[mutagen(mut_reroll = 0.1)]
#[allow(dead_code)]
pub enum PalletteColorNodes {
    Constant {
        value: PalletteColor,
    },
    FromUNFloat {
        child: Box<UNFloatNodes>,
    },
    #[mutagen(gen_weight = 0.1)]
    GiveColor {
        child_a: Box<PalletteColorNodes>,
        child_b: Box<PalletteColorNodes>,
    },
    #[mutagen(gen_weight = 0.1)]
    TakeColor {
        child_a: Box<PalletteColorNodes>,
        child_b: Box<PalletteColorNodes>,
    },
    #[mutagen(gen_weight = 0.1)]
    XorColor {
        child_a: Box<PalletteColorNodes>,
        child_b: Box<PalletteColorNodes>,
    },
    #[mutagen(gen_weight = 0.1)]
    EqColor {
        child_a: Box<PalletteColorNodes>,
        child_b: Box<PalletteColorNodes>,
    },
    FromComponents {
        r: Box<BooleanNodes>,
        g: Box<BooleanNodes>,
        b: Box<BooleanNodes>,
    },
    FromFloatColor {
        child: Box<FloatColorNodes>,
    },
}

impl Node for PalletteColorNodes {
    type Output = PalletteColor;

    fn compute(&self, state: UpdateState) -> Self::Output {
        use PalletteColorNodes::*;
        match self {
            Constant { value } => *value,
            FromUNFloat { child } => PalletteColor::from_index(
                (child.compute(state).into_inner() * 0.99 * (MAX_COLORS) as f32) as usize,
            ),
            GiveColor { child_a, child_b } => PalletteColor::from_components(
                child_a.compute(state).give_color(child_b.compute(state)),
            ),
            TakeColor { child_a, child_b } => PalletteColor::from_components(
                child_a.compute(state).take_color(child_b.compute(state)),
            ),
            XorColor { child_a, child_b } => PalletteColor::from_components(
                child_a.compute(state).xor_color(child_b.compute(state)),
            ),
            EqColor { child_a, child_b } => PalletteColor::from_components(
                child_a.compute(state).eq_color(child_b.compute(state)),
            ),
            FromComponents { r, g, b } => PalletteColor::from_components([
                r.compute(state).into_inner(),
                g.compute(state).into_inner(),
                b.compute(state).into_inner(),
            ]),
            FromFloatColor { child } => {
                PalletteColor::from_float_color(child.compute(state))
            }
        }
    }
}

#[derive(Generatable, Mutatable, Debug)]
#[mutagen(mut_reroll = 0.1)]
#[allow(dead_code)]
pub enum IntColorNodes {
    FromImage { image: Image },
}

impl Node for IntColorNodes {
    type Output = IntColor;

    fn compute(&self, state: UpdateState) -> Self::Output {
        use IntColorNodes::*;

        match self{
            FromImage { image } => image.get_pixel(state.coordinate_set.x as u32, state.coordinate_set.y as u32, state.coordinate_set.t as u32),
        }
    }
}
