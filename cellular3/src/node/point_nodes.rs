pub enum SNPointNodes
{
    Zero,
    Constant { value: SNPoint },
    FromSNFloats { child_a: Box<SNFloatNodes>, child_b: Box<SNFloatNodes> },
    Iterative,
}