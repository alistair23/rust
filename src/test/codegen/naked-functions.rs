// compile-flags: -C no-prepopulate-passes

#![crate_type = "lib"]
#![feature(naked_functions)]

// CHECK: Function Attrs: naked
// CHECK-NEXT: define void @naked_empty()
#[no_mangle]
#[naked]
pub fn naked_empty() {
    // CHECK-NEXT: {{.+}}:
    // CHECK-NEXT: ret void
}

// CHECK: Function Attrs: naked
// CHECK-NEXT: define i{{[0-9]+}} @naked_with_return()
#[no_mangle]
#[naked]
pub fn naked_with_return() -> isize {
    // CHECK-NEXT: {{.+}}:
    // CHECK-NEXT: ret i{{[0-9]+}} 0
    0
}
