#![feature(asm, naked_functions)]

#[naked] //~ ERROR E0051
#[no_mangle]
#[inline(never)]
pub extern "C" fn naked_test(_fubar: u64) {
    unsafe { asm!("int3") }
}

pub fn main() {
    naked_test(1);
}
