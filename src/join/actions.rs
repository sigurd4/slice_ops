use core::{alloc::Allocator, future::Future, pin::Pin, task::{Context, Poll}};
use alloc::{alloc::Global, boxed::Box};

use crate::private;

#[cfg(feature = "alloc")]
pub struct Actions<T, A = Global>
where
    T: Future<Output = ()>,
    A: Allocator
{
    tasks: Box<[T], A>
}
#[cfg(not(feature = "alloc"))]
pub struct Actions<T, A>
where
    T: Future<Output = ()>,
    A: Allocator
{
    tasks: Box<[T], A>
}

#[cfg(feature = "alloc")]
impl<T> Actions<T>
where
    T: Future<Output = ()>
{
    pub(crate) fn new<I>(tasks: I) -> Self
    where
        I: ExactSizeIterator<Item = T>
    {
        Self::new_in(tasks, Global)
    }
}

impl<T, A> Actions<T, A>
where
    T: Future<Output = ()>,
    A: Allocator
{
    pub(crate) fn new_in<I>(tasks: I, alloc: A) -> Self
    where
        I: ExactSizeIterator<Item = T>
    {
        Self {
            tasks: private::collect_boxed_slice_in(tasks, alloc)
        }
    }
}

impl<T, A> Future for Actions<T, A>
where
    T: Future<Output = ()>,
    A: Allocator
{
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>
    {
        let mut done = true;

        let l = self.tasks.len();
        let mut i = 0;
        
        while i < l
        {
            done &= unsafe {
                self.as_mut()
                    .map_unchecked_mut(|join| &mut join.tasks[i])
                    .poll(cx)
                    .is_ready()
            };
            i += 1;
        }

        if !done
        {
            return Poll::Pending
        }

        Poll::Ready(())
    }
}