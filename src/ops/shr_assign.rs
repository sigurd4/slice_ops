use core::ops::ShrAssign;

use slice_trait::Slice;

use super::SliceVisit;

#[const_trait]
pub trait SliceShrAssign<T>: Slice<Item = T>
{
    /// Shifts each element to the right by `rhs`.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use slice_ops::ops::*;
    /// 
    /// let mut x = [0b1, 0b10, 0b11, 0b100, 0b101, 0b110, 0b111, 0b1000];
    /// 
    /// x.shr_assign_all(2);
    ///    
    /// assert_eq!(x, [0b0, 0b0, 0b0, 0b1, 0b1, 0b1, 0b1, 0b10]);
    /// ```
    fn shr_assign_all<Rhs>(&mut self, rhs: Rhs)
    where
        T: ShrAssign<Rhs>,
        Rhs: Copy;
    
    /// Asynchronously shifts each element to the right by `rhs`.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use slice_ops::ops::*;
    /// 
    /// # tokio_test::block_on(async {
    /// let mut x = [0b1, 0b10, 0b11, 0b100, 0b101, 0b110, 0b111, 0b1000];
    /// 
    /// x.shr_assign_all_async(2).await;
    ///    
    /// assert_eq!(x, [0b0, 0b0, 0b0, 0b1, 0b1, 0b1, 0b1, 0b10]);
    /// # });
    /// ```
    #[cfg(feature = "alloc")]
    async fn shr_assign_all_async<Rhs>(&mut self, rhs: Rhs)
    where
        T: ShrAssign<Rhs>,
        Rhs: Copy;
}

impl<T> SliceShrAssign<T> for [T]
{
    fn shr_assign_all<Rhs>(&mut self, rhs: Rhs)
    where
        T: ShrAssign<Rhs>,
        Rhs: Copy
    {
        self.visit_mut(|x| *x >>= rhs)
    }

    #[cfg(feature = "alloc")]
    async fn shr_assign_all_async<Rhs>(&mut self, rhs: Rhs)
    where
        T: ShrAssign<Rhs>,
        Rhs: Copy
    {
        self.visit_mut_async(async |x| *x >>= rhs).await
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