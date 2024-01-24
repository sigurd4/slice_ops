
use core::mem::{ManuallyDrop, MaybeUninit};

use core::alloc::Allocator;

pub use slice_trait::*;

use crate::Padded;

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

#[const_trait]
pub trait SliceOps<T>: Slice<Item = T>
{
    #[cfg(feature = "std")]
    fn fill_boxed<F>(fill: F, len: usize) -> Box<Self>
    where
        F: FnMut(usize) -> T;
    #[cfg(feature = "std")]
    fn rfill_boxed<F>(fill: F, len: usize) -> Box<Self>
    where
        F: FnMut(usize) -> T;
        
    #[cfg(feature = "std")]
    fn fill_boxed_in<F, A>(fill: F, len: usize, alloc: A) -> Box<Self, A>
    where
        F: FnMut(usize) -> T,
        A: Allocator;
    #[cfg(feature = "std")]
    fn rfill_boxed_in<F, A>(fill: F, len: usize, alloc: A) -> Box<Self, A>
    where
        F: FnMut(usize) -> T,
        A: Allocator;

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
    /// arr.as_mut_slice().bit_reverse_permutation();
    /// 
    /// assert_eq!(arr, [0b000, 0b100, 0b010, 0b110, 0b001, 0b101, 0b011, 0b111])
    /// ```
    fn bit_reverse_permutation(&mut self);
}

impl<T> const SliceOps<T> for [T]
{
    #[cfg(feature = "std")]
    fn fill_boxed<F>(mut fill: F, len: usize) -> Box<Self>
    where
        F: FnMut(usize) -> T
    {
        let mut slice = unsafe {
            Box::new_uninit_slice(len).assume_init()
        };
        let ptr: *mut T = slice.as_mut_ptr();
        let mut i = 0;
        while i < len
        {
            unsafe {
                ptr.add(i).write(fill(i));
            }
            i += 1;
        }
        slice
    }
    #[cfg(feature = "std")]
    fn rfill_boxed<F>(mut fill: F, len: usize) -> Box<Self>
    where
        F: FnMut(usize) -> T
    {
        let mut slice = unsafe {
            Box::new_uninit_slice(len).assume_init()
        };
        if len != 0
        {
            let ptr: *mut T = slice.as_mut_ptr();
            let mut i = len - 1;
            loop
            {
                unsafe {
                    ptr.add(i).write(fill(i));
                }
                if i == 0
                {
                    break
                }
                i -= 1;
            }
        }
        slice
    }
    
    #[cfg(feature = "std")]
    fn fill_boxed_in<F, A>(mut fill: F, len: usize, alloc: A) -> Box<Self, A>
    where
        F: FnMut(usize) -> T,
        A: Allocator
    {
        let mut slice = unsafe {
            Box::new_uninit_slice_in(len, alloc).assume_init()
        };
        let ptr: *mut T = slice.as_mut_ptr();
        let mut i = 0;
        while i < len
        {
            unsafe {
                ptr.add(i).write(fill(i));
            }
            i += 1;
        }
        slice
    }
    #[cfg(feature = "std")]
    fn rfill_boxed_in<F, A>(mut fill: F, len: usize, alloc: A) -> Box<Self, A>
    where
        F: FnMut(usize) -> T,
        A: Allocator
    {
        let mut slice = unsafe {
            Box::new_uninit_slice_in(len, alloc).assume_init()
        };
        if len != 0
        {
            let ptr: *mut T = slice.as_mut_ptr();
            let mut i = len - 1;
            loop
            {
                unsafe {
                    ptr.add(i).write(fill(i));
                }
                if i == 0
                {
                    break
                }
                i -= 1;
            }
        }
        slice
    }
    
    fn shift_many_left(&mut self, mut items: &mut [T])
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
    
    fn bit_reverse_permutation(&mut self)
    {
        let len = self.len();
        assert!(len.is_power_of_two(), "Length must be a power of two.");

        let mut i = 0;
        while i < len/2
        {
            let j = i.reverse_bits() >> (len.leading_zeros() + 1);
            if i != j
            {
                unsafe {
                    core::ptr::swap_nonoverlapping(self.as_mut_ptr().add(i), self.as_mut_ptr().add(j), 1);
                }
            }
            i += 1;
        }
    }
}

#[test]
fn test()
{
    let mut a = [2, 1, 0];
    let mut b = 3;

    a.shift_right(&mut b);
    
    println!("a = {:?}", a);
    println!("b = {:?}", b);
}