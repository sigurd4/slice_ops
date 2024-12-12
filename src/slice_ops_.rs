
use core::cmp::Ordering;
use core::mem::MaybeUninit;

use core::{cmp::{Ord, PartialOrd}, ops::{AddAssign, BitAndAssign, BitOrAssign, BitXorAssign, DivAssign, FnMut, MulAssign, Neg, Not, RemAssign, ShlAssign, ShrAssign, SubAssign}};

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

pub const fn spread_chunks<T, const M: usize>(slice: &[T]) -> [&[Padded<T, M>]; M]
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
pub const fn spread_chunks_mut<T, const M: usize>(slice: &mut [T]) -> [&mut [Padded<T, M>]; M]
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

#[const_trait]
pub trait SliceOps<T>: Slice<Item = T>
{
    fn differentiate(&mut self)
    where
        T: SubAssign<T> + Copy;
    fn integrate(&mut self)
    where
        T: AddAssign<T> + Copy;

    fn find(&self, x: &T) -> Option<usize>
    where
        T: PartialEq;
    fn find_by<F>(&self, f: F) -> Option<usize>
    where
        F: FnMut(&T) -> bool;
    fn find_by_key<B, F>(&self, b: &B, f: F) -> Option<usize>
    where
        F: FnMut(&T) -> B,
        B: PartialEq;
        
    fn rfind(&self, x: &T) -> Option<usize>
    where
        T: PartialEq;
    fn rfind_by<F>(&self, f: F) -> Option<usize>
    where
        F: FnMut(&T) -> bool;
    fn rfind_by_key<B, F>(&self, b: &B, f: F) -> Option<usize>
    where
        F: FnMut(&T) -> B,
        B: PartialEq;
        
    fn argconverge<F>(&self, f: F) -> Option<usize>
    where
        F: FnMut(&T, &T) -> bool;

    fn argmax(&self) -> Option<usize>
    where
        T: PartialOrd<T>;
    fn argmin(&self) -> Option<usize>
    where
        T: PartialOrd<T>;
    fn argmax_by<F>(&self, f: F) -> Option<usize>
    where
        F: FnMut(&T, &T) -> Ordering;
    fn argmin_by<F>(&self, f: F) -> Option<usize>
    where
        F: FnMut(&T, &T) -> Ordering;
    fn argmax_by_key<B, F>(&self, f: F) -> Option<usize>
    where
        F: FnMut(&T) -> B,
        B: PartialOrd;
    fn argmin_by_key<B, F>(&self, f: F) -> Option<usize>
    where
        F: FnMut(&T) -> B,
        B: PartialOrd;

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
    fn not_assign_all(&mut self)
    where
        T: Not<Output = T>;

    fn shift_many_left(&mut self, items: &mut [T]);
    
    fn shift_many_right(&mut self, items: &mut [T]);
    
    fn shift_left(&mut self, item: &mut T);

    fn shift_right(&mut self, item: &mut T);

    fn split_len(&self, mid: usize) -> (usize, usize);
    fn rsplit_len(&self, mid: usize) -> (usize, usize);

    fn rsplit_at(&self, mid: usize) -> (&[T], &[T]);
    fn rsplit_at_mut(&mut self, mid: usize) -> (&mut [T], &mut [T]);

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
    /// let [odd, even] = slice.spread_chunks();
    /// 
    /// assert_eq!(odd, [1, 3]);
    /// assert_eq!(even, [2]);
    /// ```
    fn spread_chunks<const M: usize>(&self) -> [&[Padded<T, M>]; M]
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
    /// let [_, _, fizz] = slice.spread_chunks_mut();
    /// assert_eq!(fizz, ["3", "6", "9", "12", "15"]);
    /// for fizz in fizz.iter_mut()
    /// {
    ///     **fizz = "fizz";
    /// }
    /// 
    /// let [_, _, _, _, buzz] = slice.spread_chunks_mut();
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
    fn spread_chunks_mut<const M: usize>(&mut self) -> [&mut [Padded<T, M>]; M]
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

/// Waiting for `core::maker::TriviallyDrop` to arrive before making this const again...
impl<T> /*const*/ SliceOps<T> for [T]
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
    
    fn find(&self, x: &T) -> Option<usize>
    where
        T: PartialEq
    {
        self.find_by(|e| e == x)
    }
    fn find_by<F>(&self, mut f: F) -> Option<usize>
    where
        F: FnMut(&T) -> bool
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
    fn find_by_key<B, F>(&self, b: &B, mut f: F) -> Option<usize>
    where
        F: FnMut(&T) -> B,
        B: PartialEq
    {
        self.find_by(|e| f(e) == *b)
    }
        
    fn rfind(&self, x: &T) -> Option<usize>
    where
        T: PartialEq
    {
        self.rfind_by(|e| e == x)
    }
    fn rfind_by<F>(&self, mut f: F) -> Option<usize>
    where
        F: FnMut(&T) -> bool
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
    fn rfind_by_key<B, F>(&self, b: &B, mut f: F) -> Option<usize>
    where
        F: FnMut(&T) -> B,
        B: PartialEq
    {
        self.rfind_by(|e| f(e) == *b)
    }
    
    fn argconverge<F>(&self, mut f: F) -> Option<usize>
    where
        F: FnMut(&T, &T) -> bool
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

    fn argmax(&self) -> Option<usize>
    where
        T: PartialOrd
    {
        self.argconverge(PartialOrd::gt)
    }
    fn argmin(&self) -> Option<usize>
    where
        T: PartialOrd
    {
        self.argconverge(PartialOrd::lt)
    }
    fn argmax_by<F>(&self, mut f: F) -> Option<usize>
    where
        F: FnMut(&T, &T) -> Ordering
    {
        self.argconverge(|a, b| matches!(f(a, b), Ordering::Greater))
    }
    fn argmin_by<F>(&self, mut f: F) -> Option<usize>
    where
        F: FnMut(&T, &T) -> Ordering
    {
        self.argconverge(|a, b| matches!(f(a, b), Ordering::Less))
    }
    fn argmax_by_key<B, F>(&self, mut f: F) -> Option<usize>
    where
        F: FnMut(&T) -> B,
        B: PartialOrd
    {
        self.argconverge(|a, b| f(a).gt(&f(b)))
    }
    fn argmin_by_key<B, F>(&self, mut f: F) -> Option<usize>
    where
        F: FnMut(&T) -> B,
        B: PartialOrd
    {
        self.argconverge(|a, b| f(a).lt(&f(b)))
    }

    fn add_assign_all<Rhs>(&mut self, rhs: Rhs)
    where
        T: AddAssign<Rhs>,
        Rhs: Copy
    {
        let l = self.len();
        let mut i = 0;
        while i < l
        {
            self[i] += rhs;
            i += 1;
        }
    }
    fn sub_assign_all<Rhs>(&mut self, rhs: Rhs)
    where
        T: SubAssign<Rhs>,
        Rhs: Copy
    {
        let l = self.len();
        let mut i = 0;
        while i < l
        {
            self[i] -= rhs;
            i += 1;
        }
    }
    fn mul_assign_all<Rhs>(&mut self, rhs: Rhs)
    where
        T: MulAssign<Rhs>,
        Rhs: Copy
    {
        let l = self.len();
        let mut i = 0;
        while i < l
        {
            self[i] *= rhs;
            i += 1;
        }
    }
    fn div_assign_all<Rhs>(&mut self, rhs: Rhs)
    where
        T: DivAssign<Rhs>,
        Rhs: Copy
    {
        let l = self.len();
        let mut i = 0;
        while i < l
        {
            self[i] /= rhs;
            i += 1;
        }
    }
    fn rem_assign_all<Rhs>(&mut self, rhs: Rhs)
    where
        T: RemAssign<Rhs>,
        Rhs: Copy
    {
        let l = self.len();
        let mut i = 0;
        while i < l
        {
            self[i] %= rhs;
            i += 1;
        }
    }
    fn shl_assign_all<Rhs>(&mut self, rhs: Rhs)
    where
        T: ShlAssign<Rhs>,
        Rhs: Copy
    {
        let l = self.len();
        let mut i = 0;
        while i < l
        {
            self[i] <<= rhs;
            i += 1;
        }
    }
    fn shr_assign_all<Rhs>(&mut self, rhs: Rhs)
    where
        T: ShrAssign<Rhs>,
        Rhs: Copy
    {
        let l = self.len();
        let mut i = 0;
        while i < l
        {
            self[i] >>= rhs;
            i += 1;
        }
    }
    fn bitor_assign_all<Rhs>(&mut self, rhs: Rhs)
    where
        T: BitOrAssign<Rhs>,
        Rhs: Copy
    {
        let l = self.len();
        let mut i = 0;
        while i < l
        {
            self[i] |= rhs;
            i += 1;
        }
    }
    fn bitand_assign_all<Rhs>(&mut self, rhs: Rhs)
    where
        T: BitAndAssign<Rhs>,
        Rhs: Copy
    {
        let l = self.len();
        let mut i = 0;
        while i < l
        {
            self[i] &= rhs;
            i += 1;
        }
    }
    fn bitxor_assign_all<Rhs>(&mut self, rhs: Rhs)
    where
        T: BitXorAssign<Rhs>,
        Rhs: Copy
    {
        let l = self.len();
        let mut i = 0;
        while i < l
        {
            self[i] ^= rhs;
            i += 1;
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
    fn not_assign_all(&mut self)
    where
        T: Not<Output = T>
    {
        let mut i = 0;
        while i < self.len()
        {
            unsafe {
                let ptr = self.as_mut_ptr().add(i);
                ptr.write(!ptr.read());
            }
            i += 1;
        }
    }

    fn shift_many_left(&mut self, items: &mut [T])
    {
        let len = self.len();
        let m = items.len();
        let q = m.min(len);
        unsafe {
            items.rotate_left(m.saturating_sub(len));
            core::ptr::swap_nonoverlapping(self.as_mut_ptr(), items.as_mut_ptr(), q);
            self.rotate_left(q);
        }
    }
    
    fn shift_many_right(&mut self, items: &mut [T])
    {
        let len = self.len();
        let m = items.len();
        let q = m.min(len);
        unsafe {
            self.rotate_right(q);
            core::ptr::swap_nonoverlapping(self.as_mut_ptr(), items.as_mut_ptr(), q);
            items.rotate_right(m.saturating_sub(len));
        }
    }
    
    fn shift_left(&mut self, item: &mut T)
    {
        let l = self.len();
        if l <= 1
        {
            return;
        }
        let p = self.as_mut_ptr_range();
        unsafe {
            core::ptr::swap_nonoverlapping(p.start, item as *mut T, 1);

            let x = p.start.read();
            core::ptr::copy(p.start.add(1), p.start, l - 1);
            p.end.sub(1).write(x);
        }
    }

    fn shift_right(&mut self, item: &mut T)
    {
        let l = self.len();
        if l <= 1
        {
            return;
        }
        let p = self.as_mut_ptr_range();
        unsafe {
            let x = p.end.sub(1).read();
            core::ptr::copy(p.start, p.start.add(1), l - 1);
            p.start.write(x);

            core::ptr::swap_nonoverlapping(p.start, item as *mut T, 1);
        }
    }

    fn split_len(&self, mid: usize) -> (usize, usize)
    {
        assert!(mid <= self.len());
        (mid, self.len() - mid)
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
    
    fn spread_chunks<const M: usize>(&self) -> [&[Padded<T, M>]; M]
    where
        [(); M - 1]:
    {
        crate::spread_chunks(self)
    }
    fn spread_chunks_mut<const M: usize>(&mut self) -> [&mut [Padded<T, M>]; M]
    where
        [(); M - 1]:
    {
        crate::spread_chunks_mut(self)
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

#[test]
fn test()
{
    let a = [0, 1, 0];
    
    let b = a.trim_back(|a| *a == 0);

    println!("b = {:?}", b);
}