use slice_trait::Slice;

#[const_trait]
pub trait SliceSplit<T>: Slice<Item = T>
{
    /// Returns the lengths before and after a certain index, as if split.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use slice_ops::*;
    /// 
    /// let values = [1, 2, 3, 4];
    /// 
    /// let (len_left, len_right) = values.split_len(1);
    /// 
    /// assert_eq!(len_left, 1);
    /// assert_eq!(len_right, 3);
    /// assert_eq!(len_left + len_right, values.len());
    /// ```
    fn split_len(&self, mid: usize) -> (usize, usize);
    /// Returns the lengths before and after a certain index, as if split from the right.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use slice_ops::*;
    /// 
    /// let values = [1, 2, 3, 4];
    /// 
    /// let (len_left, len_right) = values.rsplit_len(3);
    /// 
    /// assert_eq!(len_left, 1);
    /// assert_eq!(len_right, 3);
    /// assert_eq!(len_left + len_right, values.len());
    /// ```
    fn rsplit_len(&self, mid: usize) -> (usize, usize);
        
    fn split_ptr(&self, mid: usize) -> (*const T, *const T);
    fn split_mut_ptr(&mut self, mid: usize) -> (*mut T, *mut T);

    fn rsplit_ptr(&self, mid: usize) -> (*const T, *const T);
    fn rsplit_mut_ptr(&mut self, mid: usize) -> (*mut T, *mut T);

    /// Splits the slice in two parts, from the right.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use slice_ops::*;
    /// 
    /// let values = [1, 2, 3, 4];
    /// 
    /// let (left, right) = values.rsplit_at(3);
    /// 
    /// assert_eq!(left, &[1]);
    /// assert_eq!(right, &[2, 3, 4]);
    /// ```
    fn rsplit_at(&self, mid: usize) -> (&[T], &[T]);
    /// Splits the slice in two parts, from the right.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use slice_ops::*;
    /// 
    /// let mut values = [1, 2, 3, 4];
    /// 
    /// let (left, right) = values.rsplit_at_mut(3);
    /// 
    /// assert_eq!(left, &mut [1]);
    /// assert_eq!(right, &mut [2, 3, 4]);
    /// ```
    fn rsplit_at_mut(&mut self, mid: usize) -> (&mut [T], &mut [T]);
}

impl<T> const SliceSplit<T> for [T]
{
    fn split_len(&self, mid: usize) -> (usize, usize)
    {
        crate::split_len(self.len(), mid)
    }
    fn rsplit_len(&self, mid: usize) -> (usize, usize)
    {
        crate::rsplit_len(self.len(), mid)
    }
    
    fn split_ptr(&self, mid: usize) -> (*const T, *const T)
    {
        let ptr = self.as_ptr();
        (ptr, unsafe {ptr.add(self.split_len(mid).0)})
    }
    fn split_mut_ptr(&mut self, mid: usize) -> (*mut T, *mut T)
    {
        let ptr = self.as_mut_ptr();
        (ptr, unsafe {ptr.add(self.split_len(mid).0)})
    }

    fn rsplit_ptr(&self, mid: usize) -> (*const T, *const T)
    {
        let ptr = self.as_ptr();
        (ptr, unsafe {ptr.add(self.rsplit_len(mid).0)})
    }
    fn rsplit_mut_ptr(&mut self, mid: usize) -> (*mut T, *mut T)
    {
        let ptr = self.as_mut_ptr();
        (ptr, unsafe {ptr.add(self.rsplit_len(mid).0)})
    }

    fn rsplit_at(&self, mid: usize) -> (&[T], &[T])
    {
        assert!(mid <= self.len());
        self.split_at(self.len() - mid)
    }
    fn rsplit_at_mut(&mut self, mid: usize) -> (&mut [T], &mut [T])
    {
        assert!(mid <= self.len());
        self.split_at_mut(self.len() - mid)
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