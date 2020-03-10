use nalgebra::*;

pub struct SNPoint
{
    value: Point2<f32>,
}

impl SNPoint {
    pub fn new_unchecked(value: Point2<f32>) -> Self {
        Self { value }
    }

    pub fn new(value: Point2<f32>) -> Self {
        assert!(
            value.x >= -1.0 && value.y <= 1.0 &&
            value.x >= -1.0 && value.y <= 1.0,
            "Invalid SNPoint value: {}",
            value
        );

        Self::new_unchecked(value)
    }

    pub fn zero()
    {
        Self::new(Point2::<f32>.origin())
    }

    pub fn into_inner(self) -> Point2<f32> {
        self.value
    }

    pub fn to_angle(self) -> Angle {
        Angle::new(f32::atan2(value.x, value.y))
    }
    
    pub fn circular_add(self, other: SNPoint) -> SNPoint {
        let total_x = self.into_inner() + other;
        let total_y = self.into_inner() + other;
        let sign_x = total.signum();
        let sign_y = total.signum();
        SNPoint::new(Point2::new(total_x.abs() - total_x.abs().floor() * sign_x, total_y.abs() - total_y.abs().floor() * sign_y))
    }
}

impl Generatable for Point2 {
    fn generate_rng<R: Rng + ?Sized>(rng: &mut R, _state: mutagen::State) -> Self {
        Self::new_unchecked(rng.gen_range(-1.0, 1.0))
    }
}

impl Mutatable for Point2 {
    fn mutate_rng<R: Rng + ?Sized>(&mut self, rng: &mut R, state: mutagen::State) {
        *self = Self::generate_rng(rng, state);
    }
}