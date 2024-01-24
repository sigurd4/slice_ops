#![cfg_attr(not(any(feature = "std", test)), no_std)]

#![feature(const_trait_impl)]
#![feature(const_slice_split_at_mut)]
#![feature(const_mut_refs)]
#![feature(slice_from_ptr_range)]
#![feature(const_refs_to_cell)]
#![feature(const_slice_from_raw_parts_mut)]
#![feature(allocator_api)]
#![cfg_attr(feature = "std", feature(new_uninit))]

#![feature(generic_const_exprs)]

moddef::moddef!(
    flat(pub) mod {
        slice_ops_,
        padded
    }
);

#[cfg(test)]
#[test]
fn test()
{
    let a = [1, 2];

    let ar: &[u8] = &a;

    let _split = ar.rsplit_array_ref2::<2>();
}