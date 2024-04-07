
use core::mem::{ManuallyDrop, MaybeUninit};

use core::alloc::Allocator;
use core::ops::{AddAssign, BitAndAssign, BitOrAssign, BitXorAssign, DivAssign, MulAssign, Neg, RemAssign, ShlAssign, ShrAssign, Sub, SubAssign};

pub use slice_trait::*;

use crate::{is_power_of, Padded};

#[inline]
pub const fn split_len(len: usize, mid: usize) -> (usize, usize)
{
    assert!(mid <= len);
    (mid, len - mid)
}
#[inline]
pub const fn rsplit_len(len: usize, mid: usize) -> (usize, usize)
{
    assert!(mid <= len);
    (len - mid, mid)
}

#[inline]
pub const fn split_at<T>(slice: &[T], mid: usize) -> (&[T], &[T])
{
    slice.split_at(mid)
}

#[inline]
pub const fn split_at_mut<T>(slice: &mut [T], mid: usize) -> (&mut [T], &mut [T])
{
    slice.split_at_mut(mid)
}

#[inline]
pub const fn rsplit_at<T>(slice: &[T], mid: usize) -> (&[T], &[T])
{
    assert!(mid <= slice.len());
    crate::split_at(slice, slice.len() - mid)
}

#[inline]
pub const fn rsplit_at_mut<T>(slice: &mut [T], mid: usize) -> (&mut [T], &mut [T])
{
    assert!(mid <= slice.len());
    crate::split_at_mut(slice, slice.len() - mid)
}

#[inline]
pub const fn split_array_ref<T, const N: usize>(slice: &[T]) -> (&[T; N], &[T])
{
    let (left, right) = crate::split_at(slice, N);
    unsafe {(&*(left.as_ptr() as *const [T; N]), right)}
}
#[inline]
pub const fn split_array_mut<T, const N: usize>(slice: &mut[T]) -> (&mut [T; N], &mut [T])
{
    let (left, right) = crate::split_at_mut(slice, N);
    unsafe {(&mut *(left.as_mut_ptr() as *mut [T; N]), right)}
}

#[inline]
pub const fn rsplit_array_ref<T, const N: usize>(slice: &[T]) -> (&[T], &[T; N])
{
    let (left, right) = crate::rsplit_at(slice, N);
    unsafe {(left, &*(right.as_ptr() as *const [T; N]))}
}
#[inline]
pub const fn rsplit_array_mut<T, const N: usize>(slice: &mut [T]) -> (&mut [T], &mut [T; N])
{
    let (left, right) = crate::rsplit_at_mut(slice, N);
    unsafe {(left, &mut *(right.as_mut_ptr() as *mut [T; N]))}
}

pub const fn spread_ref<T, const M: usize>(slice: &[T]) -> [&[Padded<T, M>]; M]
where
    [(); M - 1]:
{
    let len = slice.len();
    let ptr = slice.as_ptr();

    unsafe {
        let mut spread: [&[Padded<T, M>]; M] = MaybeUninit::assume_init(MaybeUninit::uninit());
        
        let mut i = 0;
        while i < M
        {
            spread[i] = core::slice::from_raw_parts(ptr.add(i).cast(), len/M + if len % M > i {1} else {0});
            i += 1;
        }

        spread
    }
}
pub const fn spread_mut<T, const M: usize>(slice: &mut [T]) -> [&mut [Padded<T, M>]; M]
where
    [(); M - 1]:
{
    let len = slice.len();
    let ptr = slice.as_mut_ptr();

    unsafe {
        let mut spread: [&mut [Padded<T, M>]; M] = MaybeUninit::assume_init(MaybeUninit::uninit());
        
        let mut i = 0;
        while i < M
        {
            spread[i] = core::slice::from_raw_parts_mut(ptr.add(i).cast(), len/M + if len % M > i {1} else {0});
            i += 1;
        }

        spread
    }
}

pub trait SliceOps<T>: Slice<Item = T>
{
    fn differentiate(&mut self)
    where
        T: SubAssign<T> + Copy;
    fn integrate(&mut self)
    where
        T: AddAssign<T> + Copy;

    fn argmax(&self) -> Option<usize>
    where
        T: PartialOrd<T>;
    fn argmin(&self) -> Option<usize>
    where
        T: PartialOrd<T>;

    fn add_assign_all<Rhs>(&mut self, rhs: Rhs)
    where
        T: AddAssign<Rhs>,
        Rhs: Copy;
    fn sub_assign_all<Rhs>(&mut self, rhs: Rhs)
    where
        T: SubAssign<Rhs>,
        Rhs: Copy;
    fn mul_assign_all<Rhs>(&mut self, rhs: Rhs)
    where
        T: MulAssign<Rhs>,
        Rhs: Copy;
    fn div_assign_all<Rhs>(&mut self, rhs: Rhs)
    where
        T: DivAssign<Rhs>,
        Rhs: Copy;
    fn rem_assign_all<Rhs>(&mut self, rhs: Rhs)
    where
        T: RemAssign<Rhs>,
        Rhs: Copy;
    fn shl_assign_all<Rhs>(&mut self, rhs: Rhs)
    where
        T: ShlAssign<Rhs>,
        Rhs: Copy;
    fn shr_assign_all<Rhs>(&mut self, rhs: Rhs)
    where
        T: ShrAssign<Rhs>,
        Rhs: Copy;
    fn bitor_assign_all<Rhs>(&mut self, rhs: Rhs)
    where
        T: BitOrAssign<Rhs>,
        Rhs: Copy;
    fn bitand_assign_all<Rhs>(&mut self, rhs: Rhs)
    where
        T: BitAndAssign<Rhs>,
        Rhs: Copy;
    fn bitxor_assign_all<Rhs>(&mut self, rhs: Rhs)
    where
        T: BitXorAssign<Rhs>,
        Rhs: Copy;
        
    fn neg_assign_all(&mut self)
    where
        T: Neg<Output = T>;

    fn shift_many_left(&mut self, items: &mut [T]);
    
    fn shift_many_right(&mut self, items: &mut [T]);
    
    fn shift_left(&mut self, item: &mut T);

    fn shift_right(&mut self, item: &mut T);

    fn split_len(&self, mid: usize) -> (usize, usize);
    fn rsplit_len(&self, mid: usize) -> (usize, usize);

    fn rsplit_at(&self, mid: usize) -> (&[T], &[T]);
    fn rsplit_at_mut(&mut self, mid: usize) -> (&mut [T], &mut [T]);

    /// Does the exact same as the method in the standard library in an identical way,
    /// but can be done at compile-time.
    /// 
    /// Divides one slice into an array and a remainder slice at an index.
    ///
    /// The array will contain all indices from `[0, N)` (excluding
    /// the index `N` itself) and the slice will contain all
    /// indices from `[N, len)` (excluding the index `len` itself).
    ///
    /// # Panics
    ///
    /// Panics if `N > len`.
    ///
    /// # Examples
    ///
    /// ```
    /// #![feature(split_array)]
    /// #![feature(const_slice_index)]
    /// 
    /// const V: [u8; 6] = [1, 2, 3, 4, 5, 6];
    ///
    /// {
    ///     const SPLIT: (&[u8; 0], &[u8]) = slice_ops::split_array_ref(V.as_slice());
    ///     assert_eq!(SPLIT.0, &[]);
    ///     assert_eq!(SPLIT.1, [1, 2, 3, 4, 5, 6]);
    ///     assert_eq!(SPLIT, V.split_array_ref::<0>());
    /// }
    ///
    /// {
    ///     const SPLIT: (&[u8; 2], &[u8]) = slice_ops::split_array_ref(V.as_slice());
    ///     assert_eq!(SPLIT.0, &[1, 2]);
    ///     assert_eq!(SPLIT.1, [3, 4, 5, 6]);
    ///     assert_eq!(SPLIT, V.split_array_ref::<2>());
    /// }
    ///
    /// {
    ///     const SPLIT: (&[u8; 6], &[u8]) = slice_ops::split_array_ref(V.as_slice());
    ///     assert_eq!(SPLIT.0, &[1, 2, 3, 4, 5, 6]);
    ///     assert_eq!(SPLIT.1, []);
    ///     assert_eq!(SPLIT, V.split_array_ref::<6>());
    /// }
    /// ```
    fn split_array_ref2<const N: usize>(&self) -> (&[T; N], &[T]);
    fn split_array_mut2<const N: usize>(&mut self) -> (&mut [T; N], &mut [T]);

    fn rsplit_array_ref2<const N: usize>(&self) -> (&[T], &[T; N]);
    fn rsplit_array_mut2<const N: usize>(&mut self) -> (&mut [T], &mut [T; N]);
    
    /// Spreads elements equally across `M` slices.
    /// Slices will have equal length only if the operand slice's length is divisible by `M`.
    /// 
    /// # Examples
    /// Take, for instance, that we want to divide a slice into odd and even elements:
    /// ```rust
    /// use slice_ops::SliceOps;
    /// 
    /// let arr = [1, 2, 3];
    /// let slice = arr.as_slice();
    /// 
    /// let [odd, even] = slice.spread_ref();
    /// 
    /// assert_eq!(odd, [1, 3]);
    /// assert_eq!(even, [2]);
    /// ```
    fn spread_ref<const M: usize>(&self) -> [&[Padded<T, M>]; M]
    where
        [(); M - 1]:;
    
    /// Spreads elements equally across `M` mutable slices.
    /// Slices will have equal length only if the operand slice's length is divisible by `M`.
    /// 
    /// # Examples
    /// ```rust
    /// #![feature(generic_const_exprs)]
    /// 
    /// use slice_ops::SliceOps;
    /// 
    /// let mut arr = ["1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14", "15"];
    /// let slice = arr.as_mut_slice();
    /// 
    /// let [_, _, fizz] = slice.spread_mut();
    /// assert_eq!(fizz, ["3", "6", "9", "12", "15"]);
    /// for fizz in fizz.iter_mut()
    /// {
    ///     **fizz = "fizz";
    /// }
    /// 
    /// let [_, _, _, _, buzz] = slice.spread_mut();
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
        
    /// Performs the bit-reverse permutation. Length must be a power of 2.
    /// 
    /// # Example
    /// ```rust
    /// use slice_ops::*;
    /// 
    /// let mut arr = [0b000, 0b001, 0b010, 0b011, 0b100, 0b101, 0b110, 0b111];
    /// 
    /// arr.as_mut_slice().bit_rev_permutation();
    /// 
    /// assert_eq!(arr, [0b000, 0b100, 0b010, 0b110, 0b001, 0b101, 0b011, 0b111])
    /// ```
    fn bit_rev_permutation(&mut self);
    fn digit_rev_permutation(&mut self, radix: usize);
    
    /// Performs the grey code permutation. Length must be a power of 2.
    /// 
    /// # Example
    /// ```rust
    /// use slice_ops::*;
    /// 
    /// let mut arr = [0b000, 0b001, 0b010, 0b011, 0b100, 0b101, 0b110, 0b111];
    /// 
    /// arr.as_mut_slice().grey_code_permutation();
    /// 
    /// assert_eq!(arr, [0b000, 0b001, 0b011, 0b010, 0b110, 0b111, 0b101, 0b100])
    /// ```
    fn grey_code_permutation(&mut self);

    fn trim<F>(&self, trim: F) -> &[T]
    where
        F: FnMut(&T) -> bool;
    fn trim_front<F>(&self, trim: F) -> &[T]
    where
        F: FnMut(&T) -> bool;
    fn trim_back<F>(&self, trim: F) -> &[T]
    where
        F: FnMut(&T) -> bool;
    fn trim_mut<F>(&mut self, trim: F) -> &mut [T]
    where
        F: FnMut(&T) -> bool;
    fn trim_front_mut<F>(&mut self, trim: F) -> &mut [T]
    where
        F: FnMut(&T) -> bool;
    fn trim_back_mut<F>(&mut self, trim: F) -> &mut [T]
    where
        F: FnMut(&T) -> bool;
}

impl<T> const SliceOps<T> for [T]
{
    fn differentiate(&mut self)
    where
        T: SubAssign<T> + Copy
    {
        let len = self.len();
        if len > 0
        {
            let mut i = len - 1;
            while i > 0
            {
                self[i] -= self[i - 1];
                i -= 1;
            }
        }
    }
    fn integrate(&mut self)
    where
        T: AddAssign<T> + Copy
    {
        let len = self.len();
        let mut i = 1;
        while i < len
        {
            self[i] += self[i - 1];
            i += 1;
        }
    }

    fn argmax(&self) -> Option<usize>
    where
        T: PartialOrd<T>
    {
        match self.iter().enumerate().reduce(|a, b| if a.1 >= b.1 {a} else {b})
        {
            Some((i, _)) => Some(i),
            None => None
        }
    }
        
    fn argmin(&self) -> Option<usize>
    where
        T: PartialOrd<T>
    {
        match self.iter().enumerate().reduce(|a, b| if a.1 <= b.1 {a} else {b})
        {
            Some((i, _)) => Some(i),
            None => None
        }
    }

    fn add_assign_all<Rhs>(&mut self, rhs: Rhs)
    where
        T: AddAssign<Rhs>,
        Rhs: Copy
    {
        for x in self.iter_mut()
        {
            *x += rhs;
        }
    }
    fn sub_assign_all<Rhs>(&mut self, rhs: Rhs)
    where
        T: SubAssign<Rhs>,
        Rhs: Copy
    {
        for x in self.iter_mut()
        {
            *x -= rhs;
        }
    }
    fn mul_assign_all<Rhs>(&mut self, rhs: Rhs)
    where
        T: MulAssign<Rhs>,
        Rhs: Copy
    {
        for x in self.iter_mut()
        {
            *x *= rhs;
        }
    }
    fn div_assign_all<Rhs>(&mut self, rhs: Rhs)
    where
        T: DivAssign<Rhs>,
        Rhs: Copy
    {
        for x in self.iter_mut()
        {
            *x /= rhs;
        }
    }
    fn rem_assign_all<Rhs>(&mut self, rhs: Rhs)
    where
        T: RemAssign<Rhs>,
        Rhs: Copy
    {
        for x in self.iter_mut()
        {
            *x %= rhs;
        }
    }
    fn shl_assign_all<Rhs>(&mut self, rhs: Rhs)
    where
        T: ShlAssign<Rhs>,
        Rhs: Copy
    {
        for x in self.iter_mut()
        {
            *x <<= rhs;
        }
    }
    fn shr_assign_all<Rhs>(&mut self, rhs: Rhs)
    where
        T: ShrAssign<Rhs>,
        Rhs: Copy
    {
        for x in self.iter_mut()
        {
            *x >>= rhs;
        }
    }
    fn bitor_assign_all<Rhs>(&mut self, rhs: Rhs)
    where
        T: BitOrAssign<Rhs>,
        Rhs: Copy
    {
        for x in self.iter_mut()
        {
            *x |= rhs;
        }
    }
    fn bitand_assign_all<Rhs>(&mut self, rhs: Rhs)
    where
        T: BitAndAssign<Rhs>,
        Rhs: Copy
    {
        for x in self.iter_mut()
        {
            *x &= rhs;
        }
    }
    fn bitxor_assign_all<Rhs>(&mut self, rhs: Rhs)
    where
        T: BitXorAssign<Rhs>,
        Rhs: Copy
    {
        for x in self.iter_mut()
        {
            *x ^= rhs;
        }
    }
    
    fn neg_assign_all(&mut self)
    where
        T: Neg<Output = T>
    {
        let mut i = 0;
        while i < self.len()
        {
            unsafe {
                let ptr = self.as_mut_ptr().add(i);
                ptr.write(-ptr.read());
            }
            i += 1;
        }
    }

    fn shift_many_left(&mut self, items: &mut [T])
    {
        let len = self.len();
        let m = items.len();
        unsafe {
            items.rotate_left(m.saturating_sub(len));
            core::ptr::swap_nonoverlapping(self.as_mut_ptr(), items.as_mut_ptr(), m.min(len));
            self.rotate_left(m.min(len));
        }
    }
    
    fn shift_many_right(&mut self, items: &mut [T])
    {
        let len = self.len();
        let m = items.len();
        unsafe {
            self.rotate_right(m.min(len));
            core::ptr::swap_nonoverlapping(self.as_mut_ptr(), items.as_mut_ptr(), m.min(len));
            items.rotate_right(m.saturating_sub(len));
        }
    }
    
    fn shift_left(&mut self, item: &mut T)
    {
        if self.len() > 0
        {
            unsafe {
                core::ptr::swap_nonoverlapping(self.as_mut_ptr(), item as *mut T, 1);
            }
            self.rotate_left(1);
        }
    }

    fn shift_right(&mut self, item: &mut T)
    {
        let len = self.len();
        if len > 0
        {
            self.rotate_right(1);
            unsafe {
                core::ptr::swap_nonoverlapping(self.as_mut_ptr(), item as *mut T, 1);
            }
        }
    }

    fn split_len(&self, mid: usize) -> (usize, usize)
    {
        crate::split_len(self.len(), mid)
    }
    fn rsplit_len(&self, mid: usize) -> (usize, usize)
    {
        crate::rsplit_len(self.len(), mid)
    }

    fn rsplit_at(&self, mid: usize) -> (&[T], &[T])
    {
        crate::rsplit_at(self, mid)
    }
    fn rsplit_at_mut(&mut self, mid: usize) -> (&mut [T], &mut [T])
    {
        crate::rsplit_at_mut(self, mid)
    }

    fn split_array_ref2<const N: usize>(&self) -> (&[T; N], &[T])
    {
        crate::split_array_ref(self)
    }
    fn split_array_mut2<const N: usize>(&mut self) -> (&mut [T; N], &mut [T])
    {
        crate::split_array_mut(self)
    }

    fn rsplit_array_ref2<const N: usize>(&self) -> (&[T], &[T; N])
    {
        crate::rsplit_array_ref(self)
    }
    fn rsplit_array_mut2<const N: usize>(&mut self) -> (&mut [T], &mut [T; N])
    {
        crate::rsplit_array_mut(self)
    }
    
    fn spread_ref<const M: usize>(&self) -> [&[Padded<T, M>]; M]
    where
        [(); M - 1]:
    {
        crate::spread_ref(self)
    }
    fn spread_mut<const M: usize>(&mut self) -> [&mut [Padded<T, M>]; M]
    where
        [(); M - 1]:
    {
        crate::spread_mut(self)
    }
    
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
        assert!(is_power_of(len, radix), "Length must be a power of radix.");
    
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

        let mut i = 0;
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

        while slice.first().map(&mut trim).unwrap_or(false)
        {
            slice = slice.get(1..).unwrap_or_default()
        }

        slice
    }
    fn trim_back<F>(&self, mut trim: F) -> &[T]
    where
        F: FnMut(&T) -> bool
    {
        let mut slice = self;

        while slice.last().map(&mut trim).unwrap_or(false)
        {
            slice = slice.get(..slice.len() - 1).unwrap_or_default()
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

        while slice.first().map(&mut trim).unwrap_or(false)
        {
            slice = slice.get_mut(1..).unwrap_or_default()
        }

        slice
    }
    fn trim_back_mut<F>(&mut self, mut trim: F) -> &mut [T]
    where
        F: FnMut(&T) -> bool
    {
        let mut slice = self;

        while slice.last().map(&mut trim).unwrap_or(false)
        {
            slice = slice.get_mut(..slice.len() - 1).unwrap_or_default()
        }

        slice
    }
}

#[test]
fn test()
{
    let a = [0, 1, 0];
    
    let b = a.trim_back(|a| *a == 0);

    println!("b = {:?}", b);
}