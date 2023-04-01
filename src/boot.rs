use core::arch::global_asm;
use crate::println;

// Assembly counterpart to this file.
global_asm!(
    include_str!("boot.s"),
    SCTLR_VALUE_MMU_DISABLED = const ((3 << 28) | (3 << 22) | (1 << 20) | (1 << 11) | (0 << 25) | (0 << 24) | (0 << 12) | (0 << 2) | (0 << 0)),
    CNTH_CTL_EL2_VALUE = const ((1 << 1) | (1 << 0)),
    HCR_VALUE = const (1 << 31),
    SPSR_EL2_VALUE = const ((1 << 9) | (1 << 8) | (1 << 7) | (1 << 6) | (5 << 0))
);

/// The Rust entry of the `kernel` binary.
///
/// The function is called from the assembly `_start` function.
#[no_mangle]
pub unsafe fn _start_rust(phys_boot_core_stack_end_exclusive_addr: u64) -> ! {
    println!("boot core stack address : {}", phys_boot_core_stack_end_exclusive_addr);
    crate::kernel_init()
}
