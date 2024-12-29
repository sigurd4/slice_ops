use core::ops::SubAssign;

use slice_trait::Slice;

#[const_trait]
pub trait SliceDifferentiate<T>: Slice<Item = T>
{
    /// Differentiates the slice in-place.
    /// 
    /// Each value will be subtracted by the previous value.
    /// 
    /// It's assumed that the first value of the slice is preceded by zeros.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use slice_ops::ops::*;
    /// 
    /// let mut x = [1, 5, 5, 6, 2, -1, 0, 0, 0];
    /// 
    /// x.differentiate();
    /// 
    /// assert_eq!(x, [1, 4, 0, 1, -4, -3, 1, 0, 0]);
    /// 
    /// x.integrate();
    /// 
    /// assert_eq!(x, [1, 5, 5, 6, 2, -1, 0, 0, 0]);
    /// ```
    fn differentiate(&mut self)
    where
        T: SubAssign<T> + Copy;
}

impl<T> SliceDifferentiate<T> for [T]
{
    fn differentiate(&mut self)
    where
        T: SubAssign<T> + Copy
    {
        let len = self.len();
        if len > 0
        {
            let mut i = len - 1;
            while i > 0
            {
                self[i] -= self[i - 1];
                i -= 1;
            }
        }
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