
use crate::{
    datatype::{colors::*, continuous::*, points::*},
    node::{
        color_nodes::*, coord_map_nodes::*, discrete_nodes::*, mutagen_functions::*,
        noise_nodes::*, Node,
    },
    updatestate::*,
};
use mutagen::{Generatable, Mutatable};

//Note: SNPoints are not normalised in the matematical sense, each coordinate is simply capped at -1..1
#[derive(Generatable, Mutatable, Debug)]
#[mutagen(mut_reroll = 0.1)]
pub enum SNPointNodes
{
    Zero,
    Constant { value: SNPoint },
    FromSNFloats { child_a: Box<SNFloatNodes>, child_b: Box<SNFloatNodes> },
}

impl Node for SNPointNodes
{
    type Output = SNPoint;

    fn compute(&self, state: UpdateState) -> Self::Output {
        use SNPointNodes::*;
        
        match self{
            Zero => Point::zero(),
            Constant {value} => value,
            FromSNFloats {child_a, child_b} => Point::new(Point2::new(child_a.compute(state).into_inner(), child_b.compute(state).into_inner())),
        }
    }
}