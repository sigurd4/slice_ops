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

        let _split = ar.rsplit_array_ref2::<2>();
    }
}