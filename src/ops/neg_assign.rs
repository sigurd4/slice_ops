use core::ops::Neg;

use slice_trait::Slice;

use super::SliceVisit;

#[const_trait]
pub trait SliceNegAssign<T>: Slice<Item = T>
{
    /// Negates each element in the slice.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use slice_ops::ops::*;
    /// 
    /// let mut x = [1, 2, 3, 4, 5, 6, 7, 8];
    /// 
    /// x.neg_assign_all();
    ///    
    /// assert_eq!(x, [-1, -2, -3, -4, -5, -6, -7, -8]);
    /// ```
    fn neg_assign_all(&mut self)
    where
        T: Neg<Output = T>;
             
    /// Asynchronously negates each element in the slice.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use slice_ops::ops::*;
    /// 
    /// # tokio_test::block_on(async {
    /// let mut x = [1, 2, 3, 4, 5, 6, 7, 8];
    /// 
    /// x.neg_assign_all_async().await;
    ///    
    /// assert_eq!(x, [-1, -2, -3, -4, -5, -6, -7, -8]);
    /// # });
    /// ```
    #[cfg(feature = "alloc")]
    async fn neg_assign_all_async(&mut self)
    where
        T: Neg<Output = T>;
}

impl<T> SliceNegAssign<T> for [T]
{
    fn neg_assign_all(&mut self)
    where
        T: Neg<Output = T>
    {
        self.visit_mut(|x| unsafe {
            core::ptr::write(x, -core::ptr::read(x))
        })
    }

    #[cfg(feature = "alloc")]
    async fn neg_assign_all_async(&mut self)
    where
        T: Neg<Output = T>
    {
        self.visit_mut_async(async |x| unsafe {
            core::ptr::write(x, -core::ptr::read(x))
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