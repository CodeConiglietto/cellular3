use noise::{
    BasicMulti, Billow, Checkerboard, Fbm, HybridMulti, OpenSimplex, RangeFunction, RidgedMulti,
    SuperSimplex, Value, Worley,
};
use mutagen::Generatable;
use rand::Rng;

pub struct BasicMultiFractalNoise(pub BasicMulti);
impl Generatable for BasicMultiFractalNoise {
    fn generate_rng<R: Rng + ?Sized>(rng: &mut R) -> Self{
        Self(BasicMulti::new())
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

pub struct FractalBrownianNoise(pub Fbm);
impl Generatable for FractalBrownianNoise {
    fn generate_rng<R: Rng + ?Sized>(rng: &mut R) -> Self{
        Self(Fbm::new())
    }
}

// pub struct HybridMultiFractalNoise(pub HybridMulti);
// impl Generatable for HybridMultiFractalNoise {
//     fn generate_rng<R: Rng + ?Sized>(rng: &mut R) -> Self{
//         Self(HybridMulti::new())
//     }
// }

pub struct OpenSimplexNoise(pub OpenSimplex);
impl Generatable for OpenSimplexNoise {
    fn generate_rng<R: Rng + ?Sized>(rng: &mut R) -> Self{
        Self(OpenSimplex::new())
    }
}

pub struct RidgedMultiFractalNoise(pub RidgedMulti);
impl Generatable for RidgedMultiFractalNoise {
    fn generate_rng<R: Rng + ?Sized>(rng: &mut R) -> Self{
        Self(RidgedMulti::new())
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

pub struct WorleyNoise(pub Worley);
impl Generatable for WorleyNoise {
    fn generate_rng<R: Rng + ?Sized>(rng: &mut R) -> Self{
        Self(Worley::new().enable_range(true).set_displacement(0.9).set_range_function(RangeFunction::Manhattan))
    }
}