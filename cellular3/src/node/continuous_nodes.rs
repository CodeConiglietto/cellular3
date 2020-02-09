use crate::{
    datatype::{continuous::*},
    node::{color_nodes::*, noise_nodes::*, coord_map_nodes::*, mutagen_functions::*, Node},
    updatestate::*,
};
use mutagen::{Generatable, Mutatable};

#[derive(Generatable, Mutatable, Debug)]
#[mutagen(mut_reroll = 0.1)]
pub enum AngleNodes {
    #[mutagen(gen_weight = pipe_node_weight)]
    ArcSin { theta: Box<SNFloatNodes> },

    #[mutagen(gen_weight = pipe_node_weight)]
    ArcCos { theta: Box<SNFloatNodes> },

    #[mutagen(gen_weight = leaf_node_weight)]
    #[mutagen(mut_reroll = 0.9)]
    Random,

    #[mutagen(gen_weight = leaf_node_weight)]
    Constant { value: Angle },

    #[mutagen(gen_weight = pipe_node_weight)]
    FromSNFloat { child: Box<SNFloatNodes> },

    #[mutagen(gen_weight = pipe_node_weight)]
    FromUNFloat { child: Box<UNFloatNodes> },

    #[mutagen(gen_weight = branch_node_weight)]
    ModifyState {
        child: Box<AngleNodes>,
        child_state: Box<CoordMapNodes>,
    },
}

impl Node for AngleNodes {
    type Output = Angle;

    fn compute(&self, state: UpdateState) -> Self::Output {
        use AngleNodes::*;

        match self {
            ArcSin { theta } => Angle::new(f32::asin(theta.compute(state).into_inner())),
            ArcCos { theta } => Angle::new(f32::acos(theta.compute(state).into_inner())),
            Random => Angle::generate(),
            Constant { value } => *value,
            FromSNFloat { child } => child.compute(state).to_angle(),
            FromUNFloat { child } => child.compute(state).to_angle(),
            ModifyState { child, child_state } => child.compute(UpdateState {
                coordinate_set: child_state.compute(state),
                cell_array: state.cell_array,
            }),
        }
    }
}

#[derive(Generatable, Mutatable, Debug)]
#[mutagen(mut_reroll = 0.1)]
pub enum SNFloatNodes {
    #[mutagen(gen_weight = pipe_node_weight)]
    Sin { child: Box<AngleNodes> },

    #[mutagen(gen_weight = pipe_node_weight)]
    Cos { child: Box<AngleNodes> },

    #[mutagen(gen_weight = leaf_node_weight)]
    Random,

    #[mutagen(gen_weight = leaf_node_weight)]
    Constant { value: SNFloat },

    #[mutagen(gen_weight = pipe_node_weight)]
    FromAngle { child: Box<AngleNodes> },

    #[mutagen(gen_weight = pipe_node_weight)]
    FromUNFloat { child: Box<UNFloatNodes> },

    #[mutagen(gen_weight = branch_node_weight)]
    Multiply {
        child_a: Box<SNFloatNodes>,
        child_b: Box<SNFloatNodes>,
    },

    #[mutagen(gen_weight = pipe_node_weight)]
    Abs { child: Box<SNFloatNodes> },

    #[mutagen(gen_weight = leaf_node_weight)]
    XRatio,

    #[mutagen(gen_weight = leaf_node_weight)]
    YRatio,

    #[mutagen(gen_weight = leaf_node_weight)]
    NoiseFunction { child: Box<NoiseNodes> },

    #[mutagen(gen_weight = branch_node_weight)]
    ModifyState {
        child: Box<SNFloatNodes>,
        child_state: Box<CoordMapNodes>,
    },
}

impl Node for SNFloatNodes {
    type Output = SNFloat;

    fn compute(&self, state: UpdateState) -> Self::Output {
        use SNFloatNodes::*;

        match self {
            Sin { child } => SNFloat::new(f32::sin(child.compute(state).into_inner())),
            Cos { child } => SNFloat::new(f32::cos(child.compute(state).into_inner())),
            Random => SNFloat::generate(),
            FromAngle { child } => child.compute(state).to_signed(),
            FromUNFloat { child } => child.compute(state).to_signed(),
            Constant { value } => *value,
            Multiply { child_a, child_b } => SNFloat::new(
                child_a.compute(state).into_inner() * child_b.compute(state).into_inner(),
            ),
            Abs { child } => SNFloat::new(child.compute(state).into_inner().abs()),
            XRatio => state.coordinate_set.x,
            YRatio => state.coordinate_set.y,
            ModifyState { child, child_state } => child.compute(UpdateState {
                coordinate_set: child_state.compute(state),
                cell_array: state.cell_array,
            }),
            NoiseFunction { child } => child.compute(state),
        }
    }
}

#[derive(Generatable, Mutatable, Debug)]
#[mutagen(mut_reroll = 0.1)]
pub enum UNFloatNodes {
    #[mutagen(gen_weight = leaf_node_weight)]
    Random,
    #[mutagen(gen_weight = leaf_node_weight)]
    Constant {
        value: UNFloat,
    },
    #[mutagen(gen_weight = pipe_node_weight)]
    FromAngle {
        child: Box<AngleNodes>,
    },
    #[mutagen(gen_weight = pipe_node_weight)]
    FromSNFloat {
        child: Box<SNFloatNodes>,
    },
    #[mutagen(gen_weight = pipe_node_weight)]
    AbsSNFloat {
        child: Box<SNFloatNodes>,
    },
    #[mutagen(gen_weight = pipe_node_weight)]
    SquareSNFloat {
        child: Box<SNFloatNodes>,
    },
    #[mutagen(gen_weight = branch_node_weight)]
    Multiply {
        child_a: Box<UNFloatNodes>,
        child_b: Box<UNFloatNodes>,
    },
    #[mutagen(gen_weight = branch_node_weight)]
    CircularAdd {
        child_a: Box<UNFloatNodes>,
        child_b: Box<UNFloatNodes>,
    },
    #[mutagen(gen_weight = pipe_node_weight)]
    InvertNormalised {
        child: Box<UNFloatNodes>,
    },
    #[mutagen(gen_weight = pipe_node_weight)]
    ColorAverage {
        child: Box<FloatColorNodes>,
    },
    #[mutagen(gen_weight = pipe_node_weight)]
    ColorComponentR {
        child: Box<FloatColorNodes>,
    },
    #[mutagen(gen_weight = pipe_node_weight)]
    ColorComponentG {
        child: Box<FloatColorNodes>,
    },
    #[mutagen(gen_weight = pipe_node_weight)]
    ColorComponentB {
        child: Box<FloatColorNodes>,
    },
    #[mutagen(gen_weight = branch_node_weight)]
    ModifyState {
        child: Box<UNFloatNodes>,
        child_state: Box<CoordMapNodes>,
    },
}

impl Node for UNFloatNodes {
    type Output = UNFloat;

    fn compute(&self, state: UpdateState) -> Self::Output {
        use UNFloatNodes::*;

        match self {
            Random => UNFloat::generate(),
            Constant { value } => *value,
            FromAngle { child } => child.compute(state).to_unsigned(),
            FromSNFloat { child } => child.compute(state).to_unsigned(),
            AbsSNFloat { child } => UNFloat::new(child.compute(state).into_inner().abs()),
            SquareSNFloat { child } => UNFloat::new(child.compute(state).into_inner().powf(2.0)),
            Multiply { child_a, child_b } => UNFloat::new(
                child_a.compute(state).into_inner() * child_b.compute(state).into_inner(),
            ),
            CircularAdd { child_a, child_b } => {
                let value =
                    child_a.compute(state).into_inner() + child_b.compute(state).into_inner();
                UNFloat::new(value - (value.floor()))
            }
            InvertNormalised { child } => UNFloat::new(1.0 - child.compute(state).into_inner()),
            ColorAverage { child } => {
                let color = child.compute(state);
                UNFloat::new((color.r + color.g + color.b) / 3.0)
            }
            ColorComponentR { child } => UNFloat::new(child.compute(state).r),
            ColorComponentG { child } => UNFloat::new(child.compute(state).g),
            ColorComponentB { child } => UNFloat::new(child.compute(state).b),
            ModifyState { child, child_state } => child.compute(UpdateState {
                coordinate_set: child_state.compute(state),
                cell_array: state.cell_array,
            }),
        }
    }
}
