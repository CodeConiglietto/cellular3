use crate::{
    datatype::discrete::*,
    node::{continuous_nodes::*, coord_map_nodes::*, mutagen_functions::*, Node},
    updatestate::*,
};
use mutagen::{Generatable, Mutatable};

#[derive(Generatable, Mutatable, Debug)]
pub enum BooleanNodes {
    #[mutagen(gen_weight = branch_node_weight)]
    UNFloatLess {
        child_a: Box<UNFloatNodes>,
        child_b: Box<UNFloatNodes>,
    },
    #[mutagen(gen_weight = branch_node_weight)]
    UNFloatMore {
        child_a: Box<UNFloatNodes>,
        child_b: Box<UNFloatNodes>,
    },
    #[mutagen(gen_weight = branch_node_weight)]
    And {
        child_a: Box<BooleanNodes>,
        child_b: Box<BooleanNodes>,
    },
    #[mutagen(gen_weight = branch_node_weight)]
    Or {
        child_a: Box<BooleanNodes>,
        child_b: Box<BooleanNodes>,
    },
    #[mutagen(gen_weight = pipe_node_weight)]
    Not { child: Box<BooleanNodes> },
    #[mutagen(gen_weight = leaf_node_weight)]
    Constant { child: Boolean },
    #[mutagen(mut_reroll = 0.9)]
    // #[mutagen(gen_weight = leaf_node_weight)]
    // Random,
    #[mutagen(gen_weight = branch_node_weight)]
    ModifyState {
        child: Box<BooleanNodes>,
        child_state: Box<CoordMapNodes>,
    },
    #[mutagen(gen_weight = branch_node_weight)]
    IfElse {
        predicate: Box<BooleanNodes>,
        child_a: Box<Self>,
        child_b: Box<Self>,
    },
}

impl Node for BooleanNodes {
    type Output = Boolean;

    fn compute(&self, state: UpdateState) -> Self::Output {
        use BooleanNodes::*;

        match self {
            UNFloatLess { child_a, child_b } => Boolean {
                value: child_a.compute(state).into_inner() < child_b.compute(state).into_inner(),
            },
            UNFloatMore { child_a, child_b } => Boolean {
                value: child_a.compute(state).into_inner() > child_b.compute(state).into_inner(),
            },
            And { child_a, child_b } => Boolean {
                value: child_a.compute(state).into_inner() && child_b.compute(state).into_inner(),
            },
            Or { child_a, child_b } => Boolean {
                value: child_a.compute(state).into_inner() || child_b.compute(state).into_inner(),
            },
            Not { child } => Boolean {
                value: !child.compute(state).into_inner(),
            },
            Constant { child } => *child,
            // Random => Boolean::generate(),
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
pub enum ByteNodes {
    #[mutagen(gen_weight = leaf_node_weight)]
    Constant { value: Byte },
    // #[mutagen(gen_weight = leaf_node_weight)]
    // Random,
    #[mutagen(gen_weight = branch_node_weight)]
    Add {
        child_a: Box<ByteNodes>,
        child_b: Box<ByteNodes>,
    },
    #[mutagen(gen_weight = branch_node_weight)]
    Multiply {
        child_a: Box<ByteNodes>,
        child_b: Box<ByteNodes>,
    },
    #[mutagen(gen_weight = branch_node_weight)]
    Divide {
        child_value: Box<ByteNodes>,
        child_divisor: Box<ByteNodes>,
    },
    #[mutagen(gen_weight = branch_node_weight)]
    Modulus {
        child_value: Box<ByteNodes>,
        child_divisor: Box<ByteNodes>,
    },
    #[mutagen(gen_weight = leaf_node_weight)]
    FromGametic,
    #[mutagen(gen_weight = branch_node_weight)]
    IfElse {
        predicate: Box<BooleanNodes>,
        child_a: Box<Self>,
        child_b: Box<Self>,
    },
    // InvertNormalised { child: Box<ByteNodes> },
}

impl Node for ByteNodes {
    type Output = Byte;

    fn compute(&self, state: UpdateState) -> Self::Output {
        use ByteNodes::*;

        match self {
            Constant { value } => *value,
            // Random => Byte::generate(),
            Add { child_a, child_b } => child_a.compute(state).add(child_b.compute(state)),
            Multiply { child_a, child_b } => {
                child_a.compute(state).multiply(child_b.compute(state))
            }
            Divide {
                child_value,
                child_divisor,
            } => child_value
                .compute(state)
                .divide(child_divisor.compute(state)),
            Modulus {
                child_value,
                child_divisor,
            } => child_value
                .compute(state)
                .modulus(child_divisor.compute(state)),
            FromGametic => state.coordinate_set.get_byte_t(),
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
pub enum UIntNodes {
    #[mutagen(gen_weight = leaf_node_weight)]
    Constant { value: UInt },
    // #[mutagen(gen_weight = leaf_node_weight)]
    // Random,
    #[mutagen(gen_weight = branch_node_weight)]
    Add {
        child_a: Box<UIntNodes>,
        child_b: Box<UIntNodes>,
    },
    #[mutagen(gen_weight = branch_node_weight)]
    Multiply {
        child_a: Box<UIntNodes>,
        child_b: Box<UIntNodes>,
    },
    #[mutagen(gen_weight = branch_node_weight)]
    Divide {
        child_value: Box<UIntNodes>,
        child_divisor: Box<UIntNodes>,
    },
    #[mutagen(gen_weight = branch_node_weight)]
    Modulus {
        child_value: Box<UIntNodes>,
        child_divisor: Box<UIntNodes>,
    },
    #[mutagen(gen_weight = leaf_node_weight)]
    FromGametic,
    #[mutagen(gen_weight = branch_node_weight)]
    IfElse {
        predicate: Box<BooleanNodes>,
        child_a: Box<Self>,
        child_b: Box<Self>,
    },
}

impl Node for UIntNodes {
    type Output = UInt;

    fn compute(&self, state: UpdateState) -> Self::Output {
        use UIntNodes::*;

        match self {
            Constant { value } => *value,
            // Random => UInt::generate(),
            Add { child_a, child_b } => child_a.compute(state).add(child_b.compute(state)),
            Multiply { child_a, child_b } => {
                child_a.compute(state).multiply(child_b.compute(state))
            }
            Divide {
                child_value,
                child_divisor,
            } => child_value
                .compute(state)
                .divide(child_divisor.compute(state)),
            Modulus {
                child_value,
                child_divisor,
            } => child_value
                .compute(state)
                .modulus(child_divisor.compute(state)),
            FromGametic => UInt::new(state.coordinate_set.t as u32),
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
pub enum SIntNodes {
    #[mutagen(gen_weight = leaf_node_weight)]
    Constant { value: SInt },
    // #[mutagen(gen_weight = leaf_node_weight)]
    // Random,
    #[mutagen(gen_weight = branch_node_weight)]
    Add {
        child_a: Box<SIntNodes>,
        child_b: Box<SIntNodes>,
    },
    #[mutagen(gen_weight = branch_node_weight)]
    Multiply {
        child_a: Box<SIntNodes>,
        child_b: Box<SIntNodes>,
    },
    #[mutagen(gen_weight = branch_node_weight)]
    Divide {
        child_value: Box<SIntNodes>,
        child_divisor: Box<SIntNodes>,
    },
    #[mutagen(gen_weight = branch_node_weight)]
    Modulus {
        child_value: Box<SIntNodes>,
        child_divisor: Box<SIntNodes>,
    },
    #[mutagen(gen_weight = branch_node_weight)]
    IfElse {
        predicate: Box<BooleanNodes>,
        child_a: Box<Self>,
        child_b: Box<Self>,
    },
}

impl Node for SIntNodes {
    type Output = SInt;

    fn compute(&self, state: UpdateState) -> Self::Output {
        use SIntNodes::*;

        match self {
            Constant { value } => *value,
            // Random => SInt::generate(),
            Add { child_a, child_b } => child_a.compute(state).add(child_b.compute(state)),
            Multiply { child_a, child_b } => {
                child_a.compute(state).multiply(child_b.compute(state))
            }
            Divide {
                child_value,
                child_divisor,
            } => child_value
                .compute(state)
                .divide(child_divisor.compute(state)),
            Modulus {
                child_value,
                child_divisor,
            } => child_value
                .compute(state)
                .modulus(child_divisor.compute(state)),
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
