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
#![feature(let_chains)]
#![feature(generic_const_exprs)]
#![feature(core_intrinsics)]

//! Provides many useful utility methods for slices.
//!
//! # integrate / differentiate
//!
//! - [integrate](SliceOps::integrate)
//! - [differentiate](SliceOps::differentiate)
//!
//! ```rust
//! use slice_ops::*;
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
//! - [find](SliceOps::find) / [rfind](SliceOps::rfind)
//! - [find_by](SliceOps::find_by) / [rfind_by](SliceOps::rfind_by)
//! - [find_by_key](SliceOps::find_by_key) / [rfind_by](SliceOps::rfind_by)
//!
//! ```rust
//! use slice_ops::*;
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
//! - [argmax](SliceOps::argmax) / [argmin](SliceOps::argmin)
//! - [argmax_by](SliceOps::argmax_by) / [argmin_by](SliceOps::argmin_by)
//! - [argmax_by_key](SliceOps::argmax_by_key) / [argmin_by_key](SliceOps::argmin_by_key)
//!
//! ```rust
//! use slice_ops::*;
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
//! - [visit](SliceOps::visit) / [visit_mut](SliceOps::visit_mut)
//! - [rvisit](SliceOps::rvisit) / [rvisit_mut](SliceOps::rvisit_mut)
//! - [visit_async](SliceOps::visit_async) / [visit_mut_async](SliceOps::visit_mut_async)
//! - [try_visit](SliceOps::try_visit) / [try_visit_mut](SliceOps::try_visit_mut)
//! - [try_rvisit](SliceOps::try_rvisit) / [try_rvisit_mut](SliceOps::try_rvisit_mut)
//! - [try_visit_async](SliceOps::try_visit_async) / [try_visit_mut_async](SliceOps::try_visit_mut_async)
//!
//! ```rust
//! use slice_ops::*;
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
//! - [add_assign_all](SliceOps::add_assign_all) / [add_assign_all_async](SliceOps::add_assign_all_async)
//! - [sub_assign_all](SliceOps::sub_assign_all) / [sub_assign_all_async](SliceOps::sub_assign_all_async)
//! - [mul_assign_all](SliceOps::mul_assign_all) / [mul_assign_all_async](SliceOps::mul_assign_all_async)
//! - [div_assign_all](SliceOps::div_assign_all) / [div_assign_all_async](SliceOps::div_assign_all_async)
//! - [rem_assign_all](SliceOps::rem_assign_all) / [rem_assign_all_async](SliceOps::rem_assign_all_async)
//! - [shl_assign_all](SliceOps::shl_assign_all) / [shl_assign_all_async](SliceOps::shl_assign_all_async)
//! - [shr_assign_all](SliceOps::shr_assign_all) / [shr_assign_all_async](SliceOps::shr_assign_all_async)
//! - [bitor_assign_all](SliceOps::bitor_assign_all) / [bitor_assign_all_async](SliceOps::bitor_assign_all_async)
//! - [bitand_assign_all](SliceOps::bitand_assign_all) / [bitand_assign_all_async](SliceOps::bitand_assign_all_async)
//! - [bitxor_assign_all](SliceOps::bitxor_assign_all) / [bitxor_assign_all_async](SliceOps::bitxor_assign_all_async)
//! - [neg_assign_all](SliceOps::neg_assign_all) / [neg_assign_all_async](SliceOps::neg_assign_all_async)
//! - [not_assign_all](SliceOps::not_assign_all) / [not_assign_all_async](SliceOps::not_assign_all_async)
//!
//! ```rust
//! use slice_ops::*;
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
//! - [shift_many_left](SliceOps::shift_many_left) / [shift_many_right](SliceOps::shift_many_right)
//! - [shift_left](SliceOps::shift_left) / [shift_right](SliceOps::shift_right)
//!
//! ```rust
//! use slice_ops::*;
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
//! # spread_chunks
//!
//! - [spread_chunks](SliceOps::spread_chunks) / [spread_chunks_mut](SliceOps::spread_chunks_mut)
//!
//! ```rust
//! #![feature(generic_const_exprs)]
//!
//! use slice_ops::*;
//!
//! let arr = [1, 2, 3];
//! let slice = arr.as_slice();
//!
//! let [odd, even] = slice.spread_chunks();
//!
//! assert_eq!(odd, [1, 3]);
//! assert_eq!(even, [2]);
//! ```
//!
//! # bit_rev_permutation
//!
//! - [bit_rev_permutation](SliceOps::bit_rev_permutation)
//! - [digit_rev_permutation](SliceOps::digit_rev_permutation)
//!
//! ```rust
//! use slice_ops::*;
//!
//! let mut arr = [0b000, 0b001, 0b010, 0b011, 0b100, 0b101, 0b110, 0b111];
//!
//! arr.bit_rev_permutation();
//!
//! assert_eq!(arr, [0b000, 0b100, 0b010, 0b110, 0b001, 0b101, 0b011, 0b111])
//! ```
//!
//! # trim
//!
//! - [trim](SliceOps::trim) / [trim_mut](SliceOps::trim_mut)
//! - [trim_front](SliceOps::trim_front) / [trim_front_mut](SliceOps::trim_front_mut)
//! - [trim_back](SliceOps::trim_back) / [trim_back_mut](SliceOps::trim_back_mut)
//!
//! ```rust
//! use slice_ops::*;
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
    flat(pub) mod {
        join for cfg(feature = "alloc"),
        slice_ops_,
        padded
    }
);

pub const fn is_power_of(n: usize, r: usize) -> bool
{
    r.pow(n.ilog(r)) == n
}

#[cfg(test)]
mod tests
{
    use crate::SliceOps;

    #[test]
    fn test_grey_code_permutation()
    {
        let mut arr = [0b000, 0b001, 0b010, 0b011, 0b100, 0b101, 0b110, 0b111];

        arr.as_mut_slice().grey_code_permutation();

        assert_eq!(arr, [0b000, 0b001, 0b011, 0b010, 0b110, 0b111, 0b101, 0b100])
    }

    #[test]
    fn test()
    {
        let a = [1, 2];

        let ar: &[u8] = &a;

        let i = ar.argmin().unwrap();

        println!("{}", i);
    }

    #[test]
    fn trim()
    {
        let a = [0, 0, 0, 1, 2, 3, 0, 0, 0];

        let at = a.trim(|&e| e == 0);

        assert_eq!(at, &[1, 2, 3]);
    }

    #[test]
    fn fizzbuzz()
    {
        let mut arr = ["1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14", "15"];
        let slice = arr.as_mut_slice();

        let [_, _, fizz] = slice.spread_chunks_mut();
        assert_eq!(fizz, ["3", "6", "9", "12", "15"]);
        for fizz in fizz.iter_mut()
        {
            **fizz = "fizz";
        }

        let [_, _, _, _, buzz] = slice.spread_chunks_mut();
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

    #[cfg(feature = "alloc")]
    #[test]
    fn betch_async()
    {
        tokio_test::block_on(async {
            let mut x = [1, 2, 3, 4, 5, 6, 7, 8];

            x.mul_assign_all_async(2).await;

            assert_eq!(x, [2, 4, 6, 8, 10, 12, 14, 16]);
        });
    }
}
