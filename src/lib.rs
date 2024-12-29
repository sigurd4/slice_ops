#![cfg_attr(not(test), no_std)]
#![feature(const_trait_impl)]
#![feature(slice_from_ptr_range)]
#![feature(allocator_api)]
#![feature(const_eval_select)]
#![feature(const_swap_nonoverlapping)]
#![feature(const_slice_from_ptr_range)]
#![feature(const_destruct)]
#![feature(unboxed_closures)]
#![allow(async_fn_in_trait)]
#![allow(deprecated)]
#![allow(internal_features)]
#![feature(maybe_uninit_uninit_array)]
#![feature(maybe_uninit_array_assume_init)]
#![feature(let_chains)]
#![feature(generic_const_exprs)]
#![feature(core_intrinsics)]

//! Provides many useful utility methods for slices.
//!
//! # integrate / differentiate
//!
//! - [`integrate`](crate::ops::SliceIntegrate::integrate)
//! - [`differentiate`](crate::ops::SliceDifferentiate::differentiate)
//!
//! ```rust
//! use slice_ops::ops::*;
//!
//! let mut x = [1, 5, 5, 6, 2, -1, 0, 0, 0];
//!
//! x.differentiate();
//!
//! assert_eq!(x, [1, 4, 0, 1, -4, -3, 1, 0, 0]);
//!
//! x.integrate();
//!
//! assert_eq!(x, [1, 5, 5, 6, 2, -1, 0, 0, 0]);
//! ```
//!
//! # find
//!
//! - [`find`](crate::ops::SliceFind::find) / [`rfind`](crate::ops::SliceFind::rfind)
//! - [`find_by`](crate::ops::SliceFind::find_by) / [`rfind_by`](crate::ops::SliceFind::rfind_by)
//! - [`find_by_key`](crate::ops::SliceFind::find_by_key) / [`rfind_by`](crate::ops::SliceFind::rfind_by)
//!
//! ```rust
//! use slice_ops::ops::*;
//!
//! //                   v
//! let x = [3, 1, 4, 1, 5, 9, 2, 6, 5, 3];
//!
//! let i = x.find(&5).unwrap();
//!
//! assert_eq!(i, 4);
//! assert_eq!(x[i], 5);
//! ```
//!
//! # argmax / argmin
//!
//! - [`argmax`](crate::ops::SliceArgMinMax::argmax) / [`argmin`](crate::ops::SliceArgMinMax::argmin)
//! - [`argmax_by`](crate::ops::SliceArgMinMax::argmax_by) / [`argmin_by`](crate::ops::SliceArgMinMax::argmin_by)
//! - [`argmax_by_key`](crate::ops::SliceArgMinMax::argmax_by_key) / [`argmin_by_key`](crate::ops::SliceArgMinMax::argmin_by_key)
//!
//! ```rust
//! use slice_ops::ops::*;
//!
//! //                v
//! let x = [1, 5, 5, 6, 2, -1, 0, -4, -1, 6];
//!
//! let i = x.argmax().unwrap();
//!
//! assert_eq!(i, 3);
//! ```
//!
//! # visit
//!
//! - [`visit`](crate::ops::SliceVisit::visit) / [`visit_mut`](crate::ops::SliceVisit::visit_mut)
//! - [`rvisit`](crate::ops::SliceVisit::rvisit) / [`rvisit_mut`](crate::ops::SliceVisit::rvisit_mut)
//! - [`visit_async`](crate::ops::SliceVisit::visit_async) / [`visit_mut_async`](crate::ops::SliceVisit::visit_mut_async)
//! - [`try_visit`](crate::ops::SliceVisit::try_visit) / [`try_visit_mut`](crate::ops::SliceVisit::try_visit_mut)
//! - [`try_rvisit`](crate::ops::SliceVisit::try_rvisit) / [`try_rvisit_mut`](crate::ops::SliceVisit::try_rvisit_mut)
//! - [`try_visit_async`](crate::ops::SliceVisit::try_visit_async) / [`try_visit_mut_async`](crate::ops::SliceVisit::try_visit_mut_async)
//!
//! ```rust
//! use slice_ops::ops::*;
//!
//! let mut x = [0; 8];
//!
//! let mut i = 0;
//!
//! x.visit_mut(|e| {
//!     i += 1;
//!     *e = i;
//! });
//!
//! assert_eq!(x, [1, 2, 3, 4, 5, 6, 7, 8]);
//! ```
//!
//! # ..._assign_all
//!
//! - [`add_assign_all`](crate::ops::SliceAddAssign::add_assign_all) / [`add_assign_all_async`](crate::ops::SliceAddAssign::add_assign_all_async)
//! - [`sub_assign_all`](crate::ops::SliceSubAssign::sub_assign_all) / [`sub_assign_all_async`](crate::ops::SliceSubAssign::sub_assign_all_async)
//! - [`mul_assign_all`](crate::ops::SliceMulAssign::mul_assign_all) / [`mul_assign_all_async`](crate::ops::SliceMulAssign::mul_assign_all_async)
//! - [`div_assign_all`](crate::ops::SliceDivAssign::div_assign_all) / [`div_assign_all_async`](crate::ops::SliceDivAssign::div_assign_all_async)
//! - [`rem_assign_all`](crate::ops::SliceRemAssign::rem_assign_all) / [`rem_assign_all_async`](crate::ops::SliceRemAssign::rem_assign_all_async)
//! - [`shl_assign_all`](crate::ops::SliceShlAssign::shl_assign_all) / [`shl_assign_all_async`](crate::ops::SliceShlAssign::shl_assign_all_async)
//! - [`shr_assign_all`](crate::ops::SliceShrAssign::shr_assign_all) / [`shr_assign_all_async`](crate::ops::SliceShrAssign::shr_assign_all_async)
//! - [`bitor_assign_all`](crate::ops::SliceBitOrAssign::bitor_assign_all) / [`bitor_assign_all_async`](crate::ops::SliceBitOrAssign::bitor_assign_all_async)
//! - [`bitand_assign_all`](crate::ops::SliceBitAndAssign::bitand_assign_all) / [`bitand_assign_all_async`](crate::ops::SliceBitAndAssign::bitand_assign_all_async)
//! - [`bitxor_assign_all`](crate::ops::SliceBitXorAssign::bitxor_assign_all) / [`bitxor_assign_all_async`](crate::ops::SliceBitXorAssign::bitxor_assign_all_async)
//! - [`neg_assign_all`](crate::ops::SliceNegAssign::neg_assign_all) / [`neg_assign_all_async`](crate::ops::SliceNegAssign::neg_assign_all_async)
//! - [`not_assign_all`](crate::ops::SliceNotAssign::not_assign_all) / [`not_assign_all_async`](crate::ops::SliceNotAssign::not_assign_all_async)
//!
//! ```rust
//! use slice_ops::ops::*;
//!
//! let mut x = [1, 2, 3, 4, 5, 6, 7, 8];
//!
//! x.mul_assign_all(2);
//!
//! assert_eq!(x, [2, 4, 6, 8, 10, 12, 14, 16]);
//! ```
//!
//! # shift
//!
//! - [`shift_many_left`](crate::ops::SliceShift::shift_many_left) / [`shift_many_right`](crate::ops::SliceShift::shift_many_right)
//! - [`shift_left`](crate::ops::SliceShift::shift_left) / [`shift_right`](crate::ops::SliceShift::shift_right)
//!
//! ```rust
//! use slice_ops::ops::*;
//!
//! let mut register = [4, 5, 6, 7, 8, 9];
//! let mut io = [1, 2, 3];
//!
//! register.shift_many_right(&mut io);
//!
//! assert_eq!(register, [1, 2, 3, 4, 5, 6]);
//! assert_eq!(io, [7, 8, 9]);
//! ```
//!
//! # spread
//!
//! - [`spread`](crate::ops::SliceSpread::spread) / [`spread_mut`](crate::ops::SliceSpread::spread_mut)
//!
//! ```rust
//! #![feature(generic_const_exprs)]
//!
//! use slice_ops::ops::*;
//!
//! let arr = [1, 2, 3];
//! let slice = arr.as_slice();
//!
//! let [odd, even] = slice.spread();
//!
//! assert_eq!(odd, [1, 3]);
//! assert_eq!(even, [2]);
//! ```
//!
//! # bit_rev_permutation
//!
//! - [`bit_rev_permutation`](crate::ops::SlicePermute::bit_rev_permutation)
//! - [`digit_rev_permutation`](crate::ops::SlicePermute::digit_rev_permutation)
//!
//! ```rust
//! use slice_ops::ops::*;
//!
//! let mut arr = [0b000, 0b001, 0b010, 0b011, 0b100, 0b101, 0b110, 0b111];
//!
//! arr.bit_rev_permutation();
//!
//! assert_eq!(arr, [0b000, 0b100, 0b010, 0b110, 0b001, 0b101, 0b011, 0b111])
//! ```
//!
//! # grey_code_permutation
//!
//! - [`grey_code_permutation`](crate::ops::SlicePermute::grey_code_permutation)
//!
//! ```rust
//! use slice_ops::ops::*;
//!
//! let mut arr = [0b000, 0b001, 0b010, 0b011, 0b100, 0b101, 0b110, 0b111];
//!
//! arr.as_mut_slice().grey_code_permutation();
//!
//! assert_eq!(arr, [0b000, 0b001, 0b011, 0b010, 0b110, 0b111, 0b101, 0b100])
//! ```
//!
//! # trim
//!
//! - [`trim`](crate::ops::SliceTrim::trim) / [`trim_mut`](crate::ops::SliceTrim::trim_mut)
//! - [`trim_front`](crate::ops::SliceTrim::trim_front) / [`trim_front_mut`](crate::ops::SliceTrim::trim_front_mut)
//! - [`trim_back`](crate::ops::SliceTrim::trim_back) / [`trim_back_mut`](crate::ops::SliceTrim::trim_back_mut)
//!
//! ```rust
//! use slice_ops::ops::*;
//!
//! let a = [0, 0, 0, 1, 2, 3, 0, 0, 0];
//!
//! let at = a.trim(|&e| e == 0);
//!
//! assert_eq!(at, &[1, 2, 3]);
//! ```

#[cfg(feature = "alloc")]
extern crate alloc;

moddef::moddef!(
    pub mod {
        ops,
        join,
        padded
    },
    mod {
        private
    }
);

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
pub const fn is_power_of(n: usize, r: usize) -> bool
{
    r.pow(n.ilog(r)) == n
}

#[cfg(test)]
mod tests
{
    #[test]
    fn it_works() {}
}
