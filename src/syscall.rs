use crate::println;

#[no_mangle]
fn sys_write() {
    println!("sys write called");
}

#[no_mangle]
fn sys_clone(stack: usize) -> i32 {
    println!("sys clone called");
    0
}

#[no_mangle]
fn sys_malloc() -> usize {
    println!("sys malloc called");
    0
}

#[no_mangle]
fn sys_exit() {
    println!("sys exit called");
}
