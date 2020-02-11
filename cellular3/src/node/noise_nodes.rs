use crate::{
    constants::*,
    datatype::{continuous::*, noisefunctions::*},
    node::{mutagen_functions::*, Node},
    updatestate::*,
};
use mutagen::{Generatable, Mutatable};
use noise::NoiseFn;

#[derive(Mutatable, Generatable, Debug)]
pub enum NoiseNodes {
    #[mutagen(gen_weight = leaf_node_weight)]
    BasicMultiFractalNoise { noise: Box<BasicMultiFractalNoise> },

    #[mutagen(gen_weight = leaf_node_weight)]
    BillowNoise { noise: Box<BasicMultiFractalNoise> },

    #[mutagen(gen_weight = leaf_node_weight)]
    CheckerboardNoise { noise: Box<BasicMultiFractalNoise> },

    #[mutagen(gen_weight = leaf_node_weight)]
    FractalBrownianNoise { noise: Box<FractalBrownianNoise> },

    #[mutagen(gen_weight = leaf_node_weight)]
    HybridMultiFractalNoise { noise: Box<BasicMultiFractalNoise> },

    #[mutagen(gen_weight = leaf_node_weight)]
    OpenSimplexNoise { noise: Box<OpenSimplexNoise> },

    #[mutagen(gen_weight = leaf_node_weight)]
    RidgedMultiFractalNoise { noise: Box<RidgedMultiFractalNoise> },

    #[mutagen(gen_weight = leaf_node_weight)]
    SuperSimplexNoise { noise: Box<SuperSimplexNoise> },

    #[mutagen(gen_weight = leaf_node_weight)]
    ValueNoise { noise: Box<RidgedMultiFractalNoise> },

    #[mutagen(gen_weight = leaf_node_weight)]
    WorleyNoise { noise: Box<WorleyNoise> },
}

impl Node for NoiseNodes {
    type Output = SNFloat;

    fn compute(&self, state: UpdateState) -> Self::Output {
        use NoiseNodes::*;

        match self {
            BasicMultiFractalNoise { noise } => SNFloat::new(noise.noise.get([
                state.coordinate_set.x.into_inner() as f64
                    * noise.x_scale.into_inner().powf(2.0) as f64
                    * CONSTS.noise_x_scale_factor,
                state.coordinate_set.y.into_inner() as f64
                    * noise.y_scale.into_inner().powf(2.0) as f64
                    * CONSTS.noise_y_scale_factor,
                state.coordinate_set.t as f64
                    * noise.t_scale.into_inner() as f64
                    * CONSTS.noise_t_scale_factor,
            ]) as f32),
            BillowNoise { noise } => SNFloat::new(noise.noise.get([
                state.coordinate_set.x.into_inner() as f64
                    * noise.x_scale.into_inner().powf(2.0) as f64
                    * CONSTS.noise_x_scale_factor,
                state.coordinate_set.y.into_inner() as f64
                    * noise.y_scale.into_inner().powf(2.0) as f64
                    * CONSTS.noise_y_scale_factor,
                state.coordinate_set.t as f64
                    * noise.t_scale.into_inner() as f64
                    * CONSTS.noise_t_scale_factor,
            ]) as f32),
            CheckerboardNoise { noise } => SNFloat::new(noise.noise.get([
                state.coordinate_set.x.into_inner() as f64
                    * noise.x_scale.into_inner().powf(2.0) as f64
                    * CONSTS.noise_x_scale_factor,
                state.coordinate_set.y.into_inner() as f64
                    * noise.y_scale.into_inner().powf(2.0) as f64
                    * CONSTS.noise_y_scale_factor,
                state.coordinate_set.t as f64
                    * noise.t_scale.into_inner() as f64
                    * CONSTS.noise_t_scale_factor,
            ]) as f32),
            FractalBrownianNoise { noise } => SNFloat::new(noise.noise.get([
                state.coordinate_set.x.into_inner() as f64
                    * noise.x_scale.into_inner().powf(2.0) as f64
                    * CONSTS.noise_x_scale_factor,
                state.coordinate_set.y.into_inner() as f64
                    * noise.y_scale.into_inner().powf(2.0) as f64
                    * CONSTS.noise_y_scale_factor,
                state.coordinate_set.t as f64
                    * noise.t_scale.into_inner() as f64
                    * CONSTS.noise_t_scale_factor,
            ]) as f32),
            HybridMultiFractalNoise { noise } => SNFloat::new(noise.noise.get([
                state.coordinate_set.x.into_inner() as f64
                    * noise.x_scale.into_inner().powf(2.0) as f64
                    * CONSTS.noise_x_scale_factor,
                state.coordinate_set.y.into_inner() as f64
                    * noise.y_scale.into_inner().powf(2.0) as f64
                    * CONSTS.noise_y_scale_factor,
                state.coordinate_set.t as f64
                    * noise.t_scale.into_inner() as f64
                    * CONSTS.noise_t_scale_factor,
            ]) as f32),
            OpenSimplexNoise { noise } => SNFloat::new(noise.noise.get([
                state.coordinate_set.x.into_inner() as f64
                    * noise.x_scale.into_inner().powf(2.0) as f64
                    * CONSTS.noise_x_scale_factor,
                state.coordinate_set.y.into_inner() as f64
                    * noise.y_scale.into_inner().powf(2.0) as f64
                    * CONSTS.noise_y_scale_factor,
                state.coordinate_set.t as f64
                    * noise.t_scale.into_inner() as f64
                    * CONSTS.noise_t_scale_factor,
            ]) as f32),
            RidgedMultiFractalNoise { noise } => SNFloat::new(noise.noise.get([
                state.coordinate_set.x.into_inner() as f64
                    * noise.x_scale.into_inner().powf(2.0) as f64
                    * CONSTS.noise_x_scale_factor,
                state.coordinate_set.y.into_inner() as f64
                    * noise.y_scale.into_inner().powf(2.0) as f64
                    * CONSTS.noise_y_scale_factor,
                state.coordinate_set.t as f64
                    * noise.t_scale.into_inner() as f64
                    * CONSTS.noise_t_scale_factor,
            ]) as f32),
            SuperSimplexNoise { noise } => SNFloat::new(noise.noise.get([
                state.coordinate_set.x.into_inner() as f64
                    * noise.x_scale.into_inner().powf(2.0) as f64
                    * CONSTS.noise_x_scale_factor,
                state.coordinate_set.y.into_inner() as f64
                    * noise.y_scale.into_inner().powf(2.0) as f64
                    * CONSTS.noise_y_scale_factor,
                state.coordinate_set.t as f64
                    * noise.t_scale.into_inner() as f64
                    * CONSTS.noise_t_scale_factor,
            ]) as f32),
            ValueNoise { noise } => SNFloat::new(noise.noise.get([
                state.coordinate_set.x.into_inner() as f64
                    * noise.x_scale.into_inner().powf(2.0) as f64
                    * CONSTS.noise_x_scale_factor,
                state.coordinate_set.y.into_inner() as f64
                    * noise.y_scale.into_inner().powf(2.0) as f64
                    * CONSTS.noise_y_scale_factor,
                state.coordinate_set.t as f64
                    * noise.t_scale.into_inner() as f64
                    * CONSTS.noise_t_scale_factor,
            ]) as f32),
            WorleyNoise { noise } => SNFloat::new(
                noise
                    .noise
                    .get([
                        state.coordinate_set.x.into_inner() as f64
                            * noise.x_scale.into_inner().powf(2.0) as f64
                            * CONSTS.noise_x_scale_factor,
                        state.coordinate_set.y.into_inner() as f64
                            * noise.y_scale.into_inner().powf(2.0) as f64
                            * CONSTS.noise_y_scale_factor,
                        state.coordinate_set.t as f64
                            * noise.t_scale.into_inner().powf(2.0) as f64
                            * CONSTS.noise_t_scale_factor,
                    ])
                    .min(0.99) as f32,
            ),
        }
    }
}
