use core::{mem::{MaybeUninit, ManuallyDrop}, borrow::{Borrow, BorrowMut}, ops::{Deref, DerefMut}};

#[repr(C)]
pub struct Padded<T, const WIDTH: usize>
where
    [(); WIDTH - 1]:
{
    value: T,
    _pad: ManuallyDrop<MaybeUninit<[T; WIDTH - 1]>>
}

impl<T, const WIDTH: usize> PartialEq<T> for Padded<T, WIDTH>
where
    [(); WIDTH - 1]:,
    T: PartialEq
{
    fn eq(&self, other: &T) -> bool
    {
        self.value.eq(other)
    }
}
impl<T, const WIDTH: usize> PartialEq for Padded<T, WIDTH>
where
    [(); WIDTH - 1]:,
    T: PartialEq
{
    fn eq(&self, other: &Self) -> bool
    {
        self.value.eq(&other.value)
    }
}
impl<T, const WIDTH: usize> Eq for Padded<T, WIDTH>
where
    [(); WIDTH - 1]:,
    T: Eq
{
    
}

impl<T, const WIDTH: usize> PartialOrd<T> for Padded<T, WIDTH>
where
    [(); WIDTH - 1]:,
    T: PartialOrd
{
    fn partial_cmp(&self, other: &T) -> Option<core::cmp::Ordering>
    {
        self.value.partial_cmp(other)
    }
}
impl<T, const WIDTH: usize> PartialOrd for Padded<T, WIDTH>
where
    [(); WIDTH - 1]:,
    T: PartialOrd
{
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering>
    {
        self.value.partial_cmp(&other.value)
    }
}
impl<T, const WIDTH: usize> Ord for Padded<T, WIDTH>
where
    [(); WIDTH - 1]:,
    T: Ord
{
    fn cmp(&self, other: &Self) -> core::cmp::Ordering
    {
        self.value.cmp(&other.value)
    }
}

impl<T, const WIDTH: usize> core::fmt::Debug for Padded<T, WIDTH>
where
    [(); WIDTH - 1]:,
    T: core::fmt::Debug
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
    {
        self.borrow().fmt(f)
    }
}
impl<T, const WIDTH: usize> core::fmt::Display for Padded<T, WIDTH>
where
    [(); WIDTH - 1]:,
    T: core::fmt::Display
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
    {
        self.borrow().fmt(f)
    }
}

impl<T, const WIDTH: usize> Padded<T, WIDTH>
where
    [(); WIDTH - 1]:
{
    pub const fn new(value: T) -> Self
    {
        Self
        {
            value,
            _pad: ManuallyDrop::new(MaybeUninit::uninit())
        }
    }
    pub fn into_inner(self) -> T
    {
        self.value
    }
    pub const fn borrow(&self) -> &T
    {
        &self.value
    }
    pub const fn borrow_mut(&mut self) -> &mut T
    {
        &mut self.value
    }
}
impl<T, const WIDTH: usize> Borrow<T> for Padded<T, WIDTH>
where
    [(); WIDTH - 1]:
{
    fn borrow(&self) -> &T
    {
        self.borrow()
    }
}
impl<T, const WIDTH: usize> BorrowMut<T> for Padded<T, WIDTH>
where
    [(); WIDTH - 1]:
{
    fn borrow_mut(&mut self) -> &mut T
    {
        self.borrow_mut()
    }
}
impl<T, const WIDTH: usize> Deref for Padded<T, WIDTH>
where
    [(); WIDTH - 1]:
{
    type Target = T;

    fn deref(&self) -> &Self::Target
    {
        self.borrow()
    }
}
impl<T, const WIDTH: usize> DerefMut for Padded<T, WIDTH>
where
    [(); WIDTH - 1]:
{
    fn deref_mut(&mut self) -> &mut Self::Target
    {
        self.borrow_mut()
    }
}