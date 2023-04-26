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
use crate::process::{copy_process, move_to_usermode, init_processes, schedule};
use crate::process::ProcessKind::KThread;

global_asm!(include_str!("get_el.s"));
global_asm!(include_str!("entryv1.s"));
global_asm!(include_str!("irq.s"));
global_asm!(include_str!("syscall.s"));

extern "C" {
    pub fn get_el() -> u64;
    pub fn irq_vector_init();
    pub fn enable_irq();
    pub fn disable_irq();
    pub fn delay(time : u64);
    pub fn call_sys_write();
}

unsafe fn user_process() {
    println!("In user process Exception level: {}", get_el());
}

unsafe fn KernelThread() {
    println!("Kernel Thread Started Exception Level: {}\n", get_el());
    let err = move_to_usermode(user_process as u64);
    if err < 0 {
        println!("error moving to user mode")
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
    let res1 = copy_process(KThread, KernelThread as u64, 0);
    if res1 != 0 {
        panic!("Kernel Thread fail to initiate");
    }

    loop {
        schedule();
    }
}
