use core::ops::Not;

use slice_trait::Slice;

use super::SliceVisit;

#[const_trait]
pub trait SliceNotAssign<T>: Slice<Item = T>
{   
    /// Performs a logical NOT or bitwise NOT on each element in the slice.
    /// 
    /// Booleans will be treated with a logical NOT, while integers will be treated with a bitwise NOT.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use slice_ops::*;
    /// 
    /// let mut x = [true, false, true, false, true, false, true, true];
    /// 
    /// x.not_assign_all();
    ///    
    /// assert_eq!(x, [false, true, false, true, false, true, false, false]);
    /// ```
    fn not_assign_all(&mut self)
    where
        T: Not<Output = T>;
        
    /// Asynchronously performs a logical NOT or bitwise NOT on each element in the slice.
    /// 
    /// Booleans will be treated with a logical NOT, while integers will be treated with a bitwise NOT.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use slice_ops::*;
    /// 
    /// # tokio_test::block_on(async {
    /// let mut x = [true, false, true, false, true, false, true, true];
    /// 
    /// x.not_assign_all_async().await;
    ///    
    /// assert_eq!(x, [false, true, false, true, false, true, false, false]);
    /// # });
    /// ```
    #[cfg(feature = "alloc")]
    async fn not_assign_all_async(&mut self)
    where
        T: Not<Output = T>;
}

impl<T> SliceNotAssign<T> for [T]
{
    fn not_assign_all(&mut self)
    where
        T: Not<Output = T>
    {
        self.visit_mut(|x| unsafe {
            core::ptr::write(x, !core::ptr::read(x))
        })
    }

    #[cfg(feature = "alloc")]
    async fn not_assign_all_async(&mut self)
    where
        T: Not<Output = T>
    {
        self.visit_mut_async(async |x| unsafe {
            core::ptr::write(x, !core::ptr::read(x))
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