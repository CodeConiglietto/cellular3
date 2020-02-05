use noise::{
    BasicMulti, Billow, Checkerboard, Fbm, HybridMulti, OpenSimplex, RangeFunction, RidgedMulti,
    SuperSimplex, Value, Worley,
};
use mutagen::{Generatable, Mutatable};
use rand::prelude::*;

#[derive(Clone, Debug)]
pub struct BasicMultiFractalNoise(pub BasicMulti);
impl Generatable for BasicMultiFractalNoise {
    fn generate_rng<R: Rng + ?Sized>(rng: &mut R) -> Self{
        Self(BasicMulti::new())
    }
}
impl Mutatable for BasicMultiFractalNoise {
    fn mutate_rng<R: Rng + ?Sized>(&mut self, rng: &mut R) {
        self.0 = BasicMulti::new();
    }
}

// pub struct BillowNoise(pub Billow);
// impl Generatable for BillowNoise {
//     fn generate_rng<R: Rng + ?Sized>(rng: &mut R) -> Self{
//         Self(Billow::new())
//     }
// }

// pub struct CheckerboardNoise(pub Checkerboard);
// impl Generatable for CheckerboardNoise {
//     fn generate_rng<R: Rng + ?Sized>(rng: &mut R) -> Self{
//         Self(Checkerboard::new())
//     }
// }

#[derive(Clone, Debug)]
pub struct FractalBrownianNoise(pub Fbm);
impl Generatable for FractalBrownianNoise {
    fn generate_rng<R: Rng + ?Sized>(rng: &mut R) -> Self{
        Self(Fbm::new())
    }
}
impl Mutatable for FractalBrownianNoise {
    fn mutate_rng<R: Rng + ?Sized>(&mut self, rng: &mut R) {
        self.0 = Fbm::new();
    }
}

// impl Mutatable for FractalBrownianNoise {
//     fn generate_rng<R: Rng + ?Sized>(rng: &mut R) -> Self{
//         Self(Fbm::new())
//     }
// }

// pub struct HybridMultiFractalNoise(pub HybridMulti);
// impl Generatable for HybridMultiFractalNoise {
//     fn generate_rng<R: Rng + ?Sized>(rng: &mut R) -> Self{
//         Self(HybridMulti::new())
//     }
// }

#[derive(Clone, Debug)]
pub struct OpenSimplexNoise(pub OpenSimplex);
impl Generatable for OpenSimplexNoise {
    fn generate_rng<R: Rng + ?Sized>(rng: &mut R) -> Self{
        Self(OpenSimplex::new())
    }
}
impl Mutatable for OpenSimplexNoise {
    fn mutate_rng<R: Rng + ?Sized>(&mut self, rng: &mut R) {
        self.0 = OpenSimplex::new();
    }
}

#[derive(Clone, Debug)]
pub struct RidgedMultiFractalNoise(pub RidgedMulti);
impl Generatable for RidgedMultiFractalNoise {
    fn generate_rng<R: Rng + ?Sized>(rng: &mut R) -> Self{
        Self(RidgedMulti::new())
    }
}
impl Mutatable for RidgedMultiFractalNoise {
    fn mutate_rng<R: Rng + ?Sized>(&mut self, rng: &mut R) {
        self.0 = RidgedMulti::new();
    }
}

// pub struct SuperSimplexNoise(pub SuperSimplex);
// impl Generatable for SuperSimplexNoise {
//     fn generate_rng<R: Rng + ?Sized>(rng: &mut R) -> Self{
//         Self(SuperSimplex::new())
//     }
// }

// pub struct ValueNoise(pub Value);
// impl Generatable for ValueNoise {
//     fn generate_rng<R: Rng + ?Sized>(rng: &mut R) -> Self{
//         Self(Value::new())
//     }
// }

#[derive(Clone, Debug)]
pub struct WorleyNoise(pub Worley);
impl Generatable for WorleyNoise {
    fn generate_rng<R: Rng + ?Sized>(rng: &mut R) -> Self{
        Self(Worley::new().enable_range(true).set_displacement(0.9).set_range_function(RangeFunction::Manhattan))
    }
}
impl Mutatable for WorleyNoise {
    fn mutate_rng<R: Rng + ?Sized>(&mut self, rng: &mut R) {
        match random::<i32>() % 5
        {
            0 => {self.0 = Worley::new();},
            1 => {self.0.enable_range(rng.gen::<bool>());},
            2 => {self.0.set_displacement(rng.gen::<f64>());},
            3 => {self.0.set_range_function(
                    match rng.gen::<i32>() % 5 {
                        0 => RangeFunction::Euclidean,
                        1 => RangeFunction::EuclideanSquared,
                        2 => RangeFunction::Manhattan,
                        3 => RangeFunction::Chebyshev,
                        4 => RangeFunction::Quadratic,
                        _ => panic!(),
                    }
                );},
            4 => {self.0.set_frequency(rng.gen::<f64>());},
            _ => panic!(),
        };
    }
}