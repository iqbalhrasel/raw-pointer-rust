pub struct MiniVec<T> {
    ptr: *mut T,
    len: usize,
    cap: usize,
}
