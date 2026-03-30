use std::ptr::null;

fn main() {
    rw_ptr_const();
    rw_ptr_mut();
    rw_ptr_null();
    rw_ptr_arithmatic();
}

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
    }
}
