use core::ops::MulAssign;

use slice_trait::Slice;

use super::SliceVisit;

#[const_trait]
pub trait SliceMulAssign<T>: Slice<Item = T>
{
    /// Multiplies `rhs` to each element in the slice.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use slice_ops::ops::*;
    /// 
    /// let mut x = [1, 2, 3, 4, 5, 6, 7, 8];
    /// 
    /// x.mul_assign_all(2);
    ///    
    /// assert_eq!(x, [2, 4, 6, 8, 10, 12, 14, 16]);
    /// ```
    fn mul_assign_all<Rhs>(&mut self, rhs: Rhs)
    where
        T: MulAssign<Rhs>,
        Rhs: Copy;

    /// Asynchronously multiplies `rhs` to each element in the slice.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use slice_ops::ops::*;
    /// 
    /// # tokio_test::block_on(async {
    /// let mut x = [1, 2, 3, 4, 5, 6, 7, 8];
    /// 
    /// x.mul_assign_all_async(2).await;
    ///    
    /// assert_eq!(x, [2, 4, 6, 8, 10, 12, 14, 16]);
    /// # });
    /// ```
    #[cfg(feature = "alloc")]
    async fn mul_assign_all_async<Rhs>(&mut self, rhs: Rhs)
    where
        T: MulAssign<Rhs>,
        Rhs: Copy;
}

impl<T> SliceMulAssign<T> for [T]
{
    fn mul_assign_all<Rhs>(&mut self, rhs: Rhs)
    where
        T: MulAssign<Rhs>,
        Rhs: Copy
    {
        self.visit_mut(|x| *x *= rhs)
    }

    #[cfg(feature = "alloc")]
    async fn mul_assign_all_async<Rhs>(&mut self, rhs: Rhs)
    where
        T: MulAssign<Rhs>,
        Rhs: Copy
    {
        self.visit_mut_async(async |x| *x *= rhs).await
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