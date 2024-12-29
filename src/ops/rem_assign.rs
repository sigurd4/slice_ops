use core::ops::RemAssign;

use slice_trait::Slice;

use super::SliceVisit;

#[const_trait]
pub trait SliceRemAssign<T>: Slice<Item = T>
{
    /// Replaces each value in the slice with its remainder when divided by `rhs`.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use slice_ops::ops::*;
    /// 
    /// let mut x = [1, 2, 3, 4, 5, 6, 7, 8];
    /// 
    /// x.rem_assign_all(2);
    ///    
    /// assert_eq!(x, [1, 0, 1, 0, 1, 0, 1, 0]);
    /// ```
    fn rem_assign_all<Rhs>(&mut self, rhs: Rhs)
    where
        T: RemAssign<Rhs>,
        Rhs: Copy;
        
    /// Asynchronously replaces each value in the slice with its remainder when divided by `rhs`.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use slice_ops::ops::*;
    /// 
    /// # tokio_test::block_on(async {
    /// let mut x = [1, 2, 3, 4, 5, 6, 7, 8];
    /// 
    /// x.rem_assign_all_async(2).await;
    ///    
    /// assert_eq!(x, [1, 0, 1, 0, 1, 0, 1, 0]);
    /// # });
    /// ```
    #[cfg(feature = "alloc")]
    async fn rem_assign_all_async<Rhs>(&mut self, rhs: Rhs)
    where
        T: RemAssign<Rhs>,
        Rhs: Copy;
}

impl<T> SliceRemAssign<T> for [T]
{
    fn rem_assign_all<Rhs>(&mut self, rhs: Rhs)
    where
        T: RemAssign<Rhs>,
        Rhs: Copy
    {
        self.visit_mut(|x| *x %= rhs)
    }

    #[cfg(feature = "alloc")]
    async fn rem_assign_all_async<Rhs>(&mut self, rhs: Rhs)
    where
        T: RemAssign<Rhs>,
        Rhs: Copy
    {
        self.visit_mut_async(async |x| *x %= rhs).await
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