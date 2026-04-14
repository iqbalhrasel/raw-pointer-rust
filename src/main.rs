use crate::{
    basic::rw_ptr_math,
    menual_mem::{men_mem, raw_slice, struct_mem},
    mini_vec::MiniVec,
};

mod basic;
mod menual_mem;
mod mini_vec;

fn main() {
    let mut v = MiniVec::new();
    v.push(10);
    v.push(20);
    v.push(30);
    v.push(40);

    println!("{:?}", v.get(2));

    println!("{:?}", v);
}
