use crate::{
    datatype::{colors::*, continuous::*},
    node::{color_nodes::*, continuous_nodes::*, mutagen_functions::*, Node},
    updatestate::UpdateState,
};
use mutagen::{Generatable, Mutatable};

#[derive(Generatable, Mutatable, Debug)]
#[mutagen(mut_reroll = 0.1)]
pub enum ColorBlendNodes {
    #[mutagen(gen_weight = leaf_node_weight)]
    Gray,

    #[mutagen(gen_weight = pipe_node_weight)]
    Invert { child: Box<FloatColorNodes> },

    #[mutagen(gen_weight = branch_node_weight)]
    Dissolve {
        color_a: Box<FloatColorNodes>,
        color_b: Box<FloatColorNodes>,
        value: Box<UNFloatNodes>,
    },
    // #[mutagen(gen_weight = branch_node_weight)]
    // Overlay {
    //     color_a: Box<FloatColorNodes>,
    //     color_b: Box<FloatColorNodes>,
    //     value: Box<UNFloatNodes>,
    // },

    // #[mutagen(gen_weight = branch_node_weight)]
    // ScreenDodge {
    //     color_a: Box<FloatColorNodes>,
    //     color_b: Box<FloatColorNodes>,
    //     value: Box<UNFloatNodes>,
    // },

    // #[mutagen(gen_weight = branch_node_weight)]
    // ColorDodge {
    //     color_a: Box<FloatColorNodes>,
    //     color_b: Box<FloatColorNodes>,
    //     value: Box<UNFloatNodes>,
    // },

    // #[mutagen(gen_weight = branch_node_weight)]
    // LinearDodge {
    //     color_a: Box<FloatColorNodes>,
    //     color_b: Box<FloatColorNodes>,
    //     value: Box<UNFloatNodes>,
    // },

    // #[mutagen(gen_weight = branch_node_weight)]
    // Multiply {
    //     color_a: Box<FloatColorNodes>,
    //     color_b: Box<FloatColorNodes>,
    //     value: Box<UNFloatNodes>,
    // },

    // #[mutagen(gen_weight = branch_node_weight)]
    // ColorBurn {
    //     color_a: Box<FloatColorNodes>,
    //     color_b: Box<FloatColorNodes>,
    //     value: Box<UNFloatNodes>,
    // },

    // #[mutagen(gen_weight = branch_node_weight)]
    // LinearBurn {
    //     color_a: Box<FloatColorNodes>,
    //     color_b: Box<FloatColorNodes>,
    //     value: Box<UNFloatNodes>,
    // },

    // #[mutagen(gen_weight = branch_node_weight)]
    // VividLight {
    //     color_a: Box<FloatColorNodes>,
    //     color_b: Box<FloatColorNodes>,
    //     value: Box<UNFloatNodes>,
    // },

    // #[mutagen(gen_weight = branch_node_weight)]
    // LinearLight {
    //     color_a: Box<FloatColorNodes>,
    //     color_b: Box<FloatColorNodes>,
    //     value: Box<UNFloatNodes>,
    // },

    // #[mutagen(gen_weight = branch_node_weight)]
    // Subtract {
    //     color_a: Box<FloatColorNodes>,
    //     color_b: Box<FloatColorNodes>,
    //     value: Box<UNFloatNodes>,
    // },

    // #[mutagen(gen_weight = branch_node_weight)]
    // Divide {
    //     color_a: Box<FloatColorNodes>,
    //     color_b: Box<FloatColorNodes>,
    //     value: Box<UNFloatNodes>,
    // },

    // #[mutagen(gen_weight = branch_node_weight)]
    // Lerp {
    //     color_a: Box<FloatColorNodes>,
    //     color_b: Box<FloatColorNodes>,
    //     value: Box<UNFloatNodes>,
    // },
}

impl Node for ColorBlendNodes {
    type Output = FloatColor;

    fn compute(&self, state: UpdateState) -> Self::Output {
        use ColorBlendNodes::*;

        match self {
            Gray => FloatColor {
                r: 1.0,
                g: 1.0,
                b: 1.0,
                a: 1.0,
            },
            Invert { child } => {
                let col = child.compute(state);
                FloatColor {
                    r: 1.0 - col.r,
                    g: 1.0 - col.g,
                    b: 1.0 - col.b,
                    a: 1.0 - col.a,
                }
            }
            Dissolve {
                color_a,
                color_b,
                value,
            } => {
                if UNFloat::generate().into_inner() < value.compute(state).into_inner() {
                    color_a.compute(state)
                } else {
                    color_b.compute(state)
                }
            }
            // Overlay {color_a, color_b, value} => {if UNFloat::generate().into_inner() < value.compute(state).into_inner() {color_a.compute(state)}else{color_b.compute(state)}},
            // ScreenDodge {color_a, color_b, value} => {if UNFloat::generate().into_inner() < value.compute(state).into_inner() {color_a.compute(state)}else{color_b.compute(state)}},
            // ColorDodge {color_a, color_b, value} => {if UNFloat::generate().into_inner() < value.compute(state).into_inner() {color_a.compute(state)}else{color_b.compute(state)}},
            // LinearDodge {color_a, color_b, value} => {if UNFloat::generate().into_inner() < value.compute(state).into_inner() {color_a.compute(state)}else{color_b.compute(state)}},
            // Multiply {color_a, color_b, value} => {if UNFloat::generate().into_inner() < value.compute(state).into_inner() {color_a.compute(state)}else{color_b.compute(state)}},
            // ColorBurn {color_a, color_b, value} => {if UNFloat::generate().into_inner() < value.compute(state).into_inner() {color_a.compute(state)}else{color_b.compute(state)}},
            // LinearBurn {color_a, color_b, value} => {if UNFloat::generate().into_inner() < value.compute(state).into_inner() {color_a.compute(state)}else{color_b.compute(state)}},
            // VividLight {color_a, color_b, value} => {if UNFloat::generate().into_inner() < value.compute(state).into_inner() {color_a.compute(state)}else{color_b.compute(state)}},
            // LinearLight {color_a, color_b, value} => {if UNFloat::generate().into_inner() < value.compute(state).into_inner() {color_a.compute(state)}else{color_b.compute(state)}},
            // Subtract {color_a, color_b, value} => {if UNFloat::generate().into_inner() < value.compute(state).into_inner() {color_a.compute(state)}else{color_b.compute(state)}},
            // Divide {color_a, color_b, value} => {if UNFloat::generate().into_inner() < value.compute(state).into_inner() {color_a.compute(state)}else{color_b.compute(state)}},
            // Lerp {color_a, color_b, value} => {if UNFloat::generate().into_inner() < value.compute(state).into_inner() {color_a.compute(state)}else{color_b.compute(state)}},
        }
    }
}
