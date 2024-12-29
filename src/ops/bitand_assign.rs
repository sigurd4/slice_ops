use core::ops::BitAndAssign;

use slice_trait::Slice;

use super::SliceVisit;

#[const_trait]
pub trait SliceBitAndAssign<T>: Slice<Item = T>
{
    /// Performs a bitwise AND on each element using `rhs`.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use slice_ops::*;
    /// 
    /// let mut x = [0b1, 0b10, 0b11, 0b100, 0b101, 0b110, 0b111, 0b1000];
    /// 
    /// x.bitand_assign_all(0b10);
    ///    
    /// assert_eq!(x, [0b0, 0b10, 0b10, 0b0, 0b0, 0b10, 0b10, 0b0]);
    /// ```
    fn bitand_assign_all<Rhs>(&mut self, rhs: Rhs)
    where
        T: BitAndAssign<Rhs>,
        Rhs: Copy;
        
    /// Asynchronously performs a bitwise AND on each element using `rhs`.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use slice_ops::*;
    /// 
    /// # tokio_test::block_on(async {
    /// let mut x = [0b1, 0b10, 0b11, 0b100, 0b101, 0b110, 0b111, 0b1000];
    /// 
    /// x.bitand_assign_all_async(0b10).await;
    ///    
    /// assert_eq!(x, [0b0, 0b10, 0b10, 0b0, 0b0, 0b10, 0b10, 0b0]);
    /// # });
    /// ```
    #[cfg(feature = "alloc")]
    async fn bitand_assign_all_async<Rhs>(&mut self, rhs: Rhs)
    where
        T: BitAndAssign<Rhs>,
        Rhs: Copy;
}

impl<T> SliceBitAndAssign<T> for [T]
{
    fn bitand_assign_all<Rhs>(&mut self, rhs: Rhs)
    where
        T: BitAndAssign<Rhs>,
        Rhs: Copy
    {
        self.visit_mut(|x| *x &= rhs)
    }

    #[cfg(feature = "alloc")]
    async fn bitand_assign_all_async<Rhs>(&mut self, rhs: Rhs)
    where
        T: BitAndAssign<Rhs>,
        Rhs: Copy
    {
        self.visit_mut_async(async |x| *x &= rhs).await
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