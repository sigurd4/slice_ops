
use core::mem::MaybeUninit;

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
    fn spread_mut<const M: usize>(&mut self) -> [&mut [Padded<T, M>]; M]
    where
        [(); M - 1]:;
}

#[test]
fn test()
{
    let arr = [1, 2, 3];
    let slice = arr.as_slice();

    let [a, b] = slice.spread_ref();

    assert_eq!(a, [1, 3]);
    assert_eq!(b, [2]);
}

impl<T> const SliceOps<T> for [T]
{
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
}