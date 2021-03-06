use crate::{
    datatype::{colors::*, continuous::*},
    node::{
        color_nodes::*, continuous_nodes::*, coord_map_nodes::*, discrete_nodes::*,
        mutagen_functions::*, Node,
    },
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
    #[mutagen(gen_weight = branch_node_weight)]
    Overlay {
        color_a: Box<FloatColorNodes>,
        color_b: Box<FloatColorNodes>,
    },

    #[mutagen(gen_weight = branch_node_weight)]
    ScreenDodge {
        color_a: Box<FloatColorNodes>,
        color_b: Box<FloatColorNodes>,
    },

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
    #[mutagen(gen_weight = branch_node_weight)]
    ModifyState {
        child: Box<ColorBlendNodes>,
        child_state: Box<CoordMapNodes>,
    },
    #[mutagen(gen_weight = branch_node_weight)]
    IfElse {
        predicate: Box<BooleanNodes>,
        child_a: Box<Self>,
        child_b: Box<Self>,
    },
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
            Overlay { color_a, color_b } => {
                let a_result = color_a.compute(state);
                let b_result = color_b.compute(state);

                FloatColor {
                    r: if a_result.r < 0.5 {
                        (2.0 * a_result.r * b_result.r).max(1.0)
                    } else {
                        1.0 - (2.0 * ((1.0 - a_result.r) * (1.0 - b_result.r)))
                    },
                    g: if a_result.g < 0.5 {
                        (2.0 * a_result.g * b_result.g).max(1.0)
                    } else {
                        1.0 - (2.0 * ((1.0 - a_result.g) * (1.0 - b_result.g)))
                    },
                    b: if a_result.b < 0.5 {
                        (2.0 * a_result.b * b_result.b).max(1.0)
                    } else {
                        1.0 - (2.0 * ((1.0 - a_result.b) * (1.0 - b_result.b)))
                    },
                    a: 1.0,
                }
            }
            ScreenDodge { color_a, color_b } => {
                let result_a = color_a.compute(state);
                let result_b = color_b.compute(state);

                FloatColor {
                    r: 1.0 - ((1.0 - result_a.r) * (1.0 - result_b.r)),
                    g: 1.0 - ((1.0 - result_a.g) * (1.0 - result_b.g)),
                    b: 1.0 - ((1.0 - result_a.b) * (1.0 - result_b.b)),
                    a: 1.0,
                }
            }
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
