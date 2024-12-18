use core::{alloc::Allocator, future::Future, mem::MaybeUninit, pin::Pin, task::{Context, Poll}};

use alloc::boxed::Box;

moddef::moddef!(
    flat(pub) mod {
        actions,
        error_race
    }
);

enum MaybeDone<F: Future>
{
    Future(F),
    Done(F::Output),
    Taken,
}

impl<F: Future> MaybeDone<F>
{
    pub fn take_output(&mut self) -> Option<F::Output>
    {
        match *self
        {
            MaybeDone::Done(_) => match core::mem::replace(self, Self::Taken)
            {
                MaybeDone::Done(val) => Some(val),
                _ => unreachable!(),
            },
            _ => None,
        }
    }
}

impl<F: Future> Future for MaybeDone<F>
{
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>
    {
        // SAFETY: pinning in structural for `f`
        unsafe {
            match *self.as_mut().get_unchecked_mut()
            {
                MaybeDone::Future(ref mut f) => {
                    let val = core::task::ready!(Pin::new_unchecked(f).poll(cx));
                    self.set(Self::Done(val));
                }
                MaybeDone::Done(_) => {}
                MaybeDone::Taken => unreachable!(),
            }
        }

        Poll::Ready(())
    }
}

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