#![feature(format_args_nl)]
#![feature(asm_const)]
#![no_main]
#![no_std]

mod panic;
mod boot;
mod print;
mod interrupt;
mod registers;

use core::arch::global_asm;
use crate::interrupt::{enable_interrupt_controller, timer_init};

global_asm!(include_str!("get_el.s"));
global_asm!(include_str!("entry.s"));
global_asm!(include_str!("irq.s"));

extern "C" {
    fn get_el() -> u64;
    fn irq_vector_init();
    fn enable_irq();
}

#[no_mangle]
unsafe fn kernel_init() -> ! {
    irq_vector_init();
    timer_init();
    enable_interrupt_controller();
    enable_irq();
    println!("Hello rust os v2");
    let e = get_el();
    println!("Exception  level {}", e);
    loop {}
}
