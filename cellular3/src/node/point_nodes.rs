use crate::{
    datatype::{points::*},
    node::{
        continuous_nodes::*, Node, mutagen_functions::*
    },
    updatestate::*,
};
use mutagen::{Generatable, Mutatable};
use nalgebra::*;
//Note: SNPoints are not normalised in the matematical sense, each coordinate is simply capped at -1..1
#[derive(Generatable, Mutatable, Debug)]
#[mutagen(mut_reroll = 0.1)]
pub enum SNPointNodes {
    #[mutagen(gen_weight = leaf_node_weight)]
    Zero,
    #[mutagen(gen_weight = leaf_node_weight)]
    Constant {
        value: SNPoint,
    },
    #[mutagen(gen_weight = pipe_node_weight)]
    Invert {
        child: Box<SNPointNodes>,
    },
    #[mutagen(gen_weight = branch_node_weight)]
    FromSNFloats {
        child_a: Box<SNFloatNodes>,
        child_b: Box<SNFloatNodes>,
    },
}

impl Node for SNPointNodes {
    type Output = SNPoint;

    fn compute(&self, state: UpdateState) -> Self::Output {
        use SNPointNodes::*;

        match self {
            Zero => SNPoint::zero(),
            Constant { value } => *value,
            Invert { child } => {
                let point = child.compute(state).into_inner();
                SNPoint::new(Point2::new(point.x * -1.0, point.y * -1.0))
            },
            FromSNFloats { child_a, child_b } => SNPoint::new(Point2::new(
                child_a.compute(state).into_inner(),
                child_b.compute(state).into_inner(),
            )),
        }
    }
}
