use core::{alloc::Allocator, future::Future, pin::Pin, task::{Context, Poll}};
use alloc::{alloc::Global, boxed::Box};

use crate::private;

use super::MaybeDone;

#[cfg(feature = "alloc")]
pub struct ErrorRace<T, E, A = Global>
where
    T: Future<Output = Result<(), E>>,
    A: Allocator
{
    tasks: Box<[MaybeDone<T>], A>
}
#[cfg(not(feature = "alloc"))]
pub struct ErrorRace<T, E, A>
where
    T: Future<Output = Result<(), E>>,
    A: Allocator
{
    tasks: Box<[T], A>
}

#[cfg(feature = "alloc")]
impl<T, E> ErrorRace<T, E>
where
    T: Future<Output = Result<(), E>>
{
    pub(crate) fn new<I>(tasks: I) -> Self
    where
        I: ExactSizeIterator<Item = T>
    {
        Self::new_in(tasks, Global)
    }
}

impl<T, E, A> ErrorRace<T, E, A>
where
    T: Future<Output = Result<(), E>>,
    A: Allocator
{
    pub(crate) fn new_in<I>(tasks: I, alloc: A) -> Self
    where
        I: ExactSizeIterator<Item = T>
    {
        Self {
            tasks: private::collect_boxed_slice_in(tasks.map(|task| MaybeDone::Future(task)), alloc)
        }
    }
}

impl<T, E, A> Future for ErrorRace<T, E, A>
where
    T: Future<Output = Result<(), E>>,
    A: Allocator
{
    type Output = Result<(), E>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>
    {
        let mut done = true;

        let l = self.tasks.len();
        let mut i = 0;
        
        while i < l
        {
            let task = unsafe {
                self.as_mut()
                    .map_unchecked_mut(|join| &mut join.tasks[i])
            };
            let ready = task.poll(cx)
                .is_ready();
            if ready
            {
                let join = unsafe {
                    self.as_mut()
                        .get_unchecked_mut()
                };
                let result = join.tasks[i].take_output();
                if let Some(result) = result && result.is_err()
                {
                    return Poll::Ready(result)
                }
            }
            else
            {
                done = false
            }
            i += 1;
        }

        if !done
        {
            return Poll::Pending
        }

        Poll::Ready(Ok(()))
    }
}