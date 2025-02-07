pub trait Conj: Sized
{
    fn conj(self) -> Self;
}

impl<T> Conj for T
{
    default fn conj(self) -> Self
    {
        self
    }
}

#[cfg(feature = "num")]
impl<T> Conj for T
where
    T: num_complex::ComplexFloat
{
    fn conj(self) -> Self
    {
        self.conj()
    }
}