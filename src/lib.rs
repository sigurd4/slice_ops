#![cfg_attr(not(test), no_std)]

#![feature(const_trait_impl)]
#![feature(const_slice_split_at_mut)]
#![feature(const_mut_refs)]

pub use slice_trait::*;

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
}

#[cfg(test)]
#[test]
fn test()
{
    let a = [1, 2];

    let ar: &[u8] = &a;

    let _split = ar.rsplit_array_ref2::<2>();
}