use core::ops::Mul;

pub trait Square: Mul + Copy
{
    type Output: Sized;

    fn square(&self) -> <Self as Square>::Output;
}

impl<T> Square for T
where
    T: Mul + Copy
{
    default type Output = <T as Mul>::Output;

    default fn square(&self) -> <Self as Square>::Output
    {
        unsafe {
            core::intrinsics::transmute_unchecked(*self**self)
        }
    }
}

#[cfg(feature = "num")]
impl<T> Square for T
where
    T: num_complex::ComplexFloat
{
    type Output = T::Real;

    fn square(&self) -> <Self as Square>::Output
    {
        let re = self.re();
        let im = self.im();
        re*re + im*im
    }
}