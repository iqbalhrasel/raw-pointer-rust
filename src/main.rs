use crate::{
    basic::rw_ptr_math,
    menual_mem::{men_mem, raw_slice, struct_mem},
    mini_vec::MiniVec,
};

mod basic;
mod menual_mem;
mod mini_vec;

fn main() {
    let v = MiniVec::new();
}
