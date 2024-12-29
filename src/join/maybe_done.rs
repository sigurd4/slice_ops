use core::{future::Future, pin::Pin, task::{Context, Poll}};

pub(crate) enum MaybeDone<F: Future>
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
    pub fn cancel(&mut self)
    {
        *self = Self::Taken
    }
    pub fn is_taken(&self) -> bool
    {
        core::matches!(self, MaybeDone::Taken)
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