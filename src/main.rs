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

use core::arch::global_asm;
use crate::interrupt::{enable_interrupt_controller, timer_init};
use crate::process::{copy_process, init_processes, schedule};

global_asm!(include_str!("get_el.s"));
global_asm!(include_str!("entry.s"));
global_asm!(include_str!("irq.s"));

extern "C" {
    fn get_el() -> u64;
    fn irq_vector_init();
    fn enable_irq();
    fn delay(time : u64);
}



unsafe fn TaskOne() {
    loop {
        println!("Task 1 run\\\\n");
        delay(200_000);
    }
}

unsafe fn TaskTwo() {
    loop {
        println!("Task 2 run\\\\n");
        delay(200_000_000);
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


    //let res1 = copy_process(TaskOne as u64, 0);
    //if res1 != 0 {
    //    panic!("Task 1 fail to initiate");
    //}
    //let res2 = copy_process(TaskTwo as u64, 0);
    //if res2 != 0 {
    //    panic!("Task 2 fail to initiate");
    //}

    loop {
        //println!("kernel init");
        schedule();
    }
}
