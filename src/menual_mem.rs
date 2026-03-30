use std::alloc::{Layout, alloc};

pub fn men_mem() {
    let layout = Layout::new::<i32>();

    unsafe {
        let ptr = alloc(layout) as *mut i32;
        *ptr = 32;
        println!("{}", *ptr);
    }
}

pub struct User {
    id: i32,
    name: String,
}

pub fn struct_mem() {
    let u = User {
        id: 3,
        name: String::from("john"),
    };

    let ptr = &u as *const User;

    unsafe {
        println!("{}", (*ptr).id);
        println!("{}", (*ptr).name);
    }
}

pub fn raw_slice() {
    let arr = [10, 20, 30];
    let ptr = arr.as_ptr();

    let slice = unsafe { std::slice::from_raw_parts(ptr, 2) };

    println!("{:?}", slice);
}
