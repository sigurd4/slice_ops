use core::ops::{Sub, SubAssign};

use slice_trait::Slice;

use super::SliceVisit;

#[const_trait]
pub trait SliceSubAssign<T>: Slice<Item = T>
{
    /// Subtracts each element in the slice by `rhs`.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use slice_ops::ops::*;
    /// 
    /// let mut x = [1, 2, 3, 4, 5, 6, 7, 8];
    /// 
    /// x.sub_assign_all(2);
    ///    
    /// assert_eq!(x, [-1, 0, 1, 2, 3, 4, 5, 6]);
    /// ```
    fn sub_assign_all<Rhs>(&mut self, rhs: Rhs)
    where
        T: SubAssign<Rhs>,
        Rhs: Copy;
        
    /// Asynchronously subtracts each element in the slice by `rhs`.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use slice_ops::ops::*;
    /// 
    /// # tokio_test::block_on(async {
    /// let mut x = [1, 2, 3, 4, 5, 6, 7, 8];
    /// 
    /// x.sub_assign_all_async(2).await;
    ///    
    /// assert_eq!(x, [-1, 0, 1, 2, 3, 4, 5, 6]);
    /// # });
    /// ```
    #[cfg(feature = "alloc")]
    async fn sub_assign_all_async<Rhs>(&mut self, rhs: Rhs)
    where
        T: SubAssign<Rhs>,
        Rhs: Copy;

    /// TODO
    fn rsub_assign_all<Lhs>(&mut self, lhs: Lhs)
    where
        Lhs: Copy + Sub<T, Output = T>;

    /// TODO
    #[cfg(feature = "alloc")]
    async fn rsub_assign_all_async<Lhs>(&mut self, lhs: Lhs)
    where
        Lhs: Copy + Sub<T, Output = T>;
}

impl<T> SliceSubAssign<T> for [T]
{
    fn sub_assign_all<Rhs>(&mut self, rhs: Rhs)
    where
        T: SubAssign<Rhs>,
        Rhs: Copy
    {
        self.visit_mut(|x| *x -= rhs)
    }

    #[cfg(feature = "alloc")]
    async fn sub_assign_all_async<Rhs>(&mut self, rhs: Rhs)
    where
        T: SubAssign<Rhs>,
        Rhs: Copy
    {
        self.visit_mut_async(async |x| *x -= rhs).await
    }

    fn rsub_assign_all<Lhs>(&mut self, lhs: Lhs)
    where
        Lhs: Copy + Sub<T, Output = T>
    {
        self.visit_mut(|x| unsafe {
            core::ptr::write(x, lhs - core::ptr::read(x))
        })
    }
    
    #[cfg(feature = "alloc")]
    async fn rsub_assign_all_async<Lhs>(&mut self, lhs: Lhs)
    where
        Lhs: Copy + Sub<T, Output = T>
    {
        self.visit_mut_async(async |x| unsafe {
            core::ptr::write(x, lhs - core::ptr::read(x))
        }).await
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