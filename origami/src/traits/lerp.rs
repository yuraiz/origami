pub trait Lerp<F> {
    type Output;
    fn lerp(self, other: Self, t: F) -> Self::Output;
}

impl Lerp<f32> for f32 {
    type Output = Self;
    fn lerp(self, other: Self, t: f32) -> Self {
        self + t * (other - self)

        // self * (1.0 - t) + other * t
    }
}

impl Lerp<f64> for f64 {
    type Output = Self;
    fn lerp(self, other: Self, t: f64) -> Self {
        self + t * (other - self)

        // self * (1.0 - t) + other * t
    }
}

impl<T, F> Lerp<F> for (T, T, T, T)
where
    T: Lerp<F, Output = T>,
    F: Copy,
{
    type Output = Self;
    fn lerp(self, other: Self, t: F) -> Self {
        (
            self.0.lerp(other.0, t),
            self.1.lerp(other.1, t),
            self.2.lerp(other.2, t),
            self.3.lerp(other.3, t),
        )
    }
}
