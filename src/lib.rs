#![cfg_attr(not(test), no_std)]

#![feature(const_trait_impl)]
#![feature(slice_from_ptr_range)]
#![feature(allocator_api)]
#![feature(const_eval_select)]
#![feature(const_swap_nonoverlapping)]
#![feature(const_slice_from_ptr_range)]
#![feature(const_destruct)]
#![feature(unboxed_closures)]

#![feature(generic_const_exprs)]
#![feature(core_intrinsics)]

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
        
        assert_eq!(arr, ["1", "2", "fizz", "4", "buzz", "fizz", "7", "8", "fizz", "buzz", "11", "fizz", "13", "14", "fizzbuzz"]);
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_async()
    {
        const N: usize = 8;
        let mut x = [0; N];
        for i in 0..N
        {
            x[i] = i;
        }


        tokio_test::block_on(async {
            x.mul_assign_all_async(2).await
        });

        println!("{:?}", x)
    }
}