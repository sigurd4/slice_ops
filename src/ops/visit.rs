use bulks::{AsBulk, AsBulkMut, DoubleEndedBulk};
use slice_trait::Slice;

use core::marker::Destruct;
#[cfg(feature = "alloc")]
use core::ops::AsyncFn;

pub const trait Visit<'a>: AsBulk<'a>
{
    /// Visits each element once, from left to right.
    ///
    /// # Example
    ///
    /// ```rust
    /// use slice_ops::ops::*;
    ///
    /// let x = [1, 2, 3, 4, 5, 6, 7, 8];
    ///
    /// let mut i = 0;
    ///
    /// x.visit(|&e| {
    ///     i += 1;
    ///     assert_eq!(i, e)
    /// });
    /// ```
    fn visit<F>(&'a self, visitor: F)
    where
        F: FnMut(&'a Self::Elem) + [const] Destruct;
    /// Visits each element once, from left to right, or short-circuits if visitor returns error.
    ///
    /// # Example
    ///
    /// ```rust
    /// use slice_ops::ops::*;
    ///
    /// let x = [1, 2, 3, 4, 5, 6, 7, 8];
    ///
    /// let mut i = 0;
    ///
    /// let result = x.try_visit(|&e| {
    ///     i += 1;
    ///     if i > 4
    ///     {
    ///         return Err(i)
    ///     }
    ///     assert_eq!(i, e);
    ///     Ok(())
    /// });
    ///
    /// assert_eq!(result, Err(5));
    /// ```
    fn try_visit<E, F>(&'a self, visitor: F) -> Result<(), E>
    where
        F: FnMut(&'a Self::Elem) -> Result<(), E> /* + ~const Destruct */;
}
pub const trait RVisit<'a>: Visit<'a, AsBulk: DoubleEndedBulk>
{
    /// Visits each element once, from right to left.
    ///
    /// # Example
    ///
    /// ```rust
    /// use slice_ops::ops::*;
    ///
    /// let x = [8, 7, 6, 5, 4, 3, 2, 1];
    ///
    /// let mut i = 0;
    ///
    /// x.rvisit(|&e| {
    ///     i += 1;
    ///     assert_eq!(i, e)
    /// });
    /// ```
    fn rvisit<F>(&'a self, visitor: F)
    where
        F: FnMut(&'a Self::Elem) /* + ~const Destruct */;
    /// Visits each element once, from right to left, or short-circuits if visitor returns error.
    ///
    /// # Example
    ///
    /// ```rust
    /// use slice_ops::ops::*;
    ///
    /// let x = [8, 7, 6, 5, 4, 3, 2, 1];
    ///
    /// let mut i = 0;
    ///
    /// let result = x.try_rvisit(|&e| {
    ///     i += 1;
    ///     if i > 4
    ///     {
    ///         return Err(i)
    ///     }
    ///     assert_eq!(i, e);
    ///     Ok(())
    /// });
    ///
    /// assert_eq!(result, Err(5));
    /// ```
    fn try_rvisit<E, F>(&'a self, visitor: F) -> Result<(), E>
    where
        F: FnMut(&'a Self::Elem) -> Result<(), E> /* + ~const Destruct */;
}

pub const trait VisitMut<'a>: AsBulkMut<'a> + Visit<'a>
{
    /// Mutably visits each element once, from left to right.
    ///
    /// # Example
    ///
    /// ```rust
    /// use slice_ops::ops::*;
    ///
    /// let mut x = [0; 8];
    ///
    /// let mut i = 0;
    ///
    /// x.visit_mut(|e| {
    ///     i += 1;
    ///     *e = i;
    /// });
    ///
    /// assert_eq!(x, [1, 2, 3, 4, 5, 6, 7, 8]);
    /// ```
    fn visit_mut<F>(&'a mut self, visitor: F)
    where
        F: FnMut(<Self::AsBulkMut as IntoIterator>::Item) + [const] Destruct;
    /// Mutably visits each element once, from left to right, or short-circuits if visitor returns error.
    ///
    /// # Example
    ///
    /// ```rust
    /// use slice_ops::ops::*;
    ///
    /// let mut x = [0; 8];
    ///
    /// let mut i = 0;
    ///
    /// let result = x.try_visit_mut(|e| {
    ///     i += 1;
    ///     if i > 4
    ///     {
    ///         return Err(i)
    ///     }
    ///     *e = i;
    ///     Ok(())
    /// });
    ///
    /// assert_eq!(result, Err(5));
    /// assert_eq!(x, [1, 2, 3, 4, 0, 0, 0, 0])
    /// ```
    fn try_visit_mut<E, F>(&'a mut self, visitor: F) -> Result<(), E>
    where
        F: FnMut(<Self::AsBulkMut as IntoIterator>::Item) -> Result<(), E>, /* + ~const Destruct */;
}
pub const trait RVisitMut<'a>: VisitMut<'a, AsBulk: DoubleEndedBulk> + RVisit<'a>
{
    /// Mutably visits each element once, from right to left.
    ///
    /// # Example
    ///
    /// ```rust
    /// use slice_ops::ops::*;
    ///
    /// let mut x = [0; 8];
    ///
    /// let mut i = 0;
    ///
    /// x.rvisit_mut(|e| {
    ///     i += 1;
    ///     *e = i;
    /// });
    ///
    /// assert_eq!(x, [8, 7, 6, 5, 4, 3, 2, 1]);
    /// ```
    fn rvisit_mut<F>(&'a mut self, visitor: F)
    where
        F: FnMut(&'a mut Self::Elem) /* + ~const Destruct */;
    /// Mutably visits each element once, from right to left, or short-circuits if visitor returns error.
    ///
    /// # Example
    ///
    /// ```rust
    /// use slice_ops::ops::*;
    ///
    /// let mut x = [0; 8];
    ///
    /// let mut i = 0;
    ///
    /// let result = x.try_rvisit_mut(|e| {
    ///     i += 1;
    ///     if i > 4
    ///     {
    ///         return Err(i)
    ///     }
    ///     *e = i;
    ///     Ok(())
    /// });
    ///
    /// assert_eq!(result, Err(5));
    /// assert_eq!(x, [0, 0, 0, 0, 4, 3, 2, 1])
    /// ```
    fn try_rvisit_mut<E, F>(&'a mut self, visitor: F) -> Result<(), E>
    where
        F: FnMut(&'a mut Self::Elem) -> Result<(), E> /* + ~const Destruct */;
}
pub trait VisitAsync: Visit
{
    /// Visits each element once, asynchronously.
    ///
    /// # Example
    ///
    /// ```rust
    /// use slice_ops::ops::*;
    ///
    /// let x = [1, 2, 3, 4, 5, 6, 7, 8];
    ///
    /// # tokio_test::block_on(async {
    /// x.visit_async(async |&e| {
    ///     assert_eq!(x[e - 1], e)
    /// }).await;
    /// # })
    /// ```
    #[cfg(feature = "alloc")]
    async fn visit_async<'a, F>(&'a self, visitor: F)
    where
        F: AsyncFn(&'a Self::Elem), /* + ~const Destruct */
        Self::Elem: 'a;
    /// Mutably visits each element once, asynchronously.
    ///
    /// # Example
    ///
    /// ```rust
    /// use slice_ops::ops::*;
    ///
    /// let mut x = [8, 7, 6, 5, 4, 3, 2, 1];
    ///
    /// # tokio_test::block_on(async {
    /// x.visit_mut_async(async |e| {
    ///     *e = 9 - *e
    /// }).await;
    ///
    /// assert_eq!(x, [1, 2, 3, 4, 5, 6, 7, 8]);
    /// # })
    /// ```
    #[cfg(feature = "alloc")]
    async fn visit_mut_async<'a, F>(&'a mut self, visitor: F)
    where
        F: AsyncFn(&'a mut Self::Elem), /* + ~const Destruct */
        Self::Elem: 'a;
    /// Visits each element once, asynchronously, or short-circuits if visitor returns error.
    ///
    /// # Warning
    ///
    /// When any of the tasks return an error, all other tasks will be ignored. The tasks are not nessecarily stopped, and may still be running in the background.
    ///
    /// If you want to wait for all tasks to complete, keep polling the future until it returns an [`Ok`](core::result::Result).
    ///
    /// # Example
    ///
    /// ```rust
    /// use slice_ops::ops::*;
    ///
    /// let x = [1, 2, 3, 4, 5, 6, 7, 8];
    ///
    /// # tokio_test::block_on(async {
    /// let result = x.try_visit_async(async |&e| {
    ///     if e > 4
    ///     {
    ///         return Err(e)
    ///     }
    ///     assert_eq!(x[e - 1], e);
    ///     Ok(())
    /// }).await;
    ///
    /// assert!(result == Err(5) || result == Err(6) || result == Err(7) || result == Err(8));
    /// # })
    /// ```
    #[cfg(feature = "alloc")]
    async fn try_visit_async<'a, E, F>(&'a self, visitor: F) -> Result<(), E>
    where
        F: AsyncFn(&'a Self::Elem) -> Result<(), E>, /* + ~const Destruct */
        Self::Elem: 'a;
    /// Mutably visits each element once, asynchronously, or short-circuits if visitor returns error.
    ///
    /// # Warning
    ///
    /// When any of the tasks return an error, all other tasks will be ignored. The tasks are not nessecarily stopped, and may still be running in the background.
    ///
    /// If you want to wait for all tasks to complete, keep polling the future until it returns an [`Ok`](core::result::Result).
    ///
    /// # Example
    ///
    /// ```rust
    /// use slice_ops::ops::*;
    ///
    /// let mut x = [1, 2, 3, 4, 5, 6, 7, 8];
    ///
    /// # tokio_test::block_on(async {
    /// let result = x.try_visit_mut_async(async |e| {
    ///     if *e <= 4
    ///     {
    ///         return Err(*e)
    ///     }
    ///     *e = 9 - *e;
    ///     Ok(())
    /// }).await;
    ///
    /// assert_eq!(x[..4], [1, 2, 3, 4]);
    /// assert!(x[4] == 5 || x[4] == 4);
    /// assert!(x[5] == 6 || x[5] == 3);
    /// assert!(x[6] == 7 || x[6] == 2);
    /// assert!(x[7] == 8 || x[7] == 1);
    /// # })
    /// ```
    #[cfg(feature = "alloc")]
    async fn try_visit_mut_async<'a, E, F>(&'a mut self, visitor: F) -> Result<(), E>
    where
        F: AsyncFn(&'a mut Self::Elem) -> Result<(), E>, /* + ~const Destruct */
        Self::Elem: 'a;
}

impl<T> SliceVisit for [T]
{
    fn visit<'a, F>(&'a self, mut visitor: F)
    where
        F: FnMut(&'a T),
        T: 'a
    {
        self.bulk().
    }

    fn visit_mut<'a, F>(&'a mut self, mut visitor: F)
    where
        F: FnMut(&'a mut T),
        T: 'a
    {
        let l = self.len();
        let mut i = 0;
        while i < l
        {
            visitor(unsafe { core::mem::transmute::<&mut T, &mut T>(&mut self[i]) });
            i += 1;
        }
    }

    fn try_visit<'a, E, F>(&'a self, mut visitor: F) -> Result<(), E>
    where
        F: FnMut(&'a T) -> Result<(), E>,
        T: 'a
    {
        let l = self.len();
        let mut i = 0;
        while i < l
        {
            visitor(&self[i])?;
            i += 1;
        }
        Ok(())
    }

    fn try_visit_mut<'a, E, F>(&'a mut self, mut visitor: F) -> Result<(), E>
    where
        F: FnMut(&'a mut T) -> Result<(), E>,
        T: 'a
    {
        let l = self.len();
        let mut i = 0;
        while i < l
        {
            visitor(unsafe { core::mem::transmute::<&mut T, &mut T>(&mut self[i]) })?;
            i += 1;
        }
        Ok(())
    }

    fn rvisit<'a, F>(&'a self, mut visitor: F)
    where
        F: FnMut(&'a T),
        T: 'a
    {
        let l = self.len();
        let mut i = l;
        while i > 0
        {
            i -= 1;
            visitor(&self[i]);
        }
    }

    fn rvisit_mut<'a, F>(&'a mut self, mut visitor: F)
    where
        F: FnMut(&'a mut T),
        T: 'a
    {
        let l = self.len();
        let mut i = l;
        while i > 0
        {
            i -= 1;
            visitor(unsafe { core::mem::transmute::<&mut T, &mut T>(&mut self[i]) });
        }
    }

    fn try_rvisit<'a, E, F>(&'a self, mut visitor: F) -> Result<(), E>
    where
        F: FnMut(&'a T) -> Result<(), E>,
        T: 'a
    {
        let l = self.len();
        let mut i = l;
        while i > 0
        {
            i -= 1;
            visitor(&self[i])?;
        }
        Ok(())
    }

    fn try_rvisit_mut<'a, E, F>(&'a mut self, mut visitor: F) -> Result<(), E>
    where
        F: FnMut(&'a mut T) -> Result<(), E>,
        T: 'a
    {
        let l = self.len();
        let mut i = l;
        while i > 0
        {
            i -= 1;
            visitor(unsafe { core::mem::transmute::<&mut T, &mut T>(&mut self[i]) })?;
        }
        Ok(())
    }

    #[cfg(feature = "alloc")]
    async fn visit_async<'a, F>(&'a self, visitor: F)
    where
        F: AsyncFn(&'a T),
        T: 'a
    {
        use crate::future::BoxedActions;

        #[allow(clippy::redundant_closure)]
        BoxedActions::new(self.iter().map(|x| visitor(x))).await
    }

    #[cfg(feature = "alloc")]
    async fn visit_mut_async<'a, F>(&'a mut self, visitor: F)
    where
        F: AsyncFn(&'a mut T),
        T: 'a
    {
        use crate::future::BoxedActions;

        #[allow(clippy::redundant_closure)]
        BoxedActions::new(self.iter_mut().map(|x| visitor(x))).await
    }

    #[cfg(feature = "alloc")]
    async fn try_visit_async<'a, E, F>(&'a self, visitor: F) -> Result<(), E>
    where
        F: AsyncFn(&'a T) -> Result<(), E>,
        T: 'a
    {
        use crate::future::TryBoxedActions;

        #[allow(clippy::redundant_closure)]
        TryBoxedActions::new(self.iter().map(|x| visitor(x))).await
    }

    #[cfg(feature = "alloc")]
    async fn try_visit_mut_async<'a, E, F>(&'a mut self, visitor: F) -> Result<(), E>
    where
        F: AsyncFn(&'a mut T) -> Result<(), E>,
        T: 'a
    {
        use crate::future::TryBoxedActions;

        #[allow(clippy::redundant_closure)]
        TryBoxedActions::new(self.iter_mut().map(|x| visitor(x))).await
    }
}

#[cfg(test)]
mod test
{
    #[test]
    fn it_works() {}
}
