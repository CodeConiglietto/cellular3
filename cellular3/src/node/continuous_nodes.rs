use crate::{
    datatype::{colors::*, continuous::*},
    node::{
        color_nodes::*, coord_map_nodes::*, discrete_nodes::*, mutagen_functions::*,
        noise_nodes::*, point_nodes::*, Node,
    },
    updatestate::*,
};
use mutagen::{Generatable, Mutatable};
use nalgebra::*;

#[derive(Generatable, Mutatable, Debug)]
#[mutagen(mut_reroll = 0.1)]
pub enum AngleNodes {
    #[mutagen(gen_weight = leaf_node_weight)]
    FromGametic,

    #[mutagen(gen_weight = pipe_node_weight)]
    ArcSin { theta: Box<SNFloatNodes> },

    #[mutagen(gen_weight = pipe_node_weight)]
    ArcCos { theta: Box<SNFloatNodes> },

    // #[mutagen(gen_weight = leaf_node_weight)]
    // #[mutagen(mut_reroll = 0.9)]
    // Random,
    #[mutagen(gen_weight = leaf_node_weight)]
    FromCoordinate,
    // #[mutagen(gen_weight = leaf_node_weight)]
    // Constant { value: Angle },
    #[mutagen(gen_weight = pipe_node_weight)]
    FromSNPoint { child: Box<SNPointNodes> },

    #[mutagen(gen_weight = pipe_node_weight)]
    FromSNFloat { child: Box<SNFloatNodes> },

    #[mutagen(gen_weight = pipe_node_weight)]
    FromUNFloat { child: Box<UNFloatNodes> },

    #[mutagen(gen_weight = branch_node_weight)]
    ModifyState {
        child: Box<AngleNodes>,
        child_state: Box<CoordMapNodes>,
    },
    #[mutagen(gen_weight = branch_node_weight)]
    IfElse {
        predicate: Box<BooleanNodes>,
        child_a: Box<Self>,
        child_b: Box<Self>,
    },
}

impl Node for AngleNodes {
    type Output = Angle;

    fn compute(&self, state: UpdateState) -> Self::Output {
        use AngleNodes::*;

        match self {
            FromGametic => Angle::new(state.coordinate_set.t * 0.1),
            ArcSin { theta } => Angle::new(f32::asin(theta.compute(state).into_inner())),
            ArcCos { theta } => Angle::new(f32::acos(theta.compute(state).into_inner())),
            FromCoordinate => Angle::new(f32::atan2(
                -state.coordinate_set.x.into_inner(),
                state.coordinate_set.y.into_inner(),
            )),
            // Random => Angle::generate(),
            // Constant { value } => *value,
            FromSNPoint { child } => child.compute(state).to_angle(),
            FromSNFloat { child } => child.compute(state).to_angle(),
            FromUNFloat { child } => child.compute(state).to_angle(),
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
pub enum SNFloatNodes {
    #[mutagen(gen_weight = pipe_node_weight)]
    Sin { child: Box<AngleNodes> },

    #[mutagen(gen_weight = pipe_node_weight)]
    Cos { child: Box<AngleNodes> },

    // #[mutagen(gen_weight = leaf_node_weight)]
    // Random,
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

    #[mutagen(gen_weight = pipe_node_weight)]
    Invert { child: Box<SNFloatNodes> },

    #[mutagen(gen_weight = leaf_node_weight)]
    XRatio,

    #[mutagen(gen_weight = leaf_node_weight)]
    YRatio,

    #[mutagen(gen_weight = leaf_node_weight)]
    FromGametic,

    #[mutagen(gen_weight = leaf_node_weight)]
    NoiseFunction { child: Box<NoiseNodes> },

    #[mutagen(gen_weight = branch_node_weight)]
    SubDivide {
        child_a: Box<Self>,
        child_b: Box<NibbleNodes>,
    },

    #[mutagen(gen_weight = branch_node_weight)]
    ModifyState {
        child: Box<SNFloatNodes>,
        child_state: Box<CoordMapNodes>,
    },
    #[mutagen(gen_weight = branch_node_weight)]
    IfElse {
        predicate: Box<BooleanNodes>,
        child_a: Box<Self>,
        child_b: Box<Self>,
    },
}

impl Node for SNFloatNodes {
    type Output = SNFloat;

    fn compute(&self, state: UpdateState) -> Self::Output {
        use SNFloatNodes::*;

        match self {
            Sin { child } => SNFloat::new(f32::sin(child.compute(state).into_inner())),
            Cos { child } => SNFloat::new(f32::cos(child.compute(state).into_inner())),
            // Random => SNFloat::generate(),
            FromAngle { child } => child.compute(state).to_signed(),
            FromUNFloat { child } => child.compute(state).to_signed(),
            Constant { value } => *value,
            Multiply { child_a, child_b } => SNFloat::new(
                child_a.compute(state).into_inner() * child_b.compute(state).into_inner(),
            ),
            Abs { child } => SNFloat::new(child.compute(state).into_inner().abs()),
            Invert { child } => SNFloat::new(child.compute(state).into_inner() * -1.0),
            XRatio => state.coordinate_set.x,
            YRatio => state.coordinate_set.y,
            FromGametic => {
                SNFloat::new((state.coordinate_set.t - state.coordinate_set.t.floor()) * 2.0 - 1.0)
            }
            ModifyState { child, child_state } => child.compute(UpdateState {
                coordinate_set: child_state.compute(state),
                ..state
            }),
            NoiseFunction { child } => child.compute(state),
            SubDivide { child_a, child_b } => {
                child_a.compute(state).subdivide(child_b.compute(state))
            }
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
pub enum UNFloatNodes {
    // #[mutagen(gen_weight = leaf_node_weight)]
    // Random,
    #[mutagen(gen_weight = leaf_node_weight)]
    Constant { value: UNFloat },
    #[mutagen(gen_weight = pipe_node_weight)]
    FromAngle { child: Box<AngleNodes> },
    #[mutagen(gen_weight = pipe_node_weight)]
    FromSNFloat { child: Box<SNFloatNodes> },
    #[mutagen(gen_weight = pipe_node_weight)]
    AbsSNFloat { child: Box<SNFloatNodes> },
    #[mutagen(gen_weight = pipe_node_weight)]
    SquareSNFloat { child: Box<SNFloatNodes> },
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
    InvertNormalised { child: Box<UNFloatNodes> },
    #[mutagen(gen_weight = pipe_node_weight)]
    ColorAverage { child: Box<FloatColorNodes> },
    #[mutagen(gen_weight = pipe_node_weight)]
    ColorComponentR { child: Box<FloatColorNodes> },
    #[mutagen(gen_weight = pipe_node_weight)]
    ColorComponentG { child: Box<FloatColorNodes> },
    #[mutagen(gen_weight = pipe_node_weight)]
    ColorComponentB { child: Box<FloatColorNodes> },
    #[mutagen(gen_weight = pipe_node_weight)]
    ColorComponentH { child: Box<FloatColorNodes> },
    #[mutagen(gen_weight = leaf_node_weight)]
    FromGametic,
    #[mutagen(gen_weight = pipe_node_weight)]
    Mandelbrot { 
        child_power: Box<UNFloatNodes>,
        child_offset: Box<SNPointNodes>, 
        child_scale: Box<SNPointNodes>,
        child_iterations: Box<ByteNodes> 
    },
    // #[mutagen(gen_weight = leaf_node_weight)]
    // LastRotation,
    #[mutagen(gen_weight = branch_node_weight)]
    SubDivide {
        child_a: Box<Self>,
        child_b: Box<NibbleNodes>,
    },
    #[mutagen(gen_weight = branch_node_weight)]
    EuclideanDistance {
        child_a: Box<SNPointNodes>,
        child_b: Box<SNPointNodes>,
    },
    #[mutagen(gen_weight = branch_node_weight)]
    ModifyState {
        child: Box<UNFloatNodes>,
        child_state: Box<CoordMapNodes>,
    },
    #[mutagen(gen_weight = branch_node_weight)]
    IfElse {
        predicate: Box<BooleanNodes>,
        child_a: Box<Self>,
        child_b: Box<Self>,
    },
}

impl Node for UNFloatNodes {
    type Output = UNFloat;

    fn compute(&self, state: UpdateState) -> Self::Output {
        use UNFloatNodes::*;

        match self {
            // Random => UNFloat::generate(),
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
            ColorComponentH { child } => get_hue_unfloat(child.compute(state)),
            FromGametic => state.coordinate_set.get_unfloat_t(),
            Mandelbrot { child_power, child_offset, child_scale, child_iterations } => {
                let power = 1.0 + child_power.compute(state).into_inner() * 8.0;
                let offset = child_offset.compute(state).into_inner();
                let scale = child_scale.compute(state).into_inner();
                let mut z = Complex { re: 0.0, im: 0.0 };
                //scaling in this fashion will give us a lot of boring stuff :<
                let c = Complex { re: ((state.coordinate_set.x.into_inner() * scale.x) + offset.x) * 0.5, im: ((state.coordinate_set.y.into_inner() * scale.y) + offset.y)};
                let mut escape = 0;
                let iterations = child_iterations.compute(state).into_inner() / 2;
                for i in 0..=iterations {
                    z = z.powf(power) + c;  
                    if z.norm_sqr() > 4.0 {
                        escape = i;
                        break;
                    }
                }

                UNFloat::new(escape as f32 / (1 + iterations) as f32)
            }
            SubDivide { child_a, child_b } => {
                child_a.compute(state).subdivide(child_b.compute(state))
            }
            EuclideanDistance { child_a, child_b } => UNFloat::new(
                (distance(
                    &child_a.compute(state).into_inner(),
                    &child_b.compute(state).into_inner(),
                ) * 0.5)
                    .min(1.0),
            ),
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
