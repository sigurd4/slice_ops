use core::ops::AddAssign;

use slice_trait::Slice;

#[const_trait]
pub trait SliceIntegrate<T>: Slice<Item = T>
{
    /// Integrates the slice in-place.
    /// 
    /// Each value will be added by the sum of all previous values.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use slice_ops::ops::*;
    /// 
    /// let mut x = [1, 5, 5, 6, 2, -1, 0, 0, 0];
    /// 
    /// x.integrate();
    /// 
    /// assert_eq!(x, [1, 6, 11, 17, 19, 18, 18, 18, 18]);
    /// 
    /// x.differentiate();
    /// 
    /// assert_eq!(x, [1, 5, 5, 6, 2, -1, 0, 0, 0]);
    /// ```
    fn integrate(&mut self)
    where
        T: AddAssign<T> + Copy;
}

impl<T> SliceIntegrate<T> for [T]
{
    fn integrate(&mut self)
    where
        T: AddAssign<T> + Copy
    {
        let len = self.len();
        let mut i = 1;
        while i < len
        {
            self[i] += self[i - 1];
            i += 1;
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