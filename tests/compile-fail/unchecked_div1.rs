#![feature(core_intrinsics)]
fn main() {
    // MIN/-1 cannot be represented
    unsafe { std::intrinsics::unchecked_div(i16::MIN, -1); } //~ ERROR Overflow executing `unchecked_div`
}
