use crate::{
    constants::*,
    datatype::{continuous::*, discrete::*, noisefunctions::*},
    node::{color_nodes::*, Node},
    updatestate::*,
};
use mutagen::{Generatable, Mutatable};
use noise::NoiseFn;

#[derive(Generatable, Mutatable, Debug)]
#[mutagen(mut_reroll = 0.1)]
pub enum AngleNodes {
    ArcSin { theta: Box<SNFloatNodes> },
    ArcCos { theta: Box<SNFloatNodes> },
    Random,
    Constant { value: Angle },
    FromSNFloat { child: Box<SNFloatNodes> },
    FromUNFloat { child: Box<UNFloatNodes> },
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
        }
    }
}

#[derive(Generatable, Mutatable, Debug)]
#[mutagen(mut_reroll = 0.1)]
pub enum SNFloatNodes {
    Sin {
        child: Box<AngleNodes>,
    },
    Cos {
        child: Box<AngleNodes>,
    },
    Random,
    Constant {
        value: SNFloat,
    },
    FromAngle {
        child: Box<AngleNodes>,
    },
    FromUNFloat {
        child: Box<UNFloatNodes>,
    },
    BasicMultiFractalNoise {
        noise: BasicMultiFractalNoise,
    },
    BillowNoise {
        noise: BasicMultiFractalNoise,
    },
    CheckerboardNoise {
        noise: BasicMultiFractalNoise,
    },
    FractalBrownianNoise {
        noise: FractalBrownianNoise,
    },
    HybridMultiFractalNoise {
        noise: BasicMultiFractalNoise,
    },
    OpenSimplexNoise {
        noise: OpenSimplexNoise,
    },
    RidgedMultiFractalNoise {
        noise: RidgedMultiFractalNoise,
    },
    SuperSimplexNoise {
        noise: SuperSimplexNoise,
    },
    ValueNoise {
        noise: RidgedMultiFractalNoise,
    },
    WorleyNoise {
        noise: WorleyNoise,
    },
    Multiply {
        child_a: Box<SNFloatNodes>,
        child_b: Box<SNFloatNodes>,
    },
    Abs {
        child: Box<SNFloatNodes>,
    },
    XRatio,
    YRatio,
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
            BasicMultiFractalNoise { noise } => SNFloat::new(noise.noise.get([
                state.coordinate_set.x as f64 * noise.x_scale.into_inner().powf(2.0) as f64 * NOISE_X_SCALE_FACTOR,
                state.coordinate_set.y as f64 * noise.y_scale.into_inner().powf(2.0) as f64 * NOISE_Y_SCALE_FACTOR,
                state.coordinate_set.t as f64 * noise.t_scale.into_inner() as f64 * NOISE_T_SCALE_FACTOR,
            ]) as f32),
            BillowNoise { noise } => SNFloat::new(noise.noise.get([
                state.coordinate_set.x as f64 * noise.x_scale.into_inner().powf(2.0) as f64 * NOISE_X_SCALE_FACTOR,
                state.coordinate_set.y as f64 * noise.y_scale.into_inner().powf(2.0) as f64 * NOISE_Y_SCALE_FACTOR,
                state.coordinate_set.t as f64 * noise.t_scale.into_inner() as f64 * NOISE_T_SCALE_FACTOR,
            ]) as f32),
            CheckerboardNoise { noise } => SNFloat::new(noise.noise.get([
                state.coordinate_set.x as f64 * noise.x_scale.into_inner().powf(2.0) as f64 * NOISE_X_SCALE_FACTOR,
                state.coordinate_set.y as f64 * noise.y_scale.into_inner().powf(2.0) as f64 * NOISE_Y_SCALE_FACTOR,
                state.coordinate_set.t as f64 * noise.t_scale.into_inner() as f64 * NOISE_T_SCALE_FACTOR,
            ]) as f32),
            FractalBrownianNoise { noise } => SNFloat::new(noise.noise.get([
                state.coordinate_set.x as f64 * noise.x_scale.into_inner().powf(2.0) as f64 * NOISE_X_SCALE_FACTOR,
                state.coordinate_set.y as f64 * noise.y_scale.into_inner().powf(2.0) as f64 * NOISE_Y_SCALE_FACTOR,
                state.coordinate_set.t as f64 * noise.t_scale.into_inner() as f64 * NOISE_T_SCALE_FACTOR,
            ]) as f32),
            HybridMultiFractalNoise { noise } => SNFloat::new(noise.noise.get([
                state.coordinate_set.x as f64 * noise.x_scale.into_inner().powf(2.0) as f64 * NOISE_X_SCALE_FACTOR,
                state.coordinate_set.y as f64 * noise.y_scale.into_inner().powf(2.0) as f64 * NOISE_Y_SCALE_FACTOR,
                state.coordinate_set.t as f64 * noise.t_scale.into_inner() as f64 * NOISE_T_SCALE_FACTOR,
            ]) as f32),
            OpenSimplexNoise { noise } => SNFloat::new(noise.noise.get([
                state.coordinate_set.x as f64 * noise.x_scale.into_inner().powf(2.0) as f64 * NOISE_X_SCALE_FACTOR,
                state.coordinate_set.y as f64 * noise.y_scale.into_inner().powf(2.0) as f64 * NOISE_Y_SCALE_FACTOR,
                state.coordinate_set.t as f64 * noise.t_scale.into_inner() as f64 * NOISE_T_SCALE_FACTOR,
            ]) as f32),
            RidgedMultiFractalNoise { noise } => SNFloat::new(noise.noise.get([
                state.coordinate_set.x as f64 * noise.x_scale.into_inner().powf(2.0) as f64 * NOISE_X_SCALE_FACTOR,
                state.coordinate_set.y as f64 * noise.y_scale.into_inner().powf(2.0) as f64 * NOISE_Y_SCALE_FACTOR,
                state.coordinate_set.t as f64 * noise.t_scale.into_inner() as f64 * NOISE_T_SCALE_FACTOR,
            ]) as f32),
            SuperSimplexNoise { noise } => SNFloat::new(noise.noise.get([
                state.coordinate_set.x as f64 * noise.x_scale.into_inner().powf(2.0) as f64 * NOISE_X_SCALE_FACTOR,
                state.coordinate_set.y as f64 * noise.y_scale.into_inner().powf(2.0) as f64 * NOISE_Y_SCALE_FACTOR,
                state.coordinate_set.t as f64 * noise.t_scale.into_inner() as f64 * NOISE_T_SCALE_FACTOR,
            ]) as f32),
            ValueNoise { noise } => SNFloat::new(noise.noise.get([
                state.coordinate_set.x as f64 * noise.x_scale.into_inner().powf(2.0) as f64 * NOISE_X_SCALE_FACTOR,
                state.coordinate_set.y as f64 * noise.y_scale.into_inner().powf(2.0) as f64 * NOISE_Y_SCALE_FACTOR,
                state.coordinate_set.t as f64 * noise.t_scale.into_inner() as f64 * NOISE_T_SCALE_FACTOR,
            ]) as f32),
            WorleyNoise { noise } => SNFloat::new(
                noise
                    .noise
                    .get([
                        state.coordinate_set.x as f64
                            * noise.x_scale.into_inner().powf(2.0) as f64
                            * NOISE_X_SCALE_FACTOR,
                        state.coordinate_set.y as f64
                            * noise.y_scale.into_inner().powf(2.0) as f64
                            * NOISE_Y_SCALE_FACTOR,
                        state.coordinate_set.t as f64
                            * noise.t_scale.into_inner().powf(2.0) as f64
                            * NOISE_T_SCALE_FACTOR,
                    ])
                    .min(0.99) as f32,
            ),
            Multiply { child_a, child_b } => SNFloat::new(
                child_a.compute(state).into_inner() * child_b.compute(state).into_inner(),
            ),
            Abs { child } => SNFloat::new(child.compute(state).into_inner().abs()),
            XRatio => {
                let width = state.cell_array.dim().0 as f32;

                SNFloat::new_from_range(state.coordinate_set.x as f32, 0.0, width)
            }
            YRatio => {
                let height = state.cell_array.dim().1 as f32;

                SNFloat::new_from_range(state.coordinate_set.y as f32, 0.0, height)
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

#[derive(Generatable, Mutatable, Debug)]
#[mutagen(mut_reroll = 0.1)]
pub enum UNFloatNodes {
    Random,
    Constant {
        value: UNFloat,
    },
    FromAngle {
        child: Box<AngleNodes>,
    },
    FromSNFloat {
        child: Box<SNFloatNodes>,
    },
    AbsSNFloat {
        child: Box<SNFloatNodes>,
    },
    SquareSNFloat {
        child: Box<SNFloatNodes>,
    },
    Multiply {
        child_a: Box<UNFloatNodes>,
        child_b: Box<UNFloatNodes>,
    },
    CircularAdd {
        child_a: Box<UNFloatNodes>,
        child_b: Box<UNFloatNodes>,
    },
    InvertNormalised {
        child: Box<UNFloatNodes>,
    },
    ColorAverage {
        child: Box<FloatColorNodes>,
    },
    ColorComponentR {
        child: Box<FloatColorNodes>,
    },
    ColorComponentG {
        child: Box<FloatColorNodes>,
    },
    ColorComponentB {
        child: Box<FloatColorNodes>,
    },
    XRatio,
    YRatio,
    // CoordinateTranslateX {
    //     child: Box<UNFloatNodes>,
    //     x: Box<SNFloatNodes>,
    // },
    // CoordinateTranslateY {
    //     child: Box<UNFloatNodes>,
    //     y: Box<SNFloatNodes>,
    // },
    // CoordinateTranslateT {
    //     child: Box<UNFloatNodes>,
    //     t: Box<SNFloatNodes>,
    // },
    // CoordinateScaleX {
    //     child: Box<UNFloatNodes>,
    //     x: Box<UNFloatNodes>,
    // },
    // CoordinateScaleY {
    //     child: Box<UNFloatNodes>,
    //     y: Box<UNFloatNodes>,
    // },
    // CoordinateScaleT {
    //     child: Box<UNFloatNodes>,
    //     t: Box<UNFloatNodes>,
    // },
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
            XRatio => {
                let width = state.cell_array.dim().0 as f32;

                UNFloat::new(state.coordinate_set.x as f32 / width)
            }
            YRatio => {
                let height = state.cell_array.dim().1 as f32;

                UNFloat::new(state.coordinate_set.y as f32 / height)
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

// pub enum UnsignedIntNodes {
//     Constant,
//     Random,
//     CurrentTic,
// }

// pub enum CoordinateTranslationNodes {
//     ShiftBy,
// }

#[derive(Generatable, Mutatable, Debug)]
#[mutagen(mut_reroll = 0.1)]
pub enum BooleanNodes {
    UNFloatLess {
        child_a: Box<UNFloatNodes>,
        child_b: Box<UNFloatNodes>,
    },
    UNFloatMore {
        child_a: Box<UNFloatNodes>,
        child_b: Box<UNFloatNodes>,
    },
    And {
        child_a: Box<BooleanNodes>,
        child_b: Box<BooleanNodes>,
    },
    Or {
        child_a: Box<BooleanNodes>,
        child_b: Box<BooleanNodes>,
    },
    //Xor {child_a: Boolean, child_b: Boolean},
    Not {
        child: Box<BooleanNodes>,
    },
    Constant {
        child: Boolean,
    },
    Random,
    // CoordinateTranslateX {
    //     child: Box<BooleanNodes>,
    //     x: Box<SNFloatNodes>,
    // },
    // CoordinateTranslateY {
    //     child: Box<BooleanNodes>,
    //     y: Box<SNFloatNodes>,
    // },
    // CoordinateTranslateT {
    //     child: Box<BooleanNodes>,
    //     t: Box<SNFloatNodes>,
    // },
    // CoordinateScaleX {
    //     child: Box<BooleanNodes>,
    //     x: Box<UNFloatNodes>,
    // },
    // CoordinateScaleY {
    //     child: Box<BooleanNodes>,
    //     y: Box<UNFloatNodes>,
    // },
    // CoordinateScaleT {
    //     child: Box<BooleanNodes>,
    //     t: Box<UNFloatNodes>,
    // },
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
            Random => Boolean::generate(),
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
