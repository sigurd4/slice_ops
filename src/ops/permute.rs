use slice_trait::Slice;

#[const_trait]
pub trait SlicePermute<T>: Slice<Item = T>
{
    /// Performs the bit-reverse permutation. Length must be a power of 2.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use slice_ops::ops::*;
    /// 
    /// let mut arr = [0b000, 0b001, 0b010, 0b011, 0b100, 0b101, 0b110, 0b111];
    /// 
    /// arr.bit_rev_permutation();
    /// 
    /// assert_eq!(arr, [0b000, 0b100, 0b010, 0b110, 0b001, 0b101, 0b011, 0b111])
    /// ```
    fn bit_rev_permutation(&mut self);

    /// Performs the digit-reverse permutation with any radix. Length must be a power of the radix.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use slice_ops::ops::*;
    /// 
    /// let mut arr = [0b000, 0b001, 0b010, 0b011, 0b100, 0b101, 0b110, 0b111];
    /// 
    /// arr.digit_rev_permutation(2);
    /// 
    /// assert_eq!(arr, [0b000, 0b100, 0b010, 0b110, 0b001, 0b101, 0b011, 0b111])
    /// ```
    fn digit_rev_permutation(&mut self, radix: usize);
    
    /// Performs the grey code permutation. Length must be a power of 2.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use slice_ops::ops::*;
    /// 
    /// let mut arr = [0b000, 0b001, 0b010, 0b011, 0b100, 0b101, 0b110, 0b111];
    /// 
    /// arr.as_mut_slice().grey_code_permutation();
    /// 
    /// assert_eq!(arr, [0b000, 0b001, 0b011, 0b010, 0b110, 0b111, 0b101, 0b100])
    /// ```
    fn grey_code_permutation(&mut self);
}

impl<T> SlicePermute<T> for [T]
{   
    fn bit_rev_permutation(&mut self)
    {
        self.digit_rev_permutation(2)
    }
    
    fn digit_rev_permutation(&mut self, radix: usize)
    {
        let len = self.len();
        if len <= radix
        {
            return;
        }
        assert!(crate::is_power_of(len, radix), "Length must be a power of radix.");
    
        let mut i = 1;
        let mut j = len/radix + 1;
        while i < len - 1
        {
            if i < j - 1
            {
                unsafe {
                    core::ptr::swap_nonoverlapping(self.as_mut_ptr().add(i), self.as_mut_ptr().add(j - 1), 1);
                }
            }
            let mut k = len/radix;
            while k*(radix - 1) < j
            {
                j -= k*(radix - 1);
                k /= radix;
            }
            j += k;
            i += 1;
        }
    }
    
    fn grey_code_permutation(&mut self)
    {
        let len = self.len();
        if len <= 2
        {
            return;
        }
        assert!(len.is_power_of_two(), "Length must be a power of two.");

        let mut i = 2;
        while i < len
        {
            let mut j = i ^ (i >> 1);
            while j < i
            {
                j = j ^ (j >> 1);
            }
            if j != i
            {
                unsafe {
                    core::ptr::swap_nonoverlapping(self.as_mut_ptr().add(i), self.as_mut_ptr().add(j), 1);
                }
            }
            i += 1;
        }
    }
}

#[cfg(test)]
mod test
{
    use crate::ops::SlicePermute;

    #[test]
    fn test_grey_code_permutation()
    {
        let mut arr = [0b000, 0b001, 0b010, 0b011, 0b100, 0b101, 0b110, 0b111];

        arr.as_mut_slice().grey_code_permutation();

        assert_eq!(arr, [0b000, 0b001, 0b011, 0b010, 0b110, 0b111, 0b101, 0b100])
    }
}