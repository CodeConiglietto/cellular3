use palette::{encoding::srgb::Srgb, rgb::Rgb, Hsv, RgbHue};

use crate::{
    constants::*,
    datatype::{colors::*, image::*},
    node::{coord_map_nodes::*, mutagen_functions::*, primitive_nodes::*, Node},
    updatestate::UpdateState,
};
use mutagen::{Generatable, Mutatable};
use ndarray::prelude::*;

#[derive(Generatable, Mutatable, Debug)]
#[mutagen(mut_reroll = 0.1)]
pub enum FloatColorNodes {
    #[mutagen(gen_weight = pipe_node_weight)]
    Grayscale { child: Box<UNFloatNodes> },

    #[mutagen(gen_weight = branch_node_weight)]
    RGB {
        r: Box<UNFloatNodes>,
        g: Box<UNFloatNodes>,
        b: Box<UNFloatNodes>,
    },

    #[mutagen(gen_weight = branch_node_weight)]
    HSV {
        h: Box<UNFloatNodes>,
        s: Box<UNFloatNodes>,
        v: Box<UNFloatNodes>,
    },

    #[mutagen(gen_weight = leaf_node_weight)]
    FromCellArray,

    #[mutagen(gen_weight = pipe_node_weight)]
    FromPalletteColor { child: Box<PalletteColorNodes> },

    #[mutagen(gen_weight = pipe_node_weight)]
    FromIntColor { child: Box<IntColorNodes> },

    #[mutagen(gen_weight = branch_node_weight)]
    ModifyState {
        child: Box<FloatColorNodes>,
        child_state: Box<CoordMapNodes>,
    },
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
                let (x, y) = wrap_point_to_cell_array(
                    state.cell_array.view(),
                    ((state.coordinate_set.x.into_inner() + 1.0) * 0.5 * CELL_ARRAY_WIDTH as f32)
                        as usize,
                    ((state.coordinate_set.y.into_inner() + 1.0) * 0.5 * CELL_ARRAY_HEIGHT as f32)
                        as usize,
                );
                state.cell_array[[x as usize, y as usize]]
            }
            FromPalletteColor { child } => FloatColor::from(child.compute(state)),
            ModifyState { child, child_state } => child.compute(UpdateState {
                coordinate_set: child_state.compute(state),
                cell_array: state.cell_array,
            }),
            FromIntColor { child } => FloatColor::from(child.compute(state)),
        }
    }
}

#[derive(Generatable, Mutatable, Debug)]
#[mutagen(mut_reroll = 0.1)]
pub enum PalletteColorNodes {
    #[mutagen(gen_weight = leaf_node_weight)]
    Constant { value: PalletteColor },

    #[mutagen(gen_weight = branch_node_weight)]
    GiveColor {
        child_a: Box<PalletteColorNodes>,
        child_b: Box<PalletteColorNodes>,
    },

    #[mutagen(gen_weight = branch_node_weight)]
    TakeColor {
        child_a: Box<PalletteColorNodes>,
        child_b: Box<PalletteColorNodes>,
    },

    #[mutagen(gen_weight = branch_node_weight)]
    XorColor {
        child_a: Box<PalletteColorNodes>,
        child_b: Box<PalletteColorNodes>,
    },

    #[mutagen(gen_weight = branch_node_weight)]
    EqColor {
        child_a: Box<PalletteColorNodes>,
        child_b: Box<PalletteColorNodes>,
    },

    #[mutagen(gen_weight = branch_node_weight)]
    FromComponents {
        r: Box<BooleanNodes>,
        g: Box<BooleanNodes>,
        b: Box<BooleanNodes>,
    },

    #[mutagen(gen_weight = pipe_node_weight)]
    FromUNFloat { child: Box<UNFloatNodes> },

    #[mutagen(gen_weight = pipe_node_weight)]
    FromFloatColor { child: Box<FloatColorNodes> },

    #[mutagen(gen_weight = branch_node_weight)]
    ModifyState {
        child: Box<PalletteColorNodes>,
        child_state: Box<CoordMapNodes>,
    },
}

impl Node for PalletteColorNodes {
    type Output = PalletteColor;

    fn compute(&self, state: UpdateState) -> Self::Output {
        use PalletteColorNodes::*;
        match self {
            Constant { value } => *value,
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
            FromUNFloat { child } => PalletteColor::from_index(
                (child.compute(state).into_inner() * 0.99 * (MAX_COLORS) as f32) as usize,
            ),
            FromFloatColor { child } => PalletteColor::from_float_color(child.compute(state)),
            ModifyState { child, child_state } => child.compute(UpdateState {
                coordinate_set: child_state.compute(state),
                cell_array: state.cell_array,
            }),
        }
    }
}

#[derive(Generatable, Mutatable, Debug)]
#[mutagen(mut_reroll = 0.1)]
pub enum IntColorNodes {
    #[mutagen(gen_weight = leaf_node_weight)]
    Constant { value: IntColor },

    #[mutagen(gen_weight = leaf_node_weight)]
    FromImage { image: Image },

    #[mutagen(gen_weight = branch_node_weight)]
    ModifyState {
        child: Box<IntColorNodes>,
        child_state: Box<CoordMapNodes>,
    },
}

impl Node for IntColorNodes {
    type Output = IntColor;

    fn compute(&self, state: UpdateState) -> Self::Output {
        use IntColorNodes::*;

        match self {
            Constant { value } => *value,
            FromImage { image } => image.get_pixel_normalised(
                state.coordinate_set.x,
                state.coordinate_set.y,
                state.coordinate_set.t,
            ),
            ModifyState { child, child_state } => child.compute(UpdateState {
                coordinate_set: child_state.compute(state),
                cell_array: state.cell_array,
            }),
        }
    }
}
