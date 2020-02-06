use palette::{encoding::srgb::Srgb, rgb::Rgb, Hsv, RgbHue};

use crate::{
    constants::MAX_COLORS,
    datatype::colors::*,
    node::{primitive_nodes::*, Node},
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
    // CoordinateTranslateX {
    //     child: Box<FloatColorNodes>,
    //     x: Box<SNFloatNodes>,
    // },
    // CoordinateTranslateY {
    //     child: Box<FloatColorNodes>,
    //     y: Box<SNFloatNodes>,
    // },
    // CoordinateTranslateT {
    //     child: Box<FloatColorNodes>,
    //     t: Box<SNFloatNodes>,
    // },
    // CoordinateScaleX {
    //     child: Box<FloatColorNodes>,
    //     x: Box<UNFloatNodes>,
    // },
    // CoordinateScaleY {
    //     child: Box<FloatColorNodes>,
    //     y: Box<UNFloatNodes>,
    // },
    // CoordinateScaleT {
    //     child: Box<FloatColorNodes>,
    //     t: Box<UNFloatNodes>,
    // },
}

// This function assumes an x and y between the ranges -dim().<dimension>..infinity
fn wrap_point_to_cell_array(
    cell_array: ArrayView2<'_, FloatColor>,
    x: usize,
    y: usize,
) -> (usize, usize) {
    let width = cell_array.dim().0 as usize;
    let height = cell_array.dim().1 as usize;

    ((x + width) % width, (y + height) % height)
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
                let (x, y) = wrap_point_to_cell_array(state.cell_array.view(), state.x as usize, state.y as usize);
                state.cell_array[[x as usize, y as usize]]
            },
            FromPalletteColor { child } => FloatColor::from(child.compute(state)),
            // CoordinateTranslateX { child, x } => child.compute(UpdateState {
            //     x: state.x + x.compute(state).into_inner(),
            //     y: state.y,
            //     t: state.t,
            //     cell_array: state.cell_array,
            // }),
            // CoordinateTranslateY { child, y } => child.compute(UpdateState {
            //     x: state.x,
            //     y: state.y + y.compute(state).into_inner(),
            //     t: state.t,
            //     cell_array: state.cell_array,
            // }),
            // CoordinateTranslateT { child, t } => child.compute(UpdateState {
            //     x: state.x,
            //     y: state.y,
            //     t: state.t + t.compute(state).into_inner(),
            //     cell_array: state.cell_array,
            // }),
            // CoordinateScaleX { child, x } => child.compute(UpdateState {
            //     x: (state.x * x.compute(state).into_inner()),
            //     y: state.y,
            //     t: state.t,
            //     cell_array: state.cell_array,
            // }),
            // CoordinateScaleY { child, y } => child.compute(UpdateState {
            //     x: state.x,
            //     y: (state.y * y.compute(state).into_inner()),
            //     t: state.t,
            //     cell_array: state.cell_array,
            // }),
            // CoordinateScaleT { child, t } => child.compute(UpdateState {
            //     x: state.x,
            //     y: state.y,
            //     t: (state.t * t.compute(state).into_inner()),
            //     cell_array: state.cell_array,
            // }),
        }
    }
}

#[derive(Generatable, Mutatable, Debug)]
#[mutagen(mut_reroll = 0.1)]
#[allow(dead_code)]
pub enum PalletteColorNodes {
    //Red,
    // Modulus {
    //     x_mod: usize,
    //     y_mod: usize,
    //     x_offset: usize,
    //     y_offset: usize,
    //     color_table: Array2<PalletteColor>,
    // },
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
    // CoordinateTranslateX {
    //     child: Box<PalletteColorNodes>,
    //     x: Box<SNFloatNodes>,
    // },
    // CoordinateTranslateY {
    //     child: Box<PalletteColorNodes>,
    //     y: Box<SNFloatNodes>,
    // },
    // CoordinateTranslateT {
    //     child: Box<PalletteColorNodes>,
    //     t: Box<SNFloatNodes>,
    // },
    // CoordinateScaleX {
    //     child: Box<PalletteColorNodes>,
    //     x: Box<UNFloatNodes>,
    // },
    // CoordinateScaleY {
    //     child: Box<PalletteColorNodes>,
    //     y: Box<UNFloatNodes>,
    // },
    // CoordinateScaleT {
    //     child: Box<PalletteColorNodes>,
    //     t: Box<UNFloatNodes>,
    // },
}

impl Node for PalletteColorNodes {
    type Output = PalletteColor;

    fn compute(&self, state: UpdateState) -> Self::Output {
        use PalletteColorNodes::*;
        match self {
            //ColorNodes::Red => PalletteColor::Red,
            // Modulus {
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
            // CoordinateTranslateX { child, x } => child.compute(UpdateState {
            //     x: state.x + x.compute(state).into_inner(),
            //     y: state.y,
            //     t: state.t,
            //     cell_array: state.cell_array,
            // }),
            // CoordinateTranslateY { child, y } => child.compute(UpdateState {
            //     x: state.x,
            //     y: state.y + y.compute(state).into_inner(),
            //     t: state.t,
            //     cell_array: state.cell_array,
            // }),
            // CoordinateTranslateT { child, t } => child.compute(UpdateState {
            //     x: state.x,
            //     y: state.y,
            //     t: state.t + t.compute(state).into_inner(),
            //     cell_array: state.cell_array,
            // }),
            // CoordinateScaleX { child, x } => child.compute(UpdateState {
            //     x: (state.x * x.compute(state).into_inner()),
            //     y: state.y,
            //     t: state.t,
            //     cell_array: state.cell_array,
            // }),
            // CoordinateScaleY { child, y } => child.compute(UpdateState {
            //     x: state.x,
            //     y: (state.y * y.compute(state).into_inner()),
            //     t: state.t,
            //     cell_array: state.cell_array,
            // }),
            // CoordinateScaleT { child, t } => child.compute(UpdateState {
            //     x: state.x,
            //     y: state.y,
            //     t: (state.t * t.compute(state).into_inner()),
            //     cell_array: state.cell_array,
            // }),
        }
    }
}
