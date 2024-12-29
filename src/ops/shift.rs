use slice_trait::Slice;

#[const_trait]
pub trait SliceShift<T>: Slice<Item = T>
{
    /// Shifts the entire slice as a SISO shift register with mutliple values to the left. The output is given in-place in `ìtems`.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use slice_ops::*;
    /// 
    /// let mut register = [9, 8, 7, 6, 5, 4];
    /// let mut io = [3, 2, 1];
    /// 
    /// register.shift_many_left(&mut io);
    /// 
    /// assert_eq!(register, [6, 5, 4, 3, 2, 1]);
    /// assert_eq!(io, [9, 8, 7]);
    /// ```
    fn shift_many_left(&mut self, items: &mut [T]);
    
    /// Shifts the entire slice as a SISO shift register with mutliple values to the left. The output is given in-place in `ìtems`.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use slice_ops::*;
    /// 
    /// let mut register = [4, 5, 6, 7, 8, 9];
    /// let mut io = [1, 2, 3];
    /// 
    /// register.shift_many_right(&mut io);
    /// 
    /// assert_eq!(register, [1, 2, 3, 4, 5, 6]);
    /// assert_eq!(io, [7, 8, 9]);
    /// ```
    fn shift_many_right(&mut self, items: &mut [T]);
    
    /// Shifts the entire slice as a SISO shift register to the left. The output is given in-place in `ìtem`.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use slice_ops::*;
    /// 
    /// let mut register = [4, 3, 2];
    /// let mut io = 1;
    /// 
    /// register.shift_left(&mut io);
    /// 
    /// assert_eq!(register, [3, 2, 1]);
    /// assert_eq!(io, 4);
    /// ```
    fn shift_left(&mut self, item: &mut T);

    /// Shifts the entire slice as a SISO shift register to the right. The output is given in-place in `ìtem`.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use slice_ops::*;
    /// 
    /// let mut register = [2, 3, 4];
    /// let mut io = 1;
    /// 
    /// register.shift_right(&mut io);
    /// 
    /// assert_eq!(register, [1, 2, 3]);
    /// assert_eq!(io, 4);
    /// ```
    fn shift_right(&mut self, item: &mut T);
}

impl<T> SliceShift<T> for [T]
{
    fn shift_many_left(&mut self, items: &mut [T])
    {
        let len = self.len();
        let m = items.len();
        let q = m.min(len);
        unsafe {
            items.rotate_left(m.saturating_sub(len));
            core::ptr::swap_nonoverlapping(self.as_mut_ptr(), items.as_mut_ptr(), q);
            self.rotate_left(q);
        }
    }
    
    fn shift_many_right(&mut self, items: &mut [T])
    {
        let len = self.len();
        let m = items.len();
        let q = m.min(len);
        unsafe {
            self.rotate_right(q);
            core::ptr::swap_nonoverlapping(self.as_mut_ptr(), items.as_mut_ptr(), q);
            items.rotate_right(m.saturating_sub(len));
        }
    }
    
    fn shift_left(&mut self, item: &mut T)
    {
        let l = self.len();
        if l <= 1
        {
            return;
        }
        let p = self.as_mut_ptr_range();
        unsafe {
            core::ptr::swap_nonoverlapping(p.start, item as *mut T, 1);

            let x = p.start.read();
            core::ptr::copy(p.start.add(1), p.start, l - 1);
            p.end.sub(1).write(x);
        }
    }

    fn shift_right(&mut self, item: &mut T)
    {
        let l = self.len();
        if l <= 1
        {
            return;
        }
        let p = self.as_mut_ptr_range();
        unsafe {
            let x = p.end.sub(1).read();
            core::ptr::copy(p.start, p.start.add(1), l - 1);
            p.start.write(x);

            core::ptr::swap_nonoverlapping(p.start, item as *mut T, 1);
        }
    }
}

#[cfg(test)]
mod test
{
    #[test]
    fn it_works()
    {
        
    }
}