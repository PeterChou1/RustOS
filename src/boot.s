// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright (c) 2021-2022 Andre Richter <andre.o.richter@gmail.com>

//--------------------------------------------------------------------------------------------------
// Definitions
//--------------------------------------------------------------------------------------------------

// Load the address of a symbol into a register, PC-relative.
//
// The symbol must lie within +/- 4 GiB of the Program Counter.
//
// # Resources
//
// - https://sourceware.org/binutils/docs-2.36/as/AArch64_002dRelocations.html
.macro ADR_REL register, symbol
	adrp	\register, \symbol
	add	\register, \register, #:lo12:\symbol
.endm

//--------------------------------------------------------------------------------------------------
// Public Code
//--------------------------------------------------------------------------------------------------
.section .text._start

//------------------------------------------------------------------------------
// fn _start()
//------------------------------------------------------------------------------
_start:
	// Only proceed on the boot core. Park it otherwise.
	mrs	x0, MPIDR_EL1
	and	x0, x0, #0x3
	cmp	x0,  #0x0
	b.ne	.L_parking_loop

	// If execution reaches here, it is the boot core.

	// Initialize DRAM.
	ADR_REL	x0, __bss_start
	ADR_REL x1, __bss_end_exclusive


.L_bss_init_loop:
	cmp	x0, x1
	b.eq switch_exception_level_el2
	stp	xzr, xzr, [x0], #16
	b	.L_bss_init_loop


switch_exception_level_el2:
    // Enable timer counter register for EL1
    ldr     x0, ={SCTLR_VALUE_MMU_DISABLED}
    msr     sctlr_el1, x0

    ldr     x0, ={CNTH_CTL_EL2_VALUE}
    msr     cnthctl_el2, x0

    // No offset for reading the counters.
    ldr     x0, #0
    msr     cntvoff_el2, x0

    // Set EL1 execution state to AArch64.
    ldr     x0, ={HCR_VALUE}
    msr     hcr_el2, x0

    // Set up a simulated exception return.
    ldr     x0, ={SPSR_EL2_VALUE}
    msr     spsr_el2, x0

    // Set the EL1 entry point
    adr     x0, .L_prepare_rust
    msr     elr_el2, x0

    // Perform the exception return to EL1
    eret

.L_prepare_rust:
	// Set the stack pointer.
	ADR_REL	x0, __boot_core_stack_end_exclusive
	mov	sp, x0
	// Jump to Rust code.
	b	_start_rust

	// Infinitely wait for events (aka "park the core").
.L_parking_loop:
	wfe
	b	.L_parking_loop

.size	_start, . - _start
.type	_start, function
.global	_start
