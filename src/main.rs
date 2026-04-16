use crate::ffi_basics::add;

mod basic;
mod ffi_basics;
mod menual_mem;
mod mini_linkedlst;
mod mini_vec;

fn main() {
    let n = unsafe { add(52, 30) };

    println!("{}", n);
}
