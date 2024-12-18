
use core::cmp::Ordering;
use core::marker::Destruct;
use core::mem::MaybeUninit;

use core::{cmp::{Ord, PartialOrd}, ops::{AddAssign, BitAndAssign, BitOrAssign, BitXorAssign, DivAssign, FnMut, MulAssign, Neg, Not, RemAssign, ShlAssign, ShrAssign, SubAssign, AsyncFn}};

pub use slice_trait::*;

use crate::{is_power_of, Padded};

#[cfg(feature = "alloc")]
use crate::{Actions, ErrorRace};

#[cfg(feature = "alloc")]
use core::future::Future;

#[deprecated(note = "This will be removed once it can be implemented as a method")]
#[inline]
pub const fn split_len(len: usize, mid: usize) -> (usize, usize)
{
    assert!(mid <= len);
    (mid, len - mid)
}
#[deprecated(note = "This will be removed once it can be implemented as a method")]
#[inline]
pub const fn rsplit_len(len: usize, mid: usize) -> (usize, usize)
{
    assert!(mid <= len);
    (len - mid, mid)
}

#[deprecated(note = "This will be removed once it can be implemented as a method")]
#[inline]
pub const fn split_at<T>(slice: &[T], mid: usize) -> (&[T], &[T])
{
    slice.split_at(mid)
}

#[deprecated(note = "This will be removed once it can be implemented as a method")]
#[inline]
pub const fn split_at_mut<T>(slice: &mut [T], mid: usize) -> (&mut [T], &mut [T])
{
    slice.split_at_mut(mid)
}

#[deprecated(note = "This will be removed once it can be implemented as a method")]
#[inline]
pub const fn rsplit_at<T>(slice: &[T], mid: usize) -> (&[T], &[T])
{
    assert!(mid <= slice.len());
    crate::split_at(slice, slice.len() - mid)
}

#[deprecated(note = "This will be removed once it can be implemented as a method")]
#[inline]
pub const fn rsplit_at_mut<T>(slice: &mut [T], mid: usize) -> (&mut [T], &mut [T])
{
    assert!(mid <= slice.len());
    crate::split_at_mut(slice, slice.len() - mid)
}

#[deprecated(note = "This will be removed once it can be implemented as a method")]
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
#[deprecated(note = "This will be removed once it can be implemented as a method")]
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

//#[const_trait]
pub trait SliceOps<T>: Slice<Item = T>
{
    /// Differentiates the slice in-place.
    /// 
    /// Each value will be subtracted by the previous value.
    /// 
    /// It's assumed that the first value of the slice is preceded by zeros.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use slice_ops::*;
    /// 
    /// let mut x = [1, 5, 5, 6, 2, -1, 0, 0, 0];
    /// 
    /// x.differentiate();
    /// 
    /// assert_eq!(x, [1, 4, 0, 1, -4, -3, 1, 0, 0]);
    /// ```
    fn differentiate(&mut self)
    where
        T: SubAssign<T> + Copy;

    /// Integrates the slice in-place.
    /// 
    /// Each value will be added by the sum of all previous values.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use slice_ops::*;
    /// 
    /// let mut x = [1, 5, 5, 6, 2, -1, 0, 0, 0];
    /// 
    /// x.integrate();
    /// 
    /// assert_eq!(x, [1, 6, 11, 17, 19, 18, 18, 18, 18]);
    /// ```
    fn integrate(&mut self)
    where
        T: AddAssign<T> + Copy;

    /// Performs a linear search for the first value that equals `x`.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use slice_ops::*;
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
    /// # Examples
    /// 
    /// ```rust
    /// use slice_ops::*;
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
    /// # Examples
    /// 
    /// ```rust
    /// use slice_ops::*;
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
    /// # Examples
    /// 
    /// ```rust
    /// use slice_ops::*;
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
    /// # Examples
    /// 
    /// ```rust
    /// use slice_ops::*;
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
    /// # Examples
    /// 
    /// ```rust
    /// use slice_ops::*;
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
        
    /// Performs an argument reduction, finding the final righthand operand for which the comparison yields true.
    /// 
    /// # Examples
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
    /// # Examples
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

    /// Finds the index of the maximum value in the slice.
    /// 
    /// If there are multiple maxima, only the first will have its index returned.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use slice_ops::*;
    /// 
    /// //                v
    /// let x = [1, 5, 5, 6, 2, -1, 0, -4, -1, 6];
    /// 
    /// let i = x.argmax().unwrap();
    /// 
    /// assert_eq!(i, 3);
    /// ```
    fn argmax(&self) -> Option<usize>
    where
        T: PartialOrd<T>;
    /// Finds the index of the minimum value in the slice.
    /// 
    /// If there are multiple minimums, only the first will have its index returned.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use slice_ops::*;
    /// 
    /// //                              v
    /// let x = [1, 5, 5, 6, 2, -1, 0, -4, -1, 6];
    /// 
    /// let i = x.argmin().unwrap();
    /// 
    /// assert_eq!(i, 7);
    /// ```
    fn argmin(&self) -> Option<usize>
    where
        T: PartialOrd<T>;
    /// Finds the index of the maximum value in the slice, given a comparison predicate.
    /// 
    /// If there are multiple maxima, only the first will have its index returned.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use slice_ops::*;
    /// 
    /// //                v
    /// let x = [1, 5, 5, 6, 2, -1, 0, -4, -1, 6];
    /// 
    /// let f = Ord::cmp;
    /// 
    /// let i = x.argmax_by(f).unwrap();
    /// 
    /// assert_eq!(i, 3);
    /// ```
    fn argmax_by<'a, F>(&'a self, f: F) -> Option<usize>
    where
        F: FnMut(&'a T, &'a T) -> Ordering /*+ ~const Destruct*/,
        T: 'a;
    /// Finds the index of the minimum value in the slice, given a comparison predicate.
    /// 
    /// If there are multiple minimums, only the first will have its index returned.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use slice_ops::*;
    /// 
    /// //                              v
    /// let x = [1, 5, 5, 6, 2, -1, 0, -4, -1, 6];
    /// 
    /// let f = Ord::cmp;
    /// 
    /// let i = x.argmin_by(f).unwrap();
    /// 
    /// assert_eq!(i, 7);
    /// ```
    fn argmin_by<'a, F>(&'a self, f: F) -> Option<usize>
    where
        F: FnMut(&'a T, &'a T) -> Ordering /*+ ~const Destruct*/,
        T: 'a;
    /// Finds the index of the maximum key in the slice, given a hashing function.
    /// 
    /// If there are multiple maxima, only the first will have its index returned.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use slice_ops::*;
    /// 
    /// //                       v
    /// let x = ["1", "5", "5", "6", "2", "-1", "0", "-4", "-1", "6"];
    /// 
    /// let f = |&e| i32::from_str_radix(e, 10).unwrap();
    /// 
    /// let i = x.argmax_by_key(f).unwrap();
    /// 
    /// assert_eq!(i, 3);
    /// ```
    fn argmax_by_key<'a, B, F>(&'a self, f: F) -> Option<usize>
    where
        F: FnMut(&'a T) -> B /*+ ~const Destruct*/,
        B: PartialOrd,
        T: 'a;
    /// Finds the index of the minimum key in the slice, given a hashing function.
    /// 
    /// If there are multiple minimums, only the first will have its index returned.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use slice_ops::*;
    /// 
    /// //                                  v
    /// let x = ["1", "5", "5", "6", "2", "-1", "0", "-4", "-1", "6"];
    /// 
    /// let f = |&e| i32::from_str_radix(e, 10).unwrap();
    /// 
    /// let i = x.argmin_by_key(f).unwrap();
    /// 
    /// assert_eq!(i, 7);
    /// ```
    fn argmin_by_key<'a, B, F>(&'a self, f: F) -> Option<usize>
    where
        F: FnMut(&'a T) -> B /*+ ~const Destruct*/,
        B: PartialOrd,
        T: 'a;
        
    /// Visits each element once, from left to right.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use slice_ops::*;
    /// 
    /// let x = [1, 2, 3, 4, 5, 6, 7, 8];
    /// 
    /// let mut i = 0;
    /// 
    /// x.visit(|&e| {
    ///     i += 1;
    ///     assert_eq!(i, e)
    /// });
    /// ```
    fn visit<'a, F>(&'a self, visitor: F)
    where
        F: FnMut(&'a T) /*+ ~const Destruct*/,
        T: 'a;
    /// Mutably visits each element once, from left to right.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use slice_ops::*;
    /// 
    /// let mut x = [0; 8];
    /// 
    /// let mut i = 0;
    /// 
    /// x.visit_mut(|e| {
    ///     i += 1;
    ///     *e = i;
    /// });
    /// 
    /// assert_eq!(x, [1, 2, 3, 4, 5, 6, 7, 8]);
    /// ```
    fn visit_mut<'a, F>(&'a mut self, visitor: F)
    where
        F: FnMut(&'a mut T) /*+ ~const Destruct*/,
        T: 'a;
    /// Visits each element once, from left to right, or short-circuits if visitor returns error.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use slice_ops::*;
    /// 
    /// let x = [1, 2, 3, 4, 5, 6, 7, 8];
    /// 
    /// let mut i = 0;
    /// 
    /// let result = x.try_visit(|&e| {
    ///     i += 1;
    ///     if i > 4
    ///     {
    ///         return Err(i)
    ///     }
    ///     assert_eq!(i, e);
    ///     Ok(())
    /// });
    /// 
    /// assert_eq!(result, Err(5));
    /// ```
    fn try_visit<'a, E, F>(&'a self, visitor: F) -> Result<(), E>
    where
        F: FnMut(&'a T) -> Result<(), E> /*+ ~const Destruct*/,
        T: 'a;
    fn try_visit_mut<'a, E, F>(&'a mut self, visitor: F) -> Result<(), E>
    where
        F: FnMut(&'a mut T) -> Result<(), E> /*+ ~const Destruct*/,
        T: 'a;
        
    fn rvisit<'a, F>(&'a self, visitor: F)
    where
        F: FnMut(&'a T) /*+ ~const Destruct*/,
        T: 'a;
    fn rvisit_mut<'a, F>(&'a mut self, visitor: F)
    where
        F: FnMut(&'a mut T) /*+ ~const Destruct*/,
        T: 'a;
    fn try_rvisit<'a, E, F>(&'a self, visitor: F) -> Result<(), E>
    where
        F: FnMut(&'a T) -> Result<(), E> /*+ ~const Destruct*/,
        T: 'a;
    fn try_rvisit_mut<'a, E, F>(&'a mut self, visitor: F) -> Result<(), E>
    where
        F: FnMut(&'a mut T) -> Result<(), E> /*+ ~const Destruct*/,
        T: 'a;
        
    #[cfg(feature = "alloc")]
    async fn visit_async<'a, F>(&'a self, visitor: F)
    where
        F: AsyncFn(&'a T) /*+ ~const Destruct*/,
        T: 'a;
    #[cfg(feature = "alloc")]
    async fn visit_mut_async<'a, F>(&'a mut self, visitor: F)
    where
        F: AsyncFn(&'a mut T) /*+ ~const Destruct*/,
        T: 'a;
    #[cfg(feature = "alloc")]
    async fn try_visit_async<'a, E, F>(&'a self, visitor: F) -> Result<(), E>
    where
        F: AsyncFn(&'a T) -> Result<(), E> /*+ ~const Destruct*/,
        T: 'a;
    #[cfg(feature = "alloc")]
    async fn try_visit_mut_async<'a, E, F>(&'a mut self, visitor: F) -> Result<(), E>
    where
        F: AsyncFn(&'a mut T) -> Result<(), E> /*+ ~const Destruct*/,
        T: 'a;

    /// Adds `rhs` to each element in the slice.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use slice_ops::*;
    /// 
    /// let mut x = [1, 2, 3, 4, 5, 6, 7, 8];
    /// 
    /// x.add_assign_all(2);
    ///    
    /// assert_eq!(x, [3, 4, 5, 6, 7, 8, 9, 10]);
    /// ```
    fn add_assign_all<Rhs>(&mut self, rhs: Rhs)
    where
        T: AddAssign<Rhs>,
        Rhs: Copy;
    /// Subtracts each element in the slice by `rhs`.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use slice_ops::*;
    /// 
    /// let mut x = [1, 2, 3, 4, 5, 6, 7, 8];
    /// 
    /// x.sub_assign_all(2);
    ///    
    /// assert_eq!(x, [-1, 0, 1, 2, 3, 4, 5, 6]);
    /// ```
    fn sub_assign_all<Rhs>(&mut self, rhs: Rhs)
    where
        T: SubAssign<Rhs>,
        Rhs: Copy;
    /// Multiplies `rhs` to each element in the slice.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use slice_ops::*;
    /// 
    /// let mut x = [1, 2, 3, 4, 5, 6, 7, 8];
    /// 
    /// x.mul_assign_all(2);
    ///    
    /// assert_eq!(x, [2, 4, 6, 8, 10, 12, 14, 16]);
    /// ```
    fn mul_assign_all<Rhs>(&mut self, rhs: Rhs)
    where
        T: MulAssign<Rhs>,
        Rhs: Copy;
    /// Divides each element in the slice by `rhs`.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use slice_ops::*;
    /// 
    /// let mut x = [1, 2, 3, 4, 5, 6, 7, 8];
    /// 
    /// x.div_assign_all(2);
    ///    
    /// assert_eq!(x, [0, 1, 1, 2, 2, 3, 3, 4]);
    /// ```
    fn div_assign_all<Rhs>(&mut self, rhs: Rhs)
    where
        T: DivAssign<Rhs>,
        Rhs: Copy;
    /// Replaces each value in the slice with its remainder when divided by `rhs`.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use slice_ops::*;
    /// 
    /// let mut x = [1, 2, 3, 4, 5, 6, 7, 8];
    /// 
    /// x.rem_assign_all(2);
    ///    
    /// assert_eq!(x, [1, 0, 1, 0, 1, 0, 1, 0]);
    /// ```
    fn rem_assign_all<Rhs>(&mut self, rhs: Rhs)
    where
        T: RemAssign<Rhs>,
        Rhs: Copy;
    /// Shifts each element to the left by `rhs`.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use slice_ops::*;
    /// 
    /// let mut x = [0b1, 0b10, 0b11, 0b100, 0b101, 0b110, 0b111, 0b1000];
    /// 
    /// x.shl_assign_all(2);
    ///    
    /// assert_eq!(x, [0b100, 0b1000, 0b1100, 0b10000, 0b10100, 0b11000, 0b11100, 0b100000]);
    /// ```
    fn shl_assign_all<Rhs>(&mut self, rhs: Rhs)
    where
        T: ShlAssign<Rhs>,
        Rhs: Copy;
    /// Shifts each element to the right by `rhs`.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use slice_ops::*;
    /// 
    /// let mut x = [0b1, 0b10, 0b11, 0b100, 0b101, 0b110, 0b111, 0b1000];
    /// 
    /// x.shr_assign_all(2);
    ///    
    /// assert_eq!(x, [0b0, 0b0, 0b0, 0b1, 0b1, 0b1, 0b1, 0b10]);
    /// ```
    fn shr_assign_all<Rhs>(&mut self, rhs: Rhs)
    where
        T: ShrAssign<Rhs>,
        Rhs: Copy;
    /// Performs a bitwise OR on each element using `rhs`.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use slice_ops::*;
    /// 
    /// let mut x = [0b1, 0b10, 0b11, 0b100, 0b101, 0b110, 0b111, 0b1000];
    /// 
    /// x.bitor_assign_all(0b10);
    ///    
    /// assert_eq!(x, [0b11, 0b10, 0b11, 0b110, 0b111, 0b110, 0b111, 0b1010]);
    /// ```
    fn bitor_assign_all<Rhs>(&mut self, rhs: Rhs)
    where
        T: BitOrAssign<Rhs>,
        Rhs: Copy;
    /// Performs a bitwise AND on each element using `rhs`.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use slice_ops::*;
    /// 
    /// let mut x = [0b1, 0b10, 0b11, 0b100, 0b101, 0b110, 0b111, 0b1000];
    /// 
    /// x.bitand_assign_all(0b10);
    ///    
    /// assert_eq!(x, [0b0, 0b10, 0b10, 0b0, 0b0, 0b10, 0b10, 0b0]);
    /// ```
    fn bitand_assign_all<Rhs>(&mut self, rhs: Rhs)
    where
        T: BitAndAssign<Rhs>,
        Rhs: Copy;
    /// Performs a bitwise XOR on each element using `rhs`.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use slice_ops::*;
    /// 
    /// let mut x = [0b1, 0b10, 0b11, 0b100, 0b101, 0b110, 0b111, 0b1000];
    /// 
    /// x.bitxor_assign_all(0b10);
    ///    
    /// assert_eq!(x, [0b11, 0b0, 0b1, 0b110, 0b111, 0b100, 0b101, 0b1010]);
    /// ```
    fn bitxor_assign_all<Rhs>(&mut self, rhs: Rhs)
    where
        T: BitXorAssign<Rhs>,
        Rhs: Copy;
        
    /// Negates each element in the slice.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use slice_ops::*;
    /// 
    /// let mut x = [1, 2, 3, 4, 5, 6, 7, 8];
    /// 
    /// x.neg_assign_all();
    ///    
    /// assert_eq!(x, [-1, -2, -3, -4, -5, -6, -7, -8]);
    /// ```
    fn neg_assign_all(&mut self)
    where
        T: Neg<Output = T>;
    /// Performs a logical NOT or bitwise NOT on each element in the slice.
    /// 
    /// Booleans will be treated with a logical NOT, while integers will be treated with a bitwise NOT.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use slice_ops::*;
    /// 
    /// let mut x = [true, false, true, false, true, false, true, true];
    /// 
    /// x.not_assign_all();
    ///    
    /// assert_eq!(x, [false, true, false, true, false, true, false, false]);
    /// ```
    fn not_assign_all(&mut self)
    where
        T: Not<Output = T>;
    
    /// Asyncronously adds `rhs` to each element in the slice.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use slice_ops::*;
    /// 
    /// # tokio_test::block_on(async {
    /// let mut x = [1, 2, 3, 4, 5, 6, 7, 8];
    /// 
    /// x.add_assign_all_async(2).await;
    ///    
    /// assert_eq!(x, [3, 4, 5, 6, 7, 8, 9, 10]);
    /// # });
    /// ```
    #[cfg(feature = "alloc")]
    async fn add_assign_all_async<Rhs>(&mut self, rhs: Rhs)
    where
        T: AddAssign<Rhs>,
        Rhs: Copy;
    /// Asyncronously subtracts each element in the slice by `rhs`.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use slice_ops::*;
    /// 
    /// # tokio_test::block_on(async {
    /// let mut x = [1, 2, 3, 4, 5, 6, 7, 8];
    /// 
    /// x.sub_assign_all_async(2).await;
    ///    
    /// assert_eq!(x, [-1, 0, 1, 2, 3, 4, 5, 6]);
    /// # });
    /// ```
    #[cfg(feature = "alloc")]
    async fn sub_assign_all_async<Rhs>(&mut self, rhs: Rhs)
    where
        T: SubAssign<Rhs>,
        Rhs: Copy;
    /// Asyncronously multiplies `rhs` to each element in the slice.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use slice_ops::*;
    /// 
    /// # tokio_test::block_on(async {
    /// let mut x = [1, 2, 3, 4, 5, 6, 7, 8];
    /// 
    /// x.mul_assign_all_async(2).await;
    ///    
    /// assert_eq!(x, [2, 4, 6, 8, 10, 12, 14, 16]);
    /// # });
    /// ```
    #[cfg(feature = "alloc")]
    async fn mul_assign_all_async<Rhs>(&mut self, rhs: Rhs)
    where
        T: MulAssign<Rhs>,
        Rhs: Copy;
    /// Asyncronously divides each element in the slice by `rhs`.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use slice_ops::*;
    /// 
    /// # tokio_test::block_on(async {
    /// let mut x = [1, 2, 3, 4, 5, 6, 7, 8];
    /// 
    /// x.div_assign_all_async(2).await;
    ///    
    /// assert_eq!(x, [0, 1, 1, 2, 2, 3, 3, 4]);
    /// # });
    /// ```
    #[cfg(feature = "alloc")]
    async fn div_assign_all_async<Rhs>(&mut self, rhs: Rhs)
    where
        T: DivAssign<Rhs>,
        Rhs: Copy;
    /// Asyncronously replaces each value in the slice with its remainder when divided by `rhs`.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use slice_ops::*;
    /// 
    /// # tokio_test::block_on(async {
    /// let mut x = [1, 2, 3, 4, 5, 6, 7, 8];
    /// 
    /// x.rem_assign_all_async(2).await;
    ///    
    /// assert_eq!(x, [1, 0, 1, 0, 1, 0, 1, 0]);
    /// # });
    /// ```
    #[cfg(feature = "alloc")]
    async fn rem_assign_all_async<Rhs>(&mut self, rhs: Rhs)
    where
        T: RemAssign<Rhs>,
        Rhs: Copy;
    /// Asyncronously shifts each element to the left by `rhs`.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use slice_ops::*;
    /// 
    /// # tokio_test::block_on(async {
    /// let mut x = [0b1, 0b10, 0b11, 0b100, 0b101, 0b110, 0b111, 0b1000];
    /// 
    /// x.shl_assign_all_async(2).await;
    ///    
    /// assert_eq!(x, [0b100, 0b1000, 0b1100, 0b10000, 0b10100, 0b11000, 0b11100, 0b100000]);
    /// # });
    /// ```
    #[cfg(feature = "alloc")]
    async fn shl_assign_all_async<Rhs>(&mut self, rhs: Rhs)
    where
        T: ShlAssign<Rhs>,
        Rhs: Copy;
    /// Asyncronously shifts each element to the right by `rhs`.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use slice_ops::*;
    /// 
    /// # tokio_test::block_on(async {
    /// let mut x = [0b1, 0b10, 0b11, 0b100, 0b101, 0b110, 0b111, 0b1000];
    /// 
    /// x.shr_assign_all_async(2).await;
    ///    
    /// assert_eq!(x, [0b0, 0b0, 0b0, 0b1, 0b1, 0b1, 0b1, 0b10]);
    /// # });
    /// ```
    #[cfg(feature = "alloc")]
    async fn shr_assign_all_async<Rhs>(&mut self, rhs: Rhs)
    where
        T: ShrAssign<Rhs>,
        Rhs: Copy;
    /// Asyncronously performs a bitwise OR on each element using `rhs`.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use slice_ops::*;
    /// 
    /// # tokio_test::block_on(async {
    /// let mut x = [0b1, 0b10, 0b11, 0b100, 0b101, 0b110, 0b111, 0b1000];
    /// 
    /// x.bitor_assign_all_async(0b10).await;
    ///    
    /// assert_eq!(x, [0b11, 0b10, 0b11, 0b110, 0b111, 0b110, 0b111, 0b1010]);
    /// # });
    /// ```
    #[cfg(feature = "alloc")]
    async fn bitor_assign_all_async<Rhs>(&mut self, rhs: Rhs)
    where
        T: BitOrAssign<Rhs>,
        Rhs: Copy;
    /// Asyncronously performs a bitwise AND on each element using `rhs`.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use slice_ops::*;
    /// 
    /// # tokio_test::block_on(async {
    /// let mut x = [0b1, 0b10, 0b11, 0b100, 0b101, 0b110, 0b111, 0b1000];
    /// 
    /// x.bitand_assign_all_async(0b10).await;
    ///    
    /// assert_eq!(x, [0b0, 0b10, 0b10, 0b0, 0b0, 0b10, 0b10, 0b0]);
    /// # });
    /// ```
    #[cfg(feature = "alloc")]
    async fn bitand_assign_all_async<Rhs>(&mut self, rhs: Rhs)
    where
        T: BitAndAssign<Rhs>,
        Rhs: Copy;
    /// Asyncronously performs a bitwise XOR on each element using `rhs`.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use slice_ops::*;
    /// 
    /// # tokio_test::block_on(async {
    /// let mut x = [0b1, 0b10, 0b11, 0b100, 0b101, 0b110, 0b111, 0b1000];
    /// 
    /// x.bitxor_assign_all_async(0b10).await;
    ///    
    /// assert_eq!(x, [0b11, 0b0, 0b1, 0b110, 0b111, 0b100, 0b101, 0b1010]);
    /// # });
    /// ```
    #[cfg(feature = "alloc")]
    async fn bitxor_assign_all_async<Rhs>(&mut self, rhs: Rhs)
    where
        T: BitXorAssign<Rhs>,
        Rhs: Copy;
        
    /// Asyncronously negates each element in the slice.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use slice_ops::*;
    /// 
    /// # tokio_test::block_on(async {
    /// let mut x = [1, 2, 3, 4, 5, 6, 7, 8];
    /// 
    /// x.neg_assign_all_async().await;
    ///    
    /// assert_eq!(x, [-1, -2, -3, -4, -5, -6, -7, -8]);
    /// # });
    /// ```
    #[cfg(feature = "alloc")]
    async fn neg_assign_all_async(&mut self)
    where
        T: Neg<Output = T>;
    /// Asyncronously performs a logical NOT or bitwise NOT on each element in the slice.
    /// 
    /// Booleans will be treated with a logical NOT, while integers will be treated with a bitwise NOT.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use slice_ops::*;
    /// 
    /// # tokio_test::block_on(async {
    /// let mut x = [true, false, true, false, true, false, true, true];
    /// 
    /// x.not_assign_all_async().await;
    ///    
    /// assert_eq!(x, [false, true, false, true, false, true, false, false]);
    /// # });
    /// ```
    #[cfg(feature = "alloc")]
    async fn not_assign_all_async(&mut self)
    where
        T: Not<Output = T>;

    /// Shifts the entire slice as a SISO shift register with mutliple values to the left. The output is given in-place in `ìtems`.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use slice_ops::*;
    /// 
    /// let mut register = [9, 8, 7, 6, 5, 4];
    /// let mut io = [3, 2, 1];
    /// 
    /// register.shift_many_left(&mut io);
    /// 
    /// assert_eq!(register, [6, 5, 4, 3, 2, 1]);
    /// assert_eq!(io, [9, 8, 7]);
    /// ```
    fn shift_many_left(&mut self, items: &mut [T]);
    
    /// Shifts the entire slice as a SISO shift register with mutliple values to the left. The output is given in-place in `ìtems`.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use slice_ops::*;
    /// 
    /// let mut register = [4, 5, 6, 7, 8, 9];
    /// let mut io = [1, 2, 3];
    /// 
    /// register.shift_many_right(&mut io);
    /// 
    /// assert_eq!(register, [1, 2, 3, 4, 5, 6]);
    /// assert_eq!(io, [7, 8, 9]);
    /// ```
    fn shift_many_right(&mut self, items: &mut [T]);
    
    /// Shifts the entire slice as a SISO shift register to the left. The output is given in-place in `ìtem`.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use slice_ops::*;
    /// 
    /// let mut register = [4, 3, 2];
    /// let mut io = 1;
    /// 
    /// register.shift_left(&mut io);
    /// 
    /// assert_eq!(register, [3, 2, 1]);
    /// assert_eq!(io, 4);
    /// ```
    fn shift_left(&mut self, item: &mut T);

    /// Shifts the entire slice as a SISO shift register to the right. The output is given in-place in `ìtem`.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use slice_ops::*;
    /// 
    /// let mut register = [2, 3, 4];
    /// let mut io = 1;
    /// 
    /// register.shift_right(&mut io);
    /// 
    /// assert_eq!(register, [1, 2, 3]);
    /// assert_eq!(io, 4);
    /// ```
    fn shift_right(&mut self, item: &mut T);

    /// Returns the lengths before and after a certain index, as if split.
    /// 
    /// # Example
    /// 
    /// ```
    /// use slice_ops::*;
    /// 
    /// let values = [1, 2, 3, 4];
    /// 
    /// let (len_left, len_right) = values.split_len(1);
    /// 
    /// assert_eq!(len_left, 1);
    /// assert_eq!(len_right, 3);
    /// assert_eq!(len_left + len_right, values.len());
    /// ```
    fn split_len(&self, mid: usize) -> (usize, usize);

    /// Returns the lengths before and after a certain index, as if split from the right.
    /// 
    /// # Example
    /// 
    /// ```
    /// use slice_ops::*;
    /// 
    /// let values = [1, 2, 3, 4];
    /// 
    /// let (len_left, len_right) = values.rsplit_len(3);
    /// 
    /// assert_eq!(len_left, 1);
    /// assert_eq!(len_right, 3);
    /// assert_eq!(len_left + len_right, values.len());
    /// ```
    fn rsplit_len(&self, mid: usize) -> (usize, usize);

    fn rsplit_at(&self, mid: usize) -> (&[T], &[T]);
    fn rsplit_at_mut(&mut self, mid: usize) -> (&mut [T], &mut [T]);

    /// Spreads elements equally across `M` slices.
    /// Slices will have equal length only if the operand slice's length is divisible by `M`.
    /// 
    /// # Examples
    /// Take, for instance, that we want to divide a slice into odd and even elements:
    /// ```rust
    /// #![feature(generic_const_exprs)]
    /// 
    /// use slice_ops::*;
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
    /// use slice_ops::*;
    /// 
    /// let mut arr = ["1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14", "15"];
    /// let slice = arr.as_mut_slice();
    /// 
    /// let [_, _, fizz] = slice.spread_chunks_mut::<3>();
    /// assert_eq!(fizz, ["3", "6", "9", "12", "15"]);
    /// for fizz in fizz.iter_mut()
    /// {
    ///     **fizz = "fizz";
    /// }
    /// 
    /// let [_, _, _, _, buzz] = slice.spread_chunks_mut::<5>();
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
    /// arr.bit_rev_permutation();
    /// 
    /// assert_eq!(arr, [0b000, 0b100, 0b010, 0b110, 0b001, 0b101, 0b011, 0b111])
    /// ```
    fn bit_rev_permutation(&mut self);

    /// Performs the digit-reverse permutation with any radix. Length must be a power of the radix.
    /// 
    /// # Example
    /// ```rust
    /// use slice_ops::*;
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

    /// Returns a trimmed subslice, trimmed from both ends using a trimming predicate.
    /// 
    /// `trim` should return `true` for each element that should be trimmed.
    /// 
    /// ```rust
    /// use slice_ops::*;
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
    /// ```rust
    /// use slice_ops::*;
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
    /// ```rust
    /// use slice_ops::*;
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
    /// ```rust
    /// use slice_ops::*;
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
    /// ```rust
    /// use slice_ops::*;
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
    /// ```rust
    /// use slice_ops::*;
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

/// Waiting for `core::maker::TriviallyDrop` or some equivalent to arrive before making this const again...
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

    fn argmax(&self) -> Option<usize>
    where
        T: PartialOrd
    {
        self.argreduce(PartialOrd::gt)
    }
    fn argmin(&self) -> Option<usize>
    where
        T: PartialOrd
    {
        self.argreduce(PartialOrd::lt)
    }
    fn argmax_by<'a, F>(&'a self, mut f: F) -> Option<usize>
    where
        F: FnMut(&'a T, &'a T) -> Ordering,
        T: 'a
    {
        self.argreduce(|a, b| matches!(f(a, b), Ordering::Greater))
    }
    fn argmin_by<'a, F>(&'a self, mut f: F) -> Option<usize>
    where
        F: FnMut(&'a T, &'a T) -> Ordering,
        T: 'a
    {
        self.argreduce(|a, b| matches!(f(a, b), Ordering::Less))
    }
    fn argmax_by_key<'a, B, F>(&'a self, f: F) -> Option<usize>
    where
        F: FnMut(&'a T) -> B,
        B: PartialOrd,
        T: 'a
    {
        self.argreduce_key(PartialOrd::gt, f)
    }
    fn argmin_by_key<'a, B, F>(&'a self, f: F) -> Option<usize>
    where
        F: FnMut(&'a T) -> B,
        B: PartialOrd,
        T: 'a
    {
        self.argreduce_key(PartialOrd::lt, f)
    }

    fn visit<'a, F>(&'a self, mut visitor: F)
    where
        F: FnMut(&'a T),
        T: 'a
    {
        let l = self.len();
        let mut i = 0;
        while i < l
        {
            visitor(&self[i]);
            i += 1;
        }
    }
    fn visit_mut<'a, F>(&'a mut self, mut visitor: F)
    where
        F: FnMut(&'a mut T),
        T: 'a
    {
        let l = self.len();
        let mut i = 0;
        while i < l
        {
            visitor(unsafe {
                core::mem::transmute(&mut self[i])
            });
            i += 1;
        }
    }
    fn try_visit<'a, E, F>(&'a self, mut visitor: F) -> Result<(), E>
    where
        F: FnMut(&'a T) -> Result<(), E>,
        T: 'a
    {
        let l = self.len();
        let mut i = 0;
        while i < l
        {
            visitor(&self[i])?;
            i += 1;
        }
        Ok(())
    }
    fn try_visit_mut<'a, E, F>(&'a mut self, mut visitor: F) -> Result<(), E>
    where
        F: FnMut(&'a mut T) -> Result<(), E>,
        T: 'a
    {
        let l = self.len();
        let mut i = 0;
        while i < l
        {
            visitor(unsafe {
                core::mem::transmute(&mut self[i])
            })?;
            i += 1;
        }
        Ok(())
    }
        
    fn rvisit<'a, F>(&'a self, mut visitor: F)
    where
        F: FnMut(&'a T) /*+ ~const Destruct*/,
        T: 'a
    {
        let l = self.len();
        let mut i = l;
        while i > 0
        {
            i -= 1;
            visitor(&self[i]);
        }
    }
    fn rvisit_mut<'a, F>(&'a mut self, mut visitor: F)
    where
        F: FnMut(&'a mut T) /*+ ~const Destruct*/,
        T: 'a
    {
        let l = self.len();
        let mut i = l;
        while i > 0
        {
            i -= 1;
            visitor(unsafe {
                core::mem::transmute(&mut self[i])
            });
        }
    }
    fn try_rvisit<'a, E, F>(&'a self, mut visitor: F) -> Result<(), E>
    where
        F: FnMut(&'a T) -> Result<(), E> /*+ ~const Destruct*/,
        T: 'a
    {
        let l = self.len();
        let mut i = l;
        while i > 0
        {
            i -= 1;
            visitor(&self[i])?;
        }
        Ok(())
    }
    fn try_rvisit_mut<'a, E, F>(&'a mut self, mut visitor: F) -> Result<(), E>
    where
        F: FnMut(&'a mut T) -> Result<(), E> /*+ ~const Destruct*/,
        T: 'a
    {
        let l = self.len();
        let mut i = l;
        while i > 0
        {
            i -= 1;
            visitor(unsafe {
                core::mem::transmute(&mut self[i])
            })?;
        }
        Ok(())
    }
    
    #[cfg(feature = "alloc")]
    async fn visit_async<'a, F>(&'a self, visitor: F)
    where
        F: AsyncFn(&'a T),
        T: 'a
    {
        Actions::new(self.iter().map(|x| visitor(x))).await
    }
    #[cfg(feature = "alloc")]
    async fn visit_mut_async<'a, F>(&'a mut self, visitor: F)
    where
        F: AsyncFn(&'a mut T),
        T: 'a
    {
        Actions::new(self.iter_mut().map(|x| visitor(x))).await
    }
    #[cfg(feature = "alloc")]
    async fn try_visit_async<'a, E, F>(&'a self, visitor: F) -> Result<(), E>
    where
        F: AsyncFn(&'a T) -> Result<(), E>,
        T: 'a
    {
        ErrorRace::new(self.iter().map(|x| visitor(x))).await
    }
    #[cfg(feature = "alloc")]
    async fn try_visit_mut_async<'a, E, F>(&'a mut self, visitor: F) -> Result<(), E>
    where
        F: AsyncFn(&'a mut T) -> Result<(), E>,
        T: 'a
    {
        ErrorRace::new(self.iter_mut().map(|x| visitor(x))).await
    }

    fn add_assign_all<Rhs>(&mut self, rhs: Rhs)
    where
        T: AddAssign<Rhs>,
        Rhs: Copy
    {
        self.visit_mut(|x| *x += rhs)
    }
    fn sub_assign_all<Rhs>(&mut self, rhs: Rhs)
    where
        T: SubAssign<Rhs>,
        Rhs: Copy
    {
        self.visit_mut(|x| *x -= rhs)
    }
    fn mul_assign_all<Rhs>(&mut self, rhs: Rhs)
    where
        T: MulAssign<Rhs>,
        Rhs: Copy
    {
        self.visit_mut(|x| *x *= rhs)
    }
    fn div_assign_all<Rhs>(&mut self, rhs: Rhs)
    where
        T: DivAssign<Rhs>,
        Rhs: Copy
    {
        self.visit_mut(|x| *x /= rhs)
    }
    fn rem_assign_all<Rhs>(&mut self, rhs: Rhs)
    where
        T: RemAssign<Rhs>,
        Rhs: Copy
    {
        self.visit_mut(|x| *x %= rhs)
    }
    fn shl_assign_all<Rhs>(&mut self, rhs: Rhs)
    where
        T: ShlAssign<Rhs>,
        Rhs: Copy
    {
        self.visit_mut(|x| *x <<= rhs)
    }
    fn shr_assign_all<Rhs>(&mut self, rhs: Rhs)
    where
        T: ShrAssign<Rhs>,
        Rhs: Copy
    {
        self.visit_mut(|x| *x >>= rhs)
    }
    fn bitor_assign_all<Rhs>(&mut self, rhs: Rhs)
    where
        T: BitOrAssign<Rhs>,
        Rhs: Copy
    {
        self.visit_mut(|x| *x |= rhs)
    }
    fn bitand_assign_all<Rhs>(&mut self, rhs: Rhs)
    where
        T: BitAndAssign<Rhs>,
        Rhs: Copy
    {
        self.visit_mut(|x| *x &= rhs)
    }
    fn bitxor_assign_all<Rhs>(&mut self, rhs: Rhs)
    where
        T: BitXorAssign<Rhs>,
        Rhs: Copy
    {
        self.visit_mut(|x| *x ^= rhs)
    }
    
    fn neg_assign_all(&mut self)
    where
        T: Neg<Output = T>
    {
        self.visit_mut(|x| unsafe {
            core::ptr::write(x, -core::ptr::read(x))
        })
    }
    fn not_assign_all(&mut self)
    where
        T: Not<Output = T>
    {
        self.visit_mut(|x| unsafe {
            core::ptr::write(x, !core::ptr::read(x))
        })
    }
    
    #[cfg(feature = "alloc")]
    async fn add_assign_all_async<Rhs>(&mut self, rhs: Rhs)
    where
        T: AddAssign<Rhs>,
        Rhs: Copy
    {
        self.visit_mut_async(async |x| *x += rhs).await
    }
    #[cfg(feature = "alloc")]
    async fn sub_assign_all_async<Rhs>(&mut self, rhs: Rhs)
    where
        T: SubAssign<Rhs>,
        Rhs: Copy
    {
        self.visit_mut_async(async |x| *x -= rhs).await
    }
    #[cfg(feature = "alloc")]
    async fn mul_assign_all_async<Rhs>(&mut self, rhs: Rhs)
    where
        T: MulAssign<Rhs>,
        Rhs: Copy
    {
        self.visit_mut_async(async |x| *x *= rhs).await
    }
    #[cfg(feature = "alloc")]
    async fn div_assign_all_async<Rhs>(&mut self, rhs: Rhs)
    where
        T: DivAssign<Rhs>,
        Rhs: Copy
    {
        self.visit_mut_async(async |x| *x /= rhs).await
    }
    #[cfg(feature = "alloc")]
    async fn rem_assign_all_async<Rhs>(&mut self, rhs: Rhs)
    where
        T: RemAssign<Rhs>,
        Rhs: Copy
    {
        self.visit_mut_async(async |x| *x %= rhs).await
    }
    #[cfg(feature = "alloc")]
    async fn shl_assign_all_async<Rhs>(&mut self, rhs: Rhs)
    where
        T: ShlAssign<Rhs>,
        Rhs: Copy
    {
        self.visit_mut_async(async |x| *x <<= rhs).await
    }
    #[cfg(feature = "alloc")]
    async fn shr_assign_all_async<Rhs>(&mut self, rhs: Rhs)
    where
        T: ShrAssign<Rhs>,
        Rhs: Copy
    {
        self.visit_mut_async(async |x| *x >>= rhs).await
    }
    #[cfg(feature = "alloc")]
    async fn bitor_assign_all_async<Rhs>(&mut self, rhs: Rhs)
    where
        T: BitOrAssign<Rhs>,
        Rhs: Copy
    {
        self.visit_mut_async(async |x| *x |= rhs).await
    }
    #[cfg(feature = "alloc")]
    async fn bitand_assign_all_async<Rhs>(&mut self, rhs: Rhs)
    where
        T: BitAndAssign<Rhs>,
        Rhs: Copy
    {
        self.visit_mut_async(async |x| *x &= rhs).await
    }
    #[cfg(feature = "alloc")]
    async fn bitxor_assign_all_async<Rhs>(&mut self, rhs: Rhs)
    where
        T: BitXorAssign<Rhs>,
        Rhs: Copy
    {
        self.visit_mut_async(async |x| *x ^= rhs).await
    }
        
    #[cfg(feature = "alloc")]
    async fn neg_assign_all_async(&mut self)
    where
        T: Neg<Output = T>
    {
        self.visit_mut_async(async |x| unsafe {
            core::ptr::write(x, -core::ptr::read(x))
        }).await
    }
    #[cfg(feature = "alloc")]
    async fn not_assign_all_async(&mut self)
    where
        T: Not<Output = T>
    {
        self.visit_mut_async(async |x| unsafe {
            core::ptr::write(x, !core::ptr::read(x))
        }).await
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