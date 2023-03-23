#![feature(format_args_nl)]

#![no_main]
#![no_std]

mod panic;
mod boot;
mod print;

unsafe fn kernel_init() -> ! {
    println!("Hello rust os v2");
    println!("Hello rust 3");
    loop {}
}
