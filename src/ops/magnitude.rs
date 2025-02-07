use core::ops::AddAssign;

use slice_trait::Slice;

use crate::spec::Square;

use super::SliceVisit;

#[const_trait]
pub trait SlicePartialMagnitude<T>: Slice<Item = T>
{
    /// Computes the square magnitude if the slice is not empty, otherwise returns [`None`].
    /// 
    /// This implementation is done without any knowledge of what the additive identity of `T` is.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use slice_ops::ops::*;
    /// 
    /// let mut x = [1, 2, 3, 4, 5];
    /// 
    /// let m = x.partial_magnitude_squared();
    /// 
    /// assert_eq!(m, Some(1*1 + 2*2 + 3*3 + 4*4 + 5*5));
    /// ```
    fn partial_magnitude_squared(&self) -> Option<<T as Square>::Output>
    where
        T: Square<Output: AddAssign>;

    /// Computes the square magnitude plus the value `from`.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use slice_ops::ops::*;
    /// 
    /// let mut x = [1, 2, 3, 4, 5];
    /// 
    /// let m = x.magnitude_squared_from(0);
    /// 
    /// assert_eq!(m, 1*1 + 2*2 + 3*3 + 4*4 + 5*5);
    /// ```
    fn magnitude_squared_from<O>(&self, from: O) -> O
    where
        T: Square,
        O: AddAssign<<T as Square>::Output>;
}

impl<T> SlicePartialMagnitude<T> for [T]
{
    fn partial_magnitude_squared(&self) -> Option<<T as Square>::Output>
    where
        T: Square<Output: AddAssign>
    {
        let n = self.len();
        if n == 0
        {
            return None
        }
        let mut y = self[0].square();
        let mut i = 1;
        while i < n
        {
            y += self[i].square();
            i += 1
        }
        Some(y)
    }

    fn magnitude_squared_from<O>(&self, mut from: O) -> O
    where
        T: Square,
        O: AddAssign<<T as Square>::Output>
    {
        self.visit(|x| from += x.square());
        from
    }
}

#[cfg(test)]
mod test
{
    #[test]
    fn it_works()
    {
        
    }
}