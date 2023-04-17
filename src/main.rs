#![feature(format_args_nl)]
#![feature(asm_const)]
#![feature(panic_info_message)]
#![no_main]
#![no_std]

mod panic;
mod boot;
mod print;
mod interrupt;
mod registers;
mod memory;
mod process;
mod synchronization;
mod syscall;

use core::arch::global_asm;
use crate::interrupt::{enable_interrupt_controller, timer_init};
use crate::process::{copy_process, init_processes, schedule};

global_asm!(include_str!("get_el.s"));
global_asm!(include_str!("entry.s"));
global_asm!(include_str!("irq.s"));
global_asm!(include_str!("syscall.s"));


extern "C" {
    pub fn get_el() -> u64;
    pub fn irq_vector_init();
    pub fn enable_irq();
    pub fn disable_irq();
    pub fn delay(time : u64);
}



unsafe fn TaskOne() {
    println!("Task 1 Start\n");
    loop {
        println!("Task 1 run");
        delay(200_000);
    }
}

unsafe fn TaskTwo() {
    println!("Task 2 Start\n");
    loop {
        println!("Task 2 run");
        delay(200_000);
    }
}

#[no_mangle]
unsafe fn kernel_init() -> ! {
    irq_vector_init();
    timer_init();
    enable_interrupt_controller();
    enable_irq();
    init_processes();
    println!("           _____                    _____                    _____                _____                           _______                   _____");
    println!("          /\\    \\                  /\\    \\                  /\\    \\              /\\    \\                         /::\\    \\                 /\\    \\");
    println!("         /::\\    \\                /::\\____\\                /::\\    \\            /::\\    \\                       /::::\\    \\               /::\\    \\");
    println!("        /::::\\    \\              /:::/    /               /::::\\    \\           \\:::\\    \\                     /::::::\\    \\             /::::\\    \\");
    println!("       /::::::\\    \\            /:::/    /               /::::::\\    \\           \\:::\\    \\                   /::::::::\\    \\           /::::::\\    \\");
    println!("      /:::/\\:::\\    \\          /:::/    /               /:::/\\:::\\    \\           \\:::\\    \\                 /:::/~~\\:::\\    \\         /:::/\\:::\\    \\");
    println!("     /:::/__\\:::\\    \\        /:::/    /               /:::/__\\:::\\    \\           \\:::\\    \\               /:::/    \\:::\\    \\       /:::/__\\:::\\    \\");
    println!("    /::::\\   \\:::\\    \\      /:::/    /                \\:::\\   \\:::\\    \\          /::::\\    \\             /:::/    / \\:::\\    \\      \\:::\\   \\:::\\    \\");
    println!("   /::::::\\   \\:::\\    \\    /:::/    /      _____    ___\\:::\\   \\:::\\    \\        /::::::\\    \\           /:::/____/   \\:::\\____\\   ___\\:::\\   \\:::\\    \\");
    println!("  /:::/\\:::\\   \\:::\\____\\  /:::/____/      /\\    \\  /\\   \\:::\\   \\:::\\    \\      /:::/\\:::\\    \\         |:::|    |     |:::|    | /\\   \\:::\\   \\:::\\    \\");
    println!(" /:::/  \\:::\\   \\:::|    ||:::|    /      /::\\____\\/::\\   \\:::\\   \\:::\\____\\    /:::/  \\:::\\____\\        |:::|____|     |:::|    |/::\\   \\:::\\   \\:::\\____\\");
    println!(" \\::/   |::::\\  /:::|____||:::|____\\     /:::/    /\\:::\\   \\:::\\   \\::/    /   /:::/    \\::/    /         \\:::\\    \\   /:::/    / \\:::\\   \\:::\\   \\::/    /");
    println!("  \\/____|:::::\\/:::/    /  \\:::\\    \\   /:::/    /  \\:::\\   \\:::\\   \\/____/   /:::/    / \\/____/           \\:::\\    \\ /:::/    /   \\:::\\   \\:::\\   \\/____/");
    println!("        |:::::::::/    /    \\:::\\    \\ /:::/    /    \\:::\\   \\:::\\    \\      /:::/    /                     \\:::\\    /:::/    /     \\:::\\   \\:::\\    \\");
    println!("        |::|\\::::/    /      \\:::\\    /:::/    /      \\:::\\   \\:::\\____\\    /:::/    /                       \\:::\\__/:::/    /       \\:::\\   \\:::\\____\\");
    println!("        |::| \\::/____/        \\:::\\__/:::/    /        \\:::\\  /:::/    /    \\::/    /                         \\::::::::/    /         \\:::\\  /:::/    /");
    println!("        |::|  ~|               \\::::::::/    /          \\:::\\/:::/    /      \\/____/                           \\::::::/    /           \\:::\\/:::/    /");
    println!("        |::|   |                \\::::::/    /            \\::::::/    /                                          \\::::/    /             \\::::::/    /");
    println!("        \\::|   |                 \\::::/    /              \\::::/    /                                            \\::/____/               \\::::/    /");
    println!("         \\:|   |                  \\::/____/                \\::/    /                                              ~~                      \\::/    /");
    println!("          \\|___|                   ~~                       \\/____/                                                                        \\/____/");

    let res1 = copy_process(TaskOne as u64, 0);
    if res1 != 0 {
        panic!("Task 1 fail to initiate");
    }
    let res2 = copy_process(TaskTwo as u64, 0);
    if res2 != 0 {
        panic!("Task 2 fail to initiate");
    }

    loop {
        //println!("in kernel loop\n");
        schedule();
    }
}
