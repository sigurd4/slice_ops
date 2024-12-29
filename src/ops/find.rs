use slice_trait::Slice;

#[const_trait]
pub trait SliceFind<T>: Slice<Item = T>
{
    /// Performs a linear search for the first value that equals `x`.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use slice_ops::ops::*;
    /// 
    /// //                   v
    /// let x = [3, 1, 4, 1, 5, 9, 2, 6, 5, 3];
    /// 
    /// let i = x.find(&5).unwrap();
    /// 
    /// assert_eq!(i, 4);
    /// assert_eq!(x[i], 5);
    /// ```
    fn find(&self, x: &T) -> Option<usize>
    where
        T: PartialEq;
    /// Performs a linear search for the first value that satisfies the given predicate.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use slice_ops::ops::*;
    /// 
    /// //                      v
    /// let x = [3, 1, 4, 1, 5, 9, 2, 6, 5, 3];
    /// 
    /// let f = |&xn| xn > 5; 
    /// 
    /// let i = x.find_by(f).unwrap();
    /// 
    /// assert_eq!(i, 5);
    /// ```
    fn find_by<'a, F>(&'a self, f: F) -> Option<usize>
    where
        F: FnMut(&'a T) -> bool /*+ ~const Destruct*/,
        T: 'a;
    /// Performs a linear search for the first value that matches the given key given a hashing function.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use slice_ops::ops::*;
    /// 
    /// //             v
    /// let x = [3, 1, 4, 1, 5, 9, 2, 6, 5, 3];
    /// 
    /// let f = |&xn| xn % 2;
    /// 
    /// let i = x.find_by_key(&0, f).unwrap();
    /// 
    /// assert_eq!(i, 2);
    /// ```
    fn find_by_key<'a, B, F>(&'a self, b: &B, f: F) -> Option<usize>
    where
        F: FnMut(&'a T) -> B /*+ ~const Destruct*/,
        B: PartialEq,
        T: 'a;
        
    /// Performs a linear search from the right for the first value that equals `x`.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use slice_ops::ops::*;
    /// 
    /// //                               v
    /// let x = [3, 1, 4, 1, 5, 9, 2, 6, 5, 3];
    /// 
    /// let i = x.rfind(&5).unwrap();
    /// 
    /// assert_eq!(i, 8);
    /// assert_eq!(x[i], 5);
    /// ```
    fn rfind(&self, x: &T) -> Option<usize>
    where
        T: PartialEq;
    /// Performs a linear search from the right for the first value that satisfies the given predicate.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use slice_ops::ops::*;
    /// 
    /// //                            v
    /// let x = [3, 1, 4, 1, 5, 9, 2, 6, 5, 3];
    /// 
    /// let f = |&xn| xn > 5;
    /// 
    /// let i = x.rfind_by(f).unwrap();
    /// 
    /// assert_eq!(i, 7);
    /// ```
    fn rfind_by<'a, F>(&'a self, f: F) -> Option<usize>
    where
        F: FnMut(&'a T) -> bool /*+ ~const Destruct*/,
        T: 'a;
    /// Performs a linear search from the right for the first value that matches the given key given a hashing function.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use slice_ops::ops::*;
    /// 
    /// //                            v
    /// let x = [3, 1, 4, 1, 5, 9, 2, 6, 5, 3];
    /// 
    /// let f = |&xn| xn % 2;
    /// 
    /// let i = x.rfind_by_key(&0, f).unwrap();
    /// 
    /// assert_eq!(i, 7);
    /// ```
    fn rfind_by_key<'a, B, F>(&'a self, b: &B, f: F) -> Option<usize>
    where
        F: FnMut(&'a T) -> B /*+ ~const Destruct*/,
        B: PartialEq,
        T: 'a;
}

impl<T> SliceFind<T> for [T]
{
    fn find(&self, x: &T) -> Option<usize>
    where
        T: PartialEq
    {
        self.find_by(|e| e == x)
    }
    fn find_by<'a, F>(&'a self, mut f: F) -> Option<usize>
    where
        F: FnMut(&'a T) -> bool,
        T: 'a
    {
        let l = self.len();
        let mut i = 0;
        
        while i < l
        {
            if f(&self[i])
            {
                return Some(i)
            }
            i += 1
        }

        None
    }
    fn find_by_key<'a, B, F>(&'a self, b: &B, mut f: F) -> Option<usize>
    where
        F: FnMut(&'a T) -> B,
        B: PartialEq,
        T: 'a
    {
        self.find_by(|e| f(e) == *b)
    }
        
    fn rfind(&self, x: &T) -> Option<usize>
    where
        T: PartialEq
    {
        self.rfind_by(|e| e == x)
    }
    fn rfind_by<'a, F>(&'a self, mut f: F) -> Option<usize>
    where
        F: FnMut(&'a T) -> bool,
        T: 'a
    {
        let l = self.len();
        let mut i = l;
        
        while i > 0
        {
            i -= 1;
            if f(&self[i])
            {
                return Some(i)
            }
        }

        None
    }
    fn rfind_by_key<'a, B, F>(&'a self, b: &B, mut f: F) -> Option<usize>
    where
        F: FnMut(&'a T) -> B,
        B: PartialEq,
        T: 'a
    {
        self.rfind_by(|e| f(e) == *b)
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