use crate::{
    basic::rw_ptr_math,
    menual_mem::{men_mem, raw_slice, struct_mem},
    mini_linkedlst::LinkedList,
    mini_vec::MiniVec,
};

mod basic;
mod menual_mem;
mod mini_linkedlst;
mod mini_vec;

fn main() {
    let mut list = LinkedList::new();
    list.add_last(10);
    list.add_last(20);
    list.add_last(30);
    list.add_first(5);

    println!("{:?}", list.get(39));
}
