use crate::datatype::primitives::UNFloat;
use mutagen::{Generatable, Mutatable};
use noise::{
    BasicMulti, Billow, Checkerboard, Fbm, HybridMulti, OpenSimplex, RangeFunction, RidgedMulti,
    SuperSimplex, Value, Worley,
};
use rand::prelude::*;

#[derive(Clone, Debug)]
pub struct BasicMultiFractalNoise {
    pub x_scale: UNFloat,
    pub y_scale: UNFloat,
    pub t_scale: UNFloat,
    pub noise: BasicMulti,
}
impl Generatable for BasicMultiFractalNoise {
    fn generate_rng<R: Rng + ?Sized>(rng: &mut R) -> Self {
        Self {
            x_scale: UNFloat::generate_rng(rng),
            y_scale: UNFloat::generate_rng(rng),
            t_scale: UNFloat::generate_rng(rng),
            noise: BasicMulti::new(),
        }
    }
}
impl Mutatable for BasicMultiFractalNoise {
    fn mutate_rng<R: Rng + ?Sized>(&mut self, rng: &mut R) {
        match random::<u32>() % 4 {
            0 => {
                self.x_scale = UNFloat::generate_rng(rng);
            }
            1 => {
                self.y_scale = UNFloat::generate_rng(rng);
            }
            2 => {
                self.t_scale = UNFloat::generate_rng(rng);
            }
            3 => {
                self.noise = BasicMulti::new();
            }
            _ => panic!(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct BillowNoise {
    pub x_scale: UNFloat,
    pub y_scale: UNFloat,
    pub t_scale: UNFloat,
    pub noise: Billow,
}
impl Generatable for BillowNoise {
    fn generate_rng<R: Rng + ?Sized>(rng: &mut R) -> Self {
        Self {
            x_scale: UNFloat::generate_rng(rng),
            y_scale: UNFloat::generate_rng(rng),
            t_scale: UNFloat::generate_rng(rng),
            noise: Billow::new(),
        }
    }
}
impl Mutatable for BillowNoise {
    fn mutate_rng<R: Rng + ?Sized>(&mut self, rng: &mut R) {
        match random::<u32>() % 4 {
            0 => {
                self.x_scale = UNFloat::generate_rng(rng);
            }
            1 => {
                self.y_scale = UNFloat::generate_rng(rng);
            }
            2 => {
                self.t_scale = UNFloat::generate_rng(rng);
            }
            3 => {
                self.noise = Billow::new();
            }
            _ => panic!(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct CheckerboardNoise {
    pub x_scale: UNFloat,
    pub y_scale: UNFloat,
    pub t_scale: UNFloat,
    pub noise: Checkerboard,
}
impl Generatable for CheckerboardNoise {
    fn generate_rng<R: Rng + ?Sized>(rng: &mut R) -> Self {
        Self {
            x_scale: UNFloat::generate_rng(rng),
            y_scale: UNFloat::generate_rng(rng),
            t_scale: UNFloat::generate_rng(rng),
            noise: Checkerboard::new(),
        }
    }
}
impl Mutatable for CheckerboardNoise {
    fn mutate_rng<R: Rng + ?Sized>(&mut self, rng: &mut R) {
        match random::<u32>() % 4 {
            0 => {
                self.x_scale = UNFloat::generate_rng(rng);
            }
            1 => {
                self.y_scale = UNFloat::generate_rng(rng);
            }
            2 => {
                self.t_scale = UNFloat::generate_rng(rng);
            }
            3 => {
                self.noise = Checkerboard::new();
            }
            _ => panic!(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct FractalBrownianNoise {
    pub x_scale: UNFloat,
    pub y_scale: UNFloat,
    pub t_scale: UNFloat,
    pub noise: Fbm,
}
impl Generatable for FractalBrownianNoise {
    fn generate_rng<R: Rng + ?Sized>(rng: &mut R) -> Self {
        Self {
            x_scale: UNFloat::generate_rng(rng),
            y_scale: UNFloat::generate_rng(rng),
            t_scale: UNFloat::generate_rng(rng),
            noise: Fbm::new(),
        }
    }
}
impl Mutatable for FractalBrownianNoise {
    fn mutate_rng<R: Rng + ?Sized>(&mut self, rng: &mut R) {
        match random::<u32>() % 4 {
            0 => {
                self.x_scale = UNFloat::generate_rng(rng);
            }
            1 => {
                self.y_scale = UNFloat::generate_rng(rng);
            }
            2 => {
                self.t_scale = UNFloat::generate_rng(rng);
            }
            3 => {
                self.noise = Fbm::new();
            }
            _ => panic!(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct HybridMultiFractalNoise {
    pub x_scale: UNFloat,
    pub y_scale: UNFloat,
    pub t_scale: UNFloat,
    pub noise: HybridMulti,
}
impl Generatable for HybridMultiFractalNoise {
    fn generate_rng<R: Rng + ?Sized>(rng: &mut R) -> Self {
        Self {
            x_scale: UNFloat::generate_rng(rng),
            y_scale: UNFloat::generate_rng(rng),
            t_scale: UNFloat::generate_rng(rng),
            noise: HybridMulti::new(),
        }
    }
}
impl Mutatable for HybridMultiFractalNoise {
    fn mutate_rng<R: Rng + ?Sized>(&mut self, rng: &mut R) {
        match random::<u32>() % 4 {
            0 => {
                self.x_scale = UNFloat::generate_rng(rng);
            }
            1 => {
                self.y_scale = UNFloat::generate_rng(rng);
            }
            2 => {
                self.t_scale = UNFloat::generate_rng(rng);
            }
            3 => {
                self.noise = HybridMulti::new();
            }
            _ => panic!(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct OpenSimplexNoise {
    pub x_scale: UNFloat,
    pub y_scale: UNFloat,
    pub t_scale: UNFloat,
    pub noise: OpenSimplex,
}
impl Generatable for OpenSimplexNoise {
    fn generate_rng<R: Rng + ?Sized>(rng: &mut R) -> Self {
        Self {
            x_scale: UNFloat::generate_rng(rng),
            y_scale: UNFloat::generate_rng(rng),
            t_scale: UNFloat::generate_rng(rng),
            noise: OpenSimplex::new(),
        }
    }
}
impl Mutatable for OpenSimplexNoise {
    fn mutate_rng<R: Rng + ?Sized>(&mut self, rng: &mut R) {
        match random::<u32>() % 4 {
            0 => {
                self.x_scale = UNFloat::generate_rng(rng);
            }
            1 => {
                self.y_scale = UNFloat::generate_rng(rng);
            }
            2 => {
                self.t_scale = UNFloat::generate_rng(rng);
            }
            3 => {
                self.noise = OpenSimplex::new();
            }
            _ => panic!(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct RidgedMultiFractalNoise {
    pub x_scale: UNFloat,
    pub y_scale: UNFloat,
    pub t_scale: UNFloat,
    pub noise: RidgedMulti,
}
impl Generatable for RidgedMultiFractalNoise {
    fn generate_rng<R: Rng + ?Sized>(rng: &mut R) -> Self {
        Self {
            x_scale: UNFloat::generate_rng(rng),
            y_scale: UNFloat::generate_rng(rng),
            t_scale: UNFloat::generate_rng(rng),
            noise: RidgedMulti::new(),
        }
    }
}
impl Mutatable for RidgedMultiFractalNoise {
    fn mutate_rng<R: Rng + ?Sized>(&mut self, rng: &mut R) {
        match random::<u32>() % 4 {
            0 => {
                self.x_scale = UNFloat::generate_rng(rng);
            }
            1 => {
                self.y_scale = UNFloat::generate_rng(rng);
            }
            2 => {
                self.t_scale = UNFloat::generate_rng(rng);
            }
            3 => {
                self.noise = RidgedMulti::new();
            }
            _ => panic!(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct SuperSimplexNoise {
    pub x_scale: UNFloat,
    pub y_scale: UNFloat,
    pub t_scale: UNFloat,
    pub noise: SuperSimplex,
}
impl Generatable for SuperSimplexNoise {
    fn generate_rng<R: Rng + ?Sized>(rng: &mut R) -> Self {
        Self {
            x_scale: UNFloat::generate_rng(rng),
            y_scale: UNFloat::generate_rng(rng),
            t_scale: UNFloat::generate_rng(rng),
            noise: SuperSimplex::new(),
        }
    }
}
impl Mutatable for SuperSimplexNoise {
    fn mutate_rng<R: Rng + ?Sized>(&mut self, rng: &mut R) {
        match random::<u32>() % 4 {
            0 => {
                self.x_scale = UNFloat::generate_rng(rng);
            }
            1 => {
                self.y_scale = UNFloat::generate_rng(rng);
            }
            2 => {
                self.t_scale = UNFloat::generate_rng(rng);
            }
            3 => {
                self.noise = SuperSimplex::new();
            }
            _ => panic!(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct ValueNoise {
    pub x_scale: UNFloat,
    pub y_scale: UNFloat,
    pub t_scale: UNFloat,
    pub noise: Value,
}
impl Generatable for ValueNoise {
    fn generate_rng<R: Rng + ?Sized>(rng: &mut R) -> Self {
        Self {
            x_scale: UNFloat::generate_rng(rng),
            y_scale: UNFloat::generate_rng(rng),
            t_scale: UNFloat::generate_rng(rng),
            noise: Value::new(),
        }
    }
}
impl Mutatable for ValueNoise {
    fn mutate_rng<R: Rng + ?Sized>(&mut self, rng: &mut R) {
        match random::<u32>() % 4 {
            0 => {
                self.x_scale = UNFloat::generate_rng(rng);
            }
            1 => {
                self.y_scale = UNFloat::generate_rng(rng);
            }
            2 => {
                self.t_scale = UNFloat::generate_rng(rng);
            }
            3 => {
                self.noise = Value::new();
            }
            _ => panic!(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct WorleyNoise {
    pub x_scale: UNFloat,
    pub y_scale: UNFloat,
    pub t_scale: UNFloat,
    pub noise: Worley,
}
impl Generatable for WorleyNoise {
    fn generate_rng<R: Rng + ?Sized>(rng: &mut R) -> Self {
        Self {
            x_scale: UNFloat::generate_rng(rng),
            y_scale: UNFloat::generate_rng(rng),
            t_scale: UNFloat::generate_rng(rng),
            noise: Worley::new()
                .enable_range(rng.gen::<bool>())
                .set_displacement(rng.gen::<f64>())
                .set_range_function(match rng.gen::<u32>() % 5 {
                    0 => RangeFunction::Euclidean,
                    1 => RangeFunction::EuclideanSquared,
                    2 => RangeFunction::Manhattan,
                    3 => RangeFunction::Chebyshev,
                    4 => RangeFunction::Quadratic,
                    _ => panic!(),
                }),
        }
    }
}
impl Mutatable for WorleyNoise {
    fn mutate_rng<R: Rng + ?Sized>(&mut self, rng: &mut R) {
        match random::<u32>() % 5 {
            0 => {
                self.x_scale = UNFloat::generate_rng(rng);
            }
            1 => {
                self.y_scale = UNFloat::generate_rng(rng);
            }
            2 => {
                self.t_scale = UNFloat::generate_rng(rng);
            }
            3 => {
                self.noise.enable_range(rng.gen::<bool>());
            }
            4 => {
                self.noise.set_displacement(rng.gen::<f64>());
            }
            5 => {
                self.noise.set_range_function(match rng.gen::<u32>() % 5 {
                    0 => RangeFunction::Euclidean,
                    1 => RangeFunction::EuclideanSquared,
                    2 => RangeFunction::Manhattan,
                    3 => RangeFunction::Chebyshev,
                    4 => RangeFunction::Quadratic,
                    _ => panic!(),
                });
            }
            _ => panic!(),
        };
    }
}
