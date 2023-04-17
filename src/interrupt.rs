use crate::{print, println};
use crate::process::schedule;
use crate::registers;

#[no_mangle]
pub extern "C" fn put32(ptr: *mut u32, value: u32) {
    unsafe {
        ptr.write_volatile(value);
    }
}

#[no_mangle]
pub extern "C" fn get32(ptr: *const u32) -> u32 {
    unsafe {
        ptr.read_volatile()
    }
}

pub unsafe fn timer_init () {
    let mut time = get32(registers::TIMER_CLO as *mut u32);
    time = time + 20_0000;
    put32(registers::TIMER_C1 as *mut u32, time);
}

pub unsafe fn enable_interrupt_controller() {
    put32(registers::ENABLE_IRQS_1 as *mut u32, registers::SYSTEM_TIMER_IRQ_1);
}

unsafe fn handle_timer_irq() {
    let mut time = get32(registers::TIMER_CLO as *mut u32);
    time = time + 20_0000;
    put32(registers::TIMER_C1 as *mut u32, time);
    put32(registers::TIMER_CS as *mut u32, registers::TIMER_CS_M1);
    crate::enable_irq();
    schedule();
    crate::disable_irq();
}


#[no_mangle]
unsafe fn handle_irq() {
    let mut irq = get32(registers::IRQ_PENDING_1 as *mut u32);
    match irq {
        registers::SYSTEM_TIMER_IRQ_1 => handle_timer_irq(),
        _ => println!("unrecognized interrupt")
    }
}


#[no_mangle]
fn show_invalid_entry_message(interruptType : u64, esr: u64, address: u64) {

}