use slice_trait::Slice;

#[const_trait]
pub trait SliceArgReduce<T>: Slice<Item = T>
{       
    /// Performs an argument reduction, finding the final righthand operand for which the comparison yields true.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use slice_ops::*;
    /// 
    /// fn my_argmax<T>(slice: &[T]) -> Option<usize>
    /// where
    ///     T: PartialOrd
    /// {
    ///     slice.argreduce(PartialOrd::gt)
    /// }
    /// 
    /// fn my_argmin<T>(slice: &[T]) -> Option<usize>
    /// where
    ///     T: PartialOrd
    /// {
    ///     slice.argreduce(PartialOrd::lt)
    /// }
    /// 
    /// let x = [1, 5, 5, 6, 2, -1, 0, -4, -1, 6];
    /// 
    /// assert_eq!(my_argmax(&x), x.argmax());
    /// assert_eq!(my_argmin(&x), x.argmin());
    /// ```
    fn argreduce<'a, F>(&'a self, reduction: F) -> Option<usize>
    where
        F: FnMut(&'a T, &'a T) -> bool /*+ ~const Destruct*/,
        T: 'a;
    
    /// Performs an argument reduction on the hashed values, finding the final righthand operand for which the comparison yields true.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use slice_ops::*;
    /// 
    /// fn hasher(str: &&str) -> i32
    /// {
    ///     i32::from_str_radix(str, 10).unwrap()
    /// }
    /// 
    /// fn my_argmax(slice: &[&str]) -> Option<usize>
    /// {
    ///     slice.argreduce_key(PartialOrd::gt, hasher)
    /// }
    /// 
    /// fn my_argmin(slice: &[&str]) -> Option<usize>
    /// {
    ///     slice.argreduce_key(PartialOrd::lt, hasher)
    /// }
    /// 
    /// let x = ["1", "5", "5", "6", "2", "-1", "0", "-4", "-1", "6"];
    /// 
    /// assert_eq!(my_argmax(&x), x.argmax_by_key(hasher));
    /// assert_eq!(my_argmin(&x), x.argmin_by_key(hasher));
    /// ```
    fn argreduce_key<'a, B, FR, FB>(&'a self, reduction: FR, hasher: FB) -> Option<usize>
    where
        FR: FnMut(&B, &B) -> bool /*+ ~const Destruct*/,
        FB: FnMut(&'a T) -> B /*+ ~const Destruct*/,
        T: 'a;
}

impl<T> SliceArgReduce<T> for [T]
{   
    fn argreduce<'a, F>(&'a self, mut f: F) -> Option<usize>
    where
        F: FnMut(&'a T, &'a T) -> bool,
        T: 'a
    {
        let l = self.len();
        if l == 0
        {
            return None;
        }
        let mut i = 1;
        let mut j = 0;
        while i < l
        {
            if f(&self[i], &self[j])
            {
                j = i;
            }
            i += 1;
        }
        Some(j)
    }
    fn argreduce_key<'a, B, FR, FB>(&'a self, mut predicate: FR, mut hasher: FB) -> Option<usize>
    where
        FR: FnMut(&B, &B) -> bool,
        FB: FnMut(&'a T) -> B,
        T: 'a
    {
        let l = self.len();
        if l == 0
        {
            return None;
        }
        let mut j = 0;
        let mut i = 1;
        let mut key = hasher(&self[j]);
        while i < l
        {
            let next_key = hasher(&self[i]);
            if predicate(&next_key, &key)
            {
                j = i;
                key = next_key;
            }
            i += 1;
        }
        Some(j)
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