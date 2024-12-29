use core::ops::BitOrAssign;

use slice_trait::Slice;

use super::SliceVisit;

#[const_trait]
pub trait SliceBitOrAssign<T>: Slice<Item = T>
{
    /// Performs a bitwise OR on each element using `rhs`.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use slice_ops::ops::*;
    /// 
    /// let mut x = [0b1, 0b10, 0b11, 0b100, 0b101, 0b110, 0b111, 0b1000];
    /// 
    /// x.bitor_assign_all(0b10);
    ///    
    /// assert_eq!(x, [0b11, 0b10, 0b11, 0b110, 0b111, 0b110, 0b111, 0b1010]);
    /// ```
    fn bitor_assign_all<Rhs>(&mut self, rhs: Rhs)
    where
        T: BitOrAssign<Rhs>,
        Rhs: Copy;
        
    /// Asynchronously performs a bitwise OR on each element using `rhs`.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use slice_ops::ops::*;
    /// 
    /// # tokio_test::block_on(async {
    /// let mut x = [0b1, 0b10, 0b11, 0b100, 0b101, 0b110, 0b111, 0b1000];
    /// 
    /// x.bitor_assign_all_async(0b10).await;
    ///    
    /// assert_eq!(x, [0b11, 0b10, 0b11, 0b110, 0b111, 0b110, 0b111, 0b1010]);
    /// # });
    /// ```
    #[cfg(feature = "alloc")]
    async fn bitor_assign_all_async<Rhs>(&mut self, rhs: Rhs)
    where
        T: BitOrAssign<Rhs>,
        Rhs: Copy;
}

impl<T> SliceBitOrAssign<T> for [T]
{
    fn bitor_assign_all<Rhs>(&mut self, rhs: Rhs)
    where
        T: BitOrAssign<Rhs>,
        Rhs: Copy
    {
        self.visit_mut(|x| *x |= rhs)
    }

    #[cfg(feature = "alloc")]
    async fn bitor_assign_all_async<Rhs>(&mut self, rhs: Rhs)
    where
        T: BitOrAssign<Rhs>,
        Rhs: Copy
    {
        self.visit_mut_async(async |x| *x |= rhs).await
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