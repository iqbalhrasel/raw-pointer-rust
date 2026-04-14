use std::ptr;

#[derive(Debug)]
struct Node {
    val: i32,
    next: *mut Node,
}

#[derive(Debug)]
pub struct LinkedList {
    head: *mut Node,
    tail: *mut Node,
    count: u32,
}

impl LinkedList {
    pub fn new() -> Self {
        return Self {
            head: ptr::null_mut(),
            tail: ptr::null_mut(),
            count: 0,
        };
    }

    pub fn add_last(&mut self, val: i32) {
        let node = Box::into_raw(Box::new(Node {
            val,
            next: ptr::null_mut(),
        }));

        if self.head.is_null() {
            self.head = node;
            self.tail = node;
        } else {
            unsafe {
                (*self.tail).next = node;
            }
            self.tail = node;
        }

        self.count += 1;
    }

    pub fn add_first(&mut self, val: i32) {
        let node = Box::into_raw(Box::new(Node {
            val,
            next: self.head,
        }));

        if self.head.is_null() {
            self.head = node;
            self.tail = node;
        } else {
            self.head = node;
        }

        self.count += 1;
    }

    pub fn get(&self, index: u32) -> Option<i32> {
        if index >= self.count {
            return None;
        }

        let mut curr = self.head;
        let mut i = 0;

        while !curr.is_null() {
            unsafe {
                if i == index {
                    return Some((*curr).val);
                }
                curr = (*curr).next;
            }
            i += 1;
        }

        None
    }
}

impl Drop for LinkedList {
    fn drop(&mut self) {
        let mut curr = self.head;

        while !curr.is_null() {
            unsafe {
                let next = (*self.head).next;
                Box::from_raw(curr);
                curr = next;
            }
        }

        self.head = ptr::null_mut();
        self.tail = ptr::null_mut();
        self.count = 0;
    }
}
