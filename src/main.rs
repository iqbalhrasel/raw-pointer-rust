use std::ptr::{null, null_mut};

fn rw_ptr_const() {
    let x = 10;
    let ptr = &x as *const i32;

    unsafe {
        println!("{}", *ptr);
    }
}

fn rw_ptr_mut() {
    let mut x = 10;
    let ptr = &mut x as *mut i32;

    unsafe {
        *ptr = 20;
    }
    println!("{}", x);
}

fn rw_ptr_null() {
    let x = 5;
    let mut ptr: *const i32 = null();

    if ptr.is_null() {
        println!("null pointer");
    } else {
        unsafe {
            println!("{:?}", *ptr);
        }
    }

    ptr = &x as *const i32;

    if ptr.is_null() {
        println!("null pointer");
    } else {
        unsafe {
            println!("{:?}", *ptr);
        }
    }
}

fn rw_ptr_arithmatic() {
    let arr = [10, 20, 30];
    let ptr = arr.as_ptr();

    unsafe {
        println!("{:?}", *ptr);
        println!("{:?}", *ptr.add(1));
        println!("{:?}", *ptr.add(2));
    }
}

fn rw_ptr_mut_array() {
    let mut arr = [10, 20, 30];
    let ptr = arr.as_mut_ptr();

    unsafe {
        *ptr.add(2) = 33;
    }

    for n in arr.iter() {
        println!("{}", n);
    }
}

fn rw_ptr_to_ref() {
    let x = 10;
    let ptr = &x as *const i32;

    unsafe {
        let r = &*ptr;
        println!("{}", r);
    }
}

fn rw_ptr_dangling_ptr() {
    let ptr;

    {
        let x = 55;
        ptr = &x as *const i32;
    }

    unsafe {
        println!("{}", *ptr);
    }
}

// raw pointer in struct
struct Node {
    val: i32,
    next: *mut Node,
}
impl Node {
    fn new() -> Self {
        return Self {
            val: 10,
            next: null_mut(), //std::ptr::null_mut()
        };
    }
}
// raw pointer in struct

fn rw_ptr_ptr_read() {
    let x = 12;
    let ptr = &x as *const i32;

    unsafe {
        let val = std::ptr::read(ptr); //copy bytes out of that address into a new owned value
        println!("{}", val);
    }
}

fn rw_ptr_ptr_write() {
    let mut x = 12;
    let ptr = &mut x as *mut i32;

    unsafe {
        std::ptr::write(ptr, 20); //place this value directly into that memory address
    }
    println!("{}", x);
}

fn rw_ptr_ptr_copy() {
    let arr1 = [10, 20, 30];
    let mut arr2 = [0; 3];

    unsafe {
        std::ptr::copy(arr1.as_ptr(), arr2.as_mut_ptr(), 3); //It copies bytes element-by-element
    }
    println!("{:?}", arr2);
}

fn main() {
    rw_ptr_ptr_copy();
}
