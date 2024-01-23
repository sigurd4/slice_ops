pub struct PaddedSlice<'a, T>
{
    slice: &'a [T],
    padding: usize
}

impl<'a, T> PaddedSlice<'a, T>
{
    pub const fn new(slice: &'a [T]) -> Self
    {
        Self {
            slice,
            padding: 0
        }
    }

    pub fn spread_slice(slice: &'a [T], width: usize) -> (Vec<Self>, &'a [T])
    {
        let (left, right) = crate::rsplit_at(slice, if width == 0 {slice.len()} else {slice.len() % width});

        unsafe {(
            (0..width).map(|i| Self {
                slice: &left[i..],
                padding: width - 1
            }).collect(),
            right
        )}
    }
    
    pub fn rspread_slice(slice: &'a [T], width: usize) -> (&'a [T], Vec<Self>)
    {
        let (left, right) = crate::split_at(slice, if width == 0 {slice.len()} else {slice.len() % width});

        unsafe {(
            left,
            (0..width).map(|i| Self {
                slice: &right[i..],
                padding: width - 1
            }).collect()
        )}
    }
}

pub struct PaddedSliceMut<'a, T>
{
    slice: &'a mut [T],
    padding: usize
}