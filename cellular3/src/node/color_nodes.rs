use palette::{encoding::srgb::Srgb, rgb::Rgb, Hsv, RgbHue};

use crate::{
    constants::*,
    datatype::{colors::*, image::*},
    node::{
        color_blend_nodes::*, continuous_nodes::*, coord_map_nodes::*, discrete_nodes::*,
        mutagen_functions::*, Node,
    },
    updatestate::UpdateState,
};
use mutagen::{Generatable, Mutatable};

#[derive(Generatable, Mutatable, Debug)]
#[mutagen(mut_reroll = 0.1)]
pub enum FloatColorNodes {
    #[mutagen(gen_weight = leaf_node_weight)]
    Gray,

    #[mutagen(gen_weight = leaf_node_weight)]
    FromImage { image: Image },
    #[mutagen(gen_weight = leaf_node_weight)]
    FromCellArray,

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

    #[mutagen(gen_weight = pipe_node_weight)]
    FromBlend { child: Box<ColorBlendNodes> },

    #[mutagen(gen_weight = pipe_node_weight)]
    FromBitColor { child: Box<BitColorNodes> },

    #[mutagen(gen_weight = pipe_node_weight)]
    FromByteColor { child: Box<ByteColorNodes> },

    #[mutagen(gen_weight = branch_node_weight)]
    ModifyState {
        child: Box<FloatColorNodes>,
        child_state: Box<CoordMapNodes>,
    },

    #[mutagen(gen_weight = branch_node_weight)]
    IfElse {
        predicate: Box<BooleanNodes>,
        child_a: Box<Self>,
        child_b: Box<Self>,
    },
}

impl Node for FloatColorNodes {
    type Output = FloatColor;

    fn compute(&self, state: UpdateState) -> Self::Output {
        use FloatColorNodes::*;

        match self {
            Gray => FloatColor {
                r: 1.0,
                g: 1.0,
                b: 1.0,
                a: 1.0,
            },
            FromImage { image } => image.get_pixel_normalised(
                state.coordinate_set.x,
                state.coordinate_set.y,
                state.coordinate_set.t,
            ).into(),
            FromCellArray => state.history.get(
                ((state.coordinate_set.x.into_inner() + 1.0) * 0.5 * CONSTS.cell_array_width as f32)
                    as usize,
                ((state.coordinate_set.y.into_inner() + 1.0)
                    * 0.5
                    * CONSTS.cell_array_height as f32) as usize,
                state.coordinate_set.t as usize,
            ).into(),
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
            FromBlend { child } => child.compute(state),
            FromBitColor { child } => FloatColor::from(child.compute(state)),
            ModifyState { child, child_state } => child.compute(UpdateState {
                coordinate_set: child_state.compute(state),
                ..state
            }),
            FromByteColor { child } => FloatColor::from(child.compute(state)),
            IfElse {
                predicate,
                child_a,
                child_b,
            } => {
                if predicate.compute(state).into_inner() {
                    child_a.compute(state)
                } else {
                    child_b.compute(state)
                }
            }
        }
    }
}

#[derive(Generatable, Mutatable, Debug)]
#[mutagen(mut_reroll = 0.1)]
pub enum BitColorNodes {
    #[mutagen(gen_weight = leaf_node_weight)]
    Constant { value: BitColor },

    #[mutagen(gen_weight = branch_node_weight)]
    GiveColor {
        child_a: Box<BitColorNodes>,
        child_b: Box<BitColorNodes>,
    },

    #[mutagen(gen_weight = branch_node_weight)]
    TakeColor {
        child_a: Box<BitColorNodes>,
        child_b: Box<BitColorNodes>,
    },

    #[mutagen(gen_weight = branch_node_weight)]
    XorColor {
        child_a: Box<BitColorNodes>,
        child_b: Box<BitColorNodes>,
    },

    #[mutagen(gen_weight = branch_node_weight)]
    EqColor {
        child_a: Box<BitColorNodes>,
        child_b: Box<BitColorNodes>,
    },

    #[mutagen(gen_weight = branch_node_weight)]
    FromComponents {
        r: Box<BooleanNodes>,
        g: Box<BooleanNodes>,
        b: Box<BooleanNodes>,
    },

    #[mutagen(gen_weight = leaf_node_weight)]
    FromImage { image: Image },
    #[mutagen(gen_weight = leaf_node_weight)]
    FromCellArray,

    #[mutagen(gen_weight = pipe_node_weight)]
    FromUNFloat { child: Box<UNFloatNodes> },

    #[mutagen(gen_weight = pipe_node_weight)]
    FromFloatColor { child: Box<FloatColorNodes> },
    #[mutagen(gen_weight = pipe_node_weight)]
    FromByteColor { child: Box<ByteColorNodes> },

    #[mutagen(gen_weight = branch_node_weight)]
    ModifyState {
        child: Box<BitColorNodes>,
        child_state: Box<CoordMapNodes>,
    },
    #[mutagen(gen_weight = branch_node_weight)]
    IfElse {
        predicate: Box<BooleanNodes>,
        child_a: Box<Self>,
        child_b: Box<Self>,
    },
}

impl Node for BitColorNodes {
    type Output = BitColor;

    fn compute(&self, state: UpdateState) -> Self::Output {
        use BitColorNodes::*;
        match self {
            Constant { value } => *value,
            GiveColor { child_a, child_b } => BitColor::from_components(
                child_a.compute(state).give_color(child_b.compute(state)),
            ),
            TakeColor { child_a, child_b } => BitColor::from_components(
                child_a.compute(state).take_color(child_b.compute(state)),
            ),
            XorColor { child_a, child_b } => BitColor::from_components(
                child_a.compute(state).xor_color(child_b.compute(state)),
            ),
            EqColor { child_a, child_b } => BitColor::from_components(
                child_a.compute(state).eq_color(child_b.compute(state)),
            ),
            FromComponents { r, g, b } => BitColor::from_components([
                r.compute(state).into_inner(),
                g.compute(state).into_inner(),
                b.compute(state).into_inner(),
            ]),
            FromImage { image } => image.get_pixel_normalised(
                state.coordinate_set.x,
                state.coordinate_set.y,
                state.coordinate_set.t,
            ).into(),
            FromCellArray => state.history.get(
                ((state.coordinate_set.x.into_inner() + 1.0) * 0.5 * CONSTS.cell_array_width as f32)
                    as usize,
                ((state.coordinate_set.y.into_inner() + 1.0)
                    * 0.5
                    * CONSTS.cell_array_height as f32) as usize,
                state.coordinate_set.t as usize,
            ).into(),
            FromUNFloat { child } => BitColor::from_index(
                (child.compute(state).into_inner() * 0.99 * (CONSTS.max_colors) as f32) as usize,
            ),
            FromFloatColor { child } => BitColor::from_float_color(child.compute(state)),
            FromByteColor { child } => BitColor::from_byte_color(child.compute(state)),
            ModifyState { child, child_state } => child.compute(UpdateState {
                coordinate_set: child_state.compute(state),
                ..state
            }),
            IfElse {
                predicate,
                child_a,
                child_b,
            } => {
                if predicate.compute(state).into_inner() {
                    child_a.compute(state)
                } else {
                    child_b.compute(state)
                }
            }
        }
    }
}

#[derive(Generatable, Mutatable, Debug)]
#[mutagen(mut_reroll = 0.1)]
pub enum ByteColorNodes {
    #[mutagen(gen_weight = leaf_node_weight)]
    Constant { value: ByteColor },

    #[mutagen(gen_weight = leaf_node_weight)]
    FromImage { image: Image },
    #[mutagen(gen_weight = leaf_node_weight)]
    FromCellArray,

    #[mutagen(gen_weight = pipe_node_weight)]
    FromFloatColor { child: Box<FloatColorNodes> },
    #[mutagen(gen_weight = pipe_node_weight)]
    FromBitColor { child: Box<FloatColorNodes> },

    #[mutagen(gen_weight = branch_node_weight)]
    Decompose {
        r: Box<ByteNodes>,
        g: Box<ByteNodes>,
        b: Box<ByteNodes>,
    },

    #[mutagen(gen_weight = branch_node_weight)]
    ModifyState {
        child: Box<ByteColorNodes>,
        child_state: Box<CoordMapNodes>,
    },

    #[mutagen(gen_weight = branch_node_weight)]
    IfElse {
        predicate: Box<BooleanNodes>,
        child_a: Box<Self>,
        child_b: Box<Self>,
    },
}

impl Node for ByteColorNodes {
    type Output = ByteColor;

    fn compute(&self, state: UpdateState) -> Self::Output {
        use ByteColorNodes::*;

        match self {
            Constant { value } => *value,
            FromImage { image } => image.get_pixel_normalised(
                state.coordinate_set.x,
                state.coordinate_set.y,
                state.coordinate_set.t,
            ),
            FromCellArray => state.history.get(
                ((state.coordinate_set.x.into_inner() + 1.0) * 0.5 * CONSTS.cell_array_width as f32)
                    as usize,
                ((state.coordinate_set.y.into_inner() + 1.0)
                    * 0.5
                    * CONSTS.cell_array_height as f32) as usize,
                state.coordinate_set.t as usize,
            ),
            Decompose { r, g, b } => ByteColor {
                r: r.compute(state).into_inner(),
                g: g.compute(state).into_inner(),
                b: b.compute(state).into_inner(),
            },
            FromFloatColor { child } => child.compute(state).into(),
            FromBitColor { child } => child.compute(state).into(),
            ModifyState { child, child_state } => child.compute(UpdateState {
                coordinate_set: child_state.compute(state),
                ..state
            }),
            IfElse {
                predicate,
                child_a,
                child_b,
            } => {
                if predicate.compute(state).into_inner() {
                    child_a.compute(state)
                } else {
                    child_b.compute(state)
                }
            }
        }
    }
}
