use slice_trait::Slice;

#[const_trait]
pub trait SliceTrim<T>: Slice<Item = T>
{
    /// Returns a trimmed subslice, trimmed from both ends using a trimming predicate.
    /// 
    /// `trim` should return `true` for each element that should be trimmed.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use slice_ops::ops::*;
    /// 
    /// let a = [0, 0, 0, 1, 2, 3, 0, 0, 0];
    /// 
    /// let at = a.trim(|&e| e == 0);
    /// 
    /// assert_eq!(at, &[1, 2, 3]);
    /// ```
    fn trim<F>(&self, trim: F) -> &[T]
    where
        F: FnMut(&T) -> bool /*+ ~const Destruct*/;
    /// Returns a trimmed subslice, trimmed from the left using a trimming predicate.
    /// 
    /// `trim` should return `true` for each element that should be trimmed.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use slice_ops::ops::*;
    /// 
    /// let a = [0, 0, 0, 1, 2, 3, 0, 0, 0];
    /// 
    /// let at = a.trim_front(|&e| e == 0);
    /// 
    /// assert_eq!(at, &[1, 2, 3, 0, 0, 0]);
    /// ```
    fn trim_front<F>(&self, trim: F) -> &[T]
    where
        F: FnMut(&T) -> bool /*+ ~const Destruct*/;
    /// Returns a trimmed subslice, trimmed from the right using a trimming predicate.
    /// 
    /// `trim` should return `true` for each element that should be trimmed.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use slice_ops::ops::*;
    /// 
    /// let a = [0, 0, 0, 1, 2, 3, 0, 0, 0];
    /// 
    /// let at = a.trim_back(|&e| e == 0);
    /// 
    /// assert_eq!(at, &[0, 0, 0, 1, 2, 3]);
    /// ```
    fn trim_back<F>(&self, trim: F) -> &[T]
    where
        F: FnMut(&T) -> bool /*+ ~const Destruct*/;
    /// Returns a mutable trimmed subslice, trimmed from both ends using a trimming predicate.
    /// 
    /// `trim` should return `true` for each element that should be trimmed.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use slice_ops::ops::*;
    /// 
    /// let mut a = [0, 0, 0, 1, 2, 3, 0, 0, 0];
    /// 
    /// let at = a.trim_mut(|&e| e == 0);
    /// 
    /// assert_eq!(at, &mut [1, 2, 3]);
    /// ```
    fn trim_mut<F>(&mut self, trim: F) -> &mut [T]
    where
        F: FnMut(&T) -> bool /*+ ~const Destruct*/;
    /// Returns a mutable trimmed subslice, trimmed from the left using a trimming predicate.
    /// 
    /// `trim` should return `true` for each element that should be trimmed.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use slice_ops::ops::*;
    /// 
    /// let mut a = [0, 0, 0, 1, 2, 3, 0, 0, 0];
    /// 
    /// let at = a.trim_front_mut(|&e| e == 0);
    /// 
    /// assert_eq!(at, &mut [1, 2, 3, 0, 0, 0]);
    /// ```
    fn trim_front_mut<F>(&mut self, trim: F) -> &mut [T]
    where
        F: FnMut(&T) -> bool /*+ ~const Destruct*/;
    /// Returns a mutable trimmed subslice, trimmed from the right using a trimming predicate.
    /// 
    /// `trim` should return `true` for each element that should be trimmed.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use slice_ops::ops::*;
    /// 
    /// let mut a = [0, 0, 0, 1, 2, 3, 0, 0, 0];
    /// 
    /// let at = a.trim_back_mut(|&e| e == 0);
    /// 
    /// assert_eq!(at, &mut [0, 0, 0, 1, 2, 3]);
    /// ```
    fn trim_back_mut<F>(&mut self, trim: F) -> &mut [T]
    where
        F: FnMut(&T) -> bool /*+ ~const Destruct*/;
}

impl<T> SliceTrim<T> for [T]
{
    fn trim<F>(&self, mut trim: F) -> &[T]
    where
        F: FnMut(&T) -> bool
    {
        self.trim_back(&mut trim).trim_front(trim)
    }
    fn trim_front<F>(&self, mut trim: F) -> &[T]
    where
        F: FnMut(&T) -> bool
    {
        let mut slice = self;
        let mut range = slice.as_ptr_range();

        while matches!(slice.first().map(&mut trim), Some(true))
        {
            unsafe {
                range.start = range.start.add(1);
                slice = core::slice::from_ptr_range(range.clone())
            }
        }

        slice
    }
    fn trim_back<F>(&self, mut trim: F) -> &[T]
    where
        F: FnMut(&T) -> bool
    {
        let mut slice = self;
        let mut range = slice.as_ptr_range();

        while matches!(slice.last().map(&mut trim), Some(true))
        {
            unsafe {
                range.end = range.end.sub(1);
                slice = core::slice::from_ptr_range(range.clone())
            }
        }

        slice
    }
    fn trim_mut<F>(&mut self, mut trim: F) -> &mut [T]
    where
        F: FnMut(&T) -> bool
    {
        self.trim_back_mut(&mut trim).trim_front_mut(trim)
    }
    fn trim_front_mut<F>(&mut self, mut trim: F) -> &mut [T]
    where
        F: FnMut(&T) -> bool
    {
        let mut slice = self;
        let mut range = slice.as_mut_ptr_range();

        while matches!(slice.first().map(&mut trim), Some(true))
        {
            unsafe {
                range.start = range.start.add(1);
                slice = core::slice::from_mut_ptr_range(range.clone())
            }
        }

        slice
    }
    fn trim_back_mut<F>(&mut self, mut trim: F) -> &mut [T]
    where
        F: FnMut(&T) -> bool
    {
        let mut slice = self;
        let mut range = slice.as_mut_ptr_range();

        while matches!(slice.last().map(&mut trim), Some(true))
        {
            unsafe {
                range.end = range.end.sub(1);
                slice = core::slice::from_mut_ptr_range(range.clone())
            }
        }

        slice
    }
}

#[cfg(test)]
mod test
{
    use crate::ops::SliceTrim;

    #[test]
    fn it_works()
    {
        let a = [0, 0, 0, 1, 2, 3, 0, 0, 0];

        let at = a.trim(|&e| e == 0);

        assert_eq!(at, &[1, 2, 3]);
    }
}