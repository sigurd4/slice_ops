use core::ops::Mul;

pub trait Square: Mul + Copy
{
    type Output: Sized + From<<Self as Mul>::Output>;

    fn square(&self) -> <Self as Square>::Output;
}

impl<T> Square for T
where
    T: Mul + Copy
{
    default type Output = <T as Mul>::Output;

    default fn square(&self) -> <Self as Square>::Output
    {
        (*self**self).into()
    }
}

#[cfg(feature = "num")]
impl<T> Square for T
where
    T: num_complex::ComplexFloat<Real: From<T>>
{
    type Output = T::Real;

    fn square(&self) -> <Self as Square>::Output
    {
        (*self*self.conj()).into()
    }
}