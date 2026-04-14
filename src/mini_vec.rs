use std::{
    alloc::{Layout, alloc, dealloc},
    ptr::{self, null_mut},
};

#[derive(Debug)]
pub struct MiniVec {
    ptr: *mut i32,
    len: usize,
    cap: usize,
}
impl MiniVec {
    pub fn new() -> Self {
        return Self {
            ptr: null_mut(),
            len: 0,
            cap: 0,
        };
    }

    pub fn push(&mut self, val: i32) {
        if self.len == self.cap {
            let new_cap = if self.cap == 0 { 1 } else { self.cap * 2 };
            let layout = Layout::array::<i32>(new_cap).unwrap();

            unsafe {
                let new_ptr = alloc(layout) as *mut i32;
                if new_ptr.is_null() {
                    println!("allocation failed");
                    return;
                }

                if !self.ptr.is_null() {
                    std::ptr::copy(self.ptr, new_ptr, self.len);
                    let old_layout = Layout::array::<i32>(self.cap).unwrap();
                    dealloc(self.ptr as *mut u8, old_layout);
                    /*
                     * dealloc frees raw bytes, not typed values
                     * That’s why it expects *mut u8.
                     */
                }

                self.ptr = new_ptr;
                self.cap = new_cap;
            }
        }
        unsafe {
            std::ptr::write(self.ptr.add(self.len), val);
        }
        self.len += 1;
    }

    pub fn get(&self, index: usize) -> Option<i32> {
        if index >= self.len {
            return None;
        }

        unsafe {
            return Some(*self.ptr.add(index));
        }
    }
}
