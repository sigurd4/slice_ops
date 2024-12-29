use core::mem::MaybeUninit;

use slice_trait::Slice;

use crate::padded::Padded;

#[const_trait]
pub trait SliceSpread<T>: Slice<Item = T>
{
    /// Spreads elements equally across `M` slices.
    /// Slices will have equal length only if the operand slice's length is divisible by `M`.
    /// 
    /// # Example
    /// 
    /// Take, for instance, that we want to divide a slice into odd and even elements:
    /// 
    /// ```rust
    /// #![feature(generic_const_exprs)]
    /// 
    /// use slice_ops::ops::*;
    /// 
    /// let arr = [1, 2, 3];
    /// let slice = arr.as_slice();
    /// 
    /// let [odd, even] = slice.spread();
    /// 
    /// assert_eq!(odd, [1, 3]);
    /// assert_eq!(even, [2]);
    /// ```
    fn spread<const M: usize>(&self) -> [&[Padded<T, M>]; M]
    where
        [(); M - 1]:;
    
    /// Spreads elements equally across `M` mutable slices.
    /// Slices will have equal length only if the operand slice's length is divisible by `M`.
    /// 
    /// # Example
    /// ```rust
    /// #![feature(generic_const_exprs)]
    /// 
    /// use slice_ops::ops::*;
    /// 
    /// let mut arr = ["1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14", "15"];
    /// let slice = arr.as_mut_slice();
    /// 
    /// let [_, _, fizz] = slice.spread_mut::<3>();
    /// assert_eq!(fizz, ["3", "6", "9", "12", "15"]);
    /// for fizz in fizz.iter_mut()
    /// {
    ///     **fizz = "fizz";
    /// }
    /// 
    /// let [_, _, _, _, buzz] = slice.spread_mut::<5>();
    /// assert_eq!(buzz, ["5", "10", "fizz"]);
    /// for buzz in buzz.iter_mut()
    /// {
    ///     if **buzz == "fizz"
    ///     {
    ///         **buzz = "fizzbuzz";
    ///         continue;
    ///     }
    ///     **buzz = "buzz";
    /// }
    /// 
    /// assert_eq!(arr, ["1", "2", "fizz", "4", "buzz", "fizz", "7", "8", "fizz", "buzz", "11", "fizz", "13", "14", "fizzbuzz"]);
    /// ```
    fn spread_mut<const M: usize>(&mut self) -> [&mut [Padded<T, M>]; M]
    where
        [(); M - 1]:;
}

impl<T> SliceSpread<T> for [T]
{   
    fn spread<const M: usize>(&self) -> [&[Padded<T, M>]; M]
    where
        [(); M - 1]:
    {
        let len = self.len();
        let ptr = self.as_ptr();
    
        let mut spread: [MaybeUninit<&[Padded<T, M>]>; M] = MaybeUninit::uninit_array();
            
        let mut i = 0;
        while i < M
        {
            spread[i].write(unsafe {
                core::slice::from_raw_parts(ptr.add(i).cast(), len/M + if len % M > i {1} else {0})
            });
            i += 1;
        }

        unsafe {
            MaybeUninit::array_assume_init(spread)
        }
    }
    fn spread_mut<const M: usize>(&mut self) -> [&mut [Padded<T, M>]; M]
    where
        [(); M - 1]:
    {
        let len = self.len();
        let ptr = self.as_mut_ptr();
    
        let mut spread: [MaybeUninit<&mut [Padded<T, M>]>; M] = MaybeUninit::uninit_array();
            
        let mut i = 0;
        while i < M
        {
            spread[i].write(unsafe {
                core::slice::from_raw_parts_mut(ptr.add(i).cast(), len/M + if len % M > i {1} else {0})
            });
            i += 1;
        }

        unsafe {
            MaybeUninit::array_assume_init(spread)
        }
    }
}

#[cfg(test)]
mod test
{
    use crate::ops::SliceSpread;

    #[test]
    fn fizzbuzz()
    {
        let mut arr = ["1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14", "15"];
        let slice = arr.as_mut_slice();

        let [_, _, fizz] = slice.spread_mut();
        assert_eq!(fizz, ["3", "6", "9", "12", "15"]);
        for fizz in fizz.iter_mut()
        {
            **fizz = "fizz";
        }

        let [_, _, _, _, buzz] = slice.spread_mut();
        assert_eq!(buzz, ["5", "10", "fizz"]);
        for buzz in buzz.iter_mut()
        {
            if **buzz == "fizz"
            {
                **buzz = "fizzbuzz";
                continue;
            }
            **buzz = "buzz";
        }

        assert_eq!(
            arr,
            ["1", "2", "fizz", "4", "buzz", "fizz", "7", "8", "fizz", "buzz", "11", "fizz", "13", "14", "fizzbuzz"]
        );
    }
}