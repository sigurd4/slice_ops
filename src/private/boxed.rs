use core::mem::MaybeUninit;
use alloc::{boxed::Box, alloc::Allocator};

pub fn collect_boxed_slice_in<I, A>(mut values: I, alloc: A) -> Box<[I::Item], A>
where
    I: ExactSizeIterator,
    A: Allocator
{
    let l = values.len();

    let mut boxed = Box::new_uninit_slice_in(l, alloc);

    let mut i = 0;
    while i < l
    {
        boxed[i] = MaybeUninit::new(unsafe {
            values.next()
                .unwrap_unchecked()
        });
        i += 1;
    }

    unsafe {
        boxed.assume_init()
    }
}