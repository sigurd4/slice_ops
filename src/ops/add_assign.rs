use core::ops::AddAssign;

use slice_trait::Slice;

use super::SliceVisit;

#[const_trait]
pub trait SliceAddAssign<T>: Slice<Item = T>
{
    /// Adds `rhs` to each element in the slice.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use slice_ops::ops::*;
    /// 
    /// let mut x = [1, 2, 3, 4, 5, 6, 7, 8];
    /// 
    /// x.add_assign_all(2);
    ///    
    /// assert_eq!(x, [3, 4, 5, 6, 7, 8, 9, 10]);
    /// ```
    fn add_assign_all<Rhs>(&mut self, rhs: Rhs)
    where
        T: AddAssign<Rhs>,
        Rhs: Copy;
        
    /// Asynchronously adds `rhs` to each element in the slice.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use slice_ops::ops::*;
    /// 
    /// # tokio_test::block_on(async {
    /// let mut x = [1, 2, 3, 4, 5, 6, 7, 8];
    /// 
    /// x.add_assign_all_async(2).await;
    ///    
    /// assert_eq!(x, [3, 4, 5, 6, 7, 8, 9, 10]);
    /// # });
    /// ```
    #[cfg(feature = "alloc")]
    async fn add_assign_all_async<Rhs>(&mut self, rhs: Rhs)
    where
        T: AddAssign<Rhs>,
        Rhs: Copy;
}

impl<T> SliceAddAssign<T> for [T]
{
    fn add_assign_all<Rhs>(&mut self, rhs: Rhs)
    where
        T: AddAssign<Rhs>,
        Rhs: Copy
    {
        self.visit_mut(|x| *x += rhs)
    }

    #[cfg(feature = "alloc")]
    async fn add_assign_all_async<Rhs>(&mut self, rhs: Rhs)
    where
        T: AddAssign<Rhs>,
        Rhs: Copy
    {
        self.visit_mut_async(async |x| *x += rhs).await
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