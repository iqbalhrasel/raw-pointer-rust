use crate::{
    basic::{node_rw, rw_ptr_math, rw_slice},
    menual_mem::{men_mem, raw_slice, struct_mem},
    mini_linkedlst::LinkedList,
    mini_vec::MiniVec,
};

mod basic;
mod menual_mem;
mod mini_linkedlst;
mod mini_vec;

fn main() {
    rw_slice();
}
