use core::ops::{Div, DivAssign};

use slice_trait::Slice;

use super::SliceVisit;

#[const_trait]
pub trait SliceDivAssign<T>: Slice<Item = T>
{
    /// Divides each element in the slice by `rhs`.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use slice_ops::*;
    /// 
    /// let mut x = [1, 2, 3, 4, 5, 6, 7, 8];
    /// 
    /// x.div_assign_all(2);
    ///    
    /// assert_eq!(x, [0, 1, 1, 2, 2, 3, 3, 4]);
    /// ```
    fn div_assign_all<Rhs>(&mut self, rhs: Rhs)
    where
        T: DivAssign<Rhs>,
        Rhs: Copy;
        
    /// Asynchronously divides each element in the slice by `rhs`.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use slice_ops::*;
    /// 
    /// # tokio_test::block_on(async {
    /// let mut x = [1, 2, 3, 4, 5, 6, 7, 8];
    /// 
    /// x.div_assign_all_async(2).await;
    ///    
    /// assert_eq!(x, [0, 1, 1, 2, 2, 3, 3, 4]);
    /// # });
    /// ```
    #[cfg(feature = "alloc")]
    async fn div_assign_all_async<Rhs>(&mut self, rhs: Rhs)
    where
        T: DivAssign<Rhs>,
        Rhs: Copy;
        
    /// TODO
    fn rdiv_assign_all<Lhs>(&mut self, lhs: Lhs)
    where
        Lhs: Copy + Div<T, Output = T>;
    
    /// TODO
    #[cfg(feature = "alloc")]
    async fn rdiv_assign_all_async<Lhs>(&mut self, lhs: Lhs)
    where
        Lhs: Copy + Div<T, Output = T>;
}

impl<T> SliceDivAssign<T> for [T]
{
    fn div_assign_all<Rhs>(&mut self, rhs: Rhs)
    where
        T: DivAssign<Rhs>,
        Rhs: Copy
    {
        self.visit_mut(|x| *x /= rhs)
    }

    #[cfg(feature = "alloc")]
    async fn div_assign_all_async<Rhs>(&mut self, rhs: Rhs)
    where
        T: DivAssign<Rhs>,
        Rhs: Copy
    {
        self.visit_mut_async(async |x| *x /= rhs).await
    }

    fn rdiv_assign_all<Lhs>(&mut self, lhs: Lhs)
    where
        Lhs: Copy + Div<T, Output = T>
    {
        self.visit_mut(|x| unsafe {
            core::ptr::write(x, lhs / core::ptr::read(x))
        })
    }

    #[cfg(feature = "alloc")]
    async fn rdiv_assign_all_async<Lhs>(&mut self, lhs: Lhs)
    where
        Lhs: Copy + Div<T, Output = T>
    {
        self.visit_mut_async(async |x| unsafe {
            core::ptr::write(x, lhs / core::ptr::read(x))
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