.macro handle_invalid_entry type
kernel_entry
mov	x0, #\type
mrs	x1, esr_el1
mrs	x2, elr_el1
bl	show_invalid_entry_message
b	err_hang
.endm

.macro	ventry	label
.align	7
b	\label
.endm

.macro	kernel_entry
sub	sp, sp, #256              // stack frame size
stp	x0, x1, [sp, #16 * 0]
stp	x2, x3, [sp, #16 * 1]
stp	x4, x5, [sp, #16 * 2]
stp	x6, x7, [sp, #16 * 3]
stp	x8, x9, [sp, #16 * 4]
stp	x10, x11, [sp, #16 * 5]
stp	x12, x13, [sp, #16 * 6]
stp	x14, x15, [sp, #16 * 7]
stp	x16, x17, [sp, #16 * 8]
stp	x18, x19, [sp, #16 * 9]
stp	x20, x21, [sp, #16 * 10]
stp	x22, x23, [sp, #16 * 11]
stp	x24, x25, [sp, #16 * 12]
stp	x26, x27, [sp, #16 * 13]
stp	x28, x29, [sp, #16 * 14]
str	x30, [sp, #16 * 15]
.endm

.macro	kernel_exit
ldp	x0, x1, [sp, #16 * 0]
ldp	x2, x3, [sp, #16 * 1]
ldp	x4, x5, [sp, #16 * 2]
ldp	x6, x7, [sp, #16 * 3]
ldp	x8, x9, [sp, #16 * 4]
ldp	x10, x11, [sp, #16 * 5]
ldp	x12, x13, [sp, #16 * 6]
ldp	x14, x15, [sp, #16 * 7]
ldp	x16, x17, [sp, #16 * 8]
ldp	x18, x19, [sp, #16 * 9]
ldp	x20, x21, [sp, #16 * 10]
ldp	x22, x23, [sp, #16 * 11]
ldp	x24, x25, [sp, #16 * 12]
ldp	x26, x27, [sp, #16 * 13]
ldp	x28, x29, [sp, #16 * 14]
ldr	x30, [sp, #16 * 15]
add	sp, sp, #256
eret
.endm

/*
 * Exception vectors.
 */
.align	11
.globl vectors
vectors:
	ventry	sync_invalid_el1t			// Synchronous EL1t
	ventry	irq_invalid_el1t			// IRQ EL1t
	ventry	fiq_invalid_el1t			// FIQ EL1t
	ventry	error_invalid_el1t			// Error EL1t

	ventry	sync_invalid_el1h			// Synchronous EL1h
	ventry	el1_irq					    // IRQ EL1h
	ventry	fiq_invalid_el1h			// FIQ EL1h
	ventry	error_invalid_el1h			// Error EL1h

	ventry	sync_invalid_el0_64			// Synchronous 64-bit EL0
	ventry	irq_invalid_el0_64			// IRQ 64-bit EL0
	ventry	fiq_invalid_el0_64			// FIQ 64-bit EL0
	ventry	error_invalid_el0_64			// Error 64-bit EL0

	ventry	sync_invalid_el0_32			// Synchronous 32-bit EL0
	ventry	irq_invalid_el0_32			// IRQ 32-bit EL0
	ventry	fiq_invalid_el0_32			// FIQ 32-bit EL0
	ventry	error_invalid_el0_32		// Error 32-bit EL0

sync_invalid_el1t:
	handle_invalid_entry  0

irq_invalid_el1t:
	handle_invalid_entry  1

fiq_invalid_el1t:
	handle_invalid_entry  2

error_invalid_el1t:
	handle_invalid_entry  3

sync_invalid_el1h:
	handle_invalid_entry  4

fiq_invalid_el1h:
	handle_invalid_entry  5

error_invalid_el1h:
	handle_invalid_entry  6

sync_invalid_el0_64:
	handle_invalid_entry  7

irq_invalid_el0_64:
	handle_invalid_entry  8

fiq_invalid_el0_64:
	handle_invalid_entry  9

error_invalid_el0_64:
	handle_invalid_entry  10

sync_invalid_el0_32:
	handle_invalid_entry  11

irq_invalid_el0_32:
	handle_invalid_entry  12

fiq_invalid_el0_32:
	handle_invalid_entry  13

error_invalid_el0_32:
	handle_invalid_entry  14

el1_irq:
	kernel_entry
	bl	handle_irq
	kernel_exit

.globl err_hang
err_hang: b err_hang

.globl delay
delay:
	subs x0, x0, #1
	bne delay
	ret

// .macro handle_invalid_entry el, type
// kernel_entry \el
// mov	x0, #\type
// mrs	x1, esr_el1
// mrs	x2, elr_el1
// bl	show_invalid_entry_message
// b	err_hang
// .endm
//
// .macro	ventry	label
// .align	7
// b	\label
// .endm
//
// .macro	kernel_entry, el
// sub	sp, sp, #272
// stp	x0, x1, [sp, #16 * 0]
// stp	x2, x3, [sp, #16 * 1]
// stp	x4, x5, [sp, #16 * 2]
// stp	x6, x7, [sp, #16 * 3]
// stp	x8, x9, [sp, #16 * 4]
// stp	x10, x11, [sp, #16 * 5]
// stp	x12, x13, [sp, #16 * 6]
// stp	x14, x15, [sp, #16 * 7]
// stp	x16, x17, [sp, #16 * 8]
// stp	x18, x19, [sp, #16 * 9]
// stp	x20, x21, [sp, #16 * 10]
// stp	x22, x23, [sp, #16 * 11]
// stp	x24, x25, [sp, #16 * 12]
// stp	x26, x27, [sp, #16 * 13]
// stp	x28, x29, [sp, #16 * 14]
//
// .if	\el == 0
// mrs	x21, sp_el0
// .else
// add	x21, sp, #S_FRAME_SIZE
// .endif /* \el == 0 */
//
// mrs	x22, elr_el1
// mrs	x23, spsr_el1
//
// stp	x30, x21, [sp, #16 * 15]
// stp	x22, x23, [sp, #16 * 16]
// .endm
//
// .macro	kernel_exit, el
// ldp	x22, x23, [sp, #16 * 16]
// ldp	x30, x21, [sp, #16 * 15]
//
// .if	\el == 0
// msr	sp_el0, x21
// .endif /* \el == 0 */
//
// msr	elr_el1, x22
// msr	spsr_el1, x23
//
//
// ldp	x0, x1, [sp, #16 * 0]
// ldp	x2, x3, [sp, #16 * 1]
// ldp	x4, x5, [sp, #16 * 2]
// ldp	x6, x7, [sp, #16 * 3]
// ldp	x8, x9, [sp, #16 * 4]
// ldp	x10, x11, [sp, #16 * 5]
// ldp	x12, x13, [sp, #16 * 6]
// ldp	x14, x15, [sp, #16 * 7]
// ldp	x16, x17, [sp, #16 * 8]
// ldp	x18, x19, [sp, #16 * 9]
// ldp	x20, x21, [sp, #16 * 10]
// ldp	x22, x23, [sp, #16 * 11]
// ldp	x24, x25, [sp, #16 * 12]
// ldp	x26, x27, [sp, #16 * 13]
// ldp	x28, x29, [sp, #16 * 14]
// add	sp, sp, #272
// eret
// .endm
//
//
// /*
//  * Exception vectors.
//  */
// .align	11
// .globl vectors
// vectors:
// 	ventry	sync_invalid_el1t			// Synchronous EL1t
// 	ventry	irq_invalid_el1t			// IRQ EL1t
// 	ventry	fiq_invalid_el1t			// FIQ EL1t
// 	ventry	error_invalid_el1t			// Error EL1t
//
// 	ventry	sync_invalid_el1h			// Synchronous EL1h
// 	ventry	el1_irq					// IRQ EL1h
// 	ventry	fiq_invalid_el1h			// FIQ EL1h
// 	ventry	error_invalid_el1h			// Error EL1h
//
// 	ventry	el0_sync				// Synchronous 64-bit EL0
// 	ventry	el0_irq					// IRQ 64-bit EL0
// 	ventry	fiq_invalid_el0_64			// FIQ 64-bit EL0
// 	ventry	error_invalid_el0_64			// Error 64-bit EL0
//
// 	ventry	sync_invalid_el0_32			// Synchronous 32-bit EL0
// 	ventry	irq_invalid_el0_32			// IRQ 32-bit EL0
// 	ventry	fiq_invalid_el0_32			// FIQ 32-bit EL0
// 	ventry	error_invalid_el0_32			// Error 32-bit EL0
//
// sync_invalid_el1t:
// 	handle_invalid_entry 1,  0
//
// irq_invalid_el1t:
// 	handle_invalid_entry  1, 1
//
// fiq_invalid_el1t:
// 	handle_invalid_entry  1, 2
//
// error_invalid_el1t:
// 	handle_invalid_entry  1, 3
//
// sync_invalid_el1h:
// 	handle_invalid_entry 1, 4
//
// fiq_invalid_el1h:
// 	handle_invalid_entry  1, 7
//
// error_invalid_el1h:
// 	handle_invalid_entry  1, 8
//
// fiq_invalid_el0_64:
// 	handle_invalid_entry  0, 10
//
// error_invalid_el0_64:
// 	handle_invalid_entry  0, 11
//
// sync_invalid_el0_32:
// 	handle_invalid_entry  0, 12
//
// irq_invalid_el0_32:
// 	handle_invalid_entry  0, 13
//
// fiq_invalid_el0_32:
// 	handle_invalid_entry  0, 14
//
// error_invalid_el0_32:
// 	handle_invalid_entry  0, 15
//
// el1_irq:
// 	kernel_entry 1
// 	bl	handle_irq
// 	kernel_exit 1
//
// el0_irq:
// 	kernel_entry 0
// 	bl	handle_irq
// 	kernel_exit 0
//
// el0_sync:
// 	kernel_entry 0
// 	mrs	x25, esr_el1				// read the syndrome register
// 	lsr	x24, x25, #26		        // exception class
// 	cmp	x24, #0x15			        // SVC in 64-bit state
// 	b.eq	el0_svc
// 	handle_invalid_entry 0, 16
//
// sc_nr	.req	x25					// number of system calls
// scno	.req	x26					// syscall number
// stbl	.req	x27					// syscall table pointer
//
// el0_svc:
// 	adr	stbl, sys_call_table	    // load syscall table pointer
// 	uxtw	scno, w8				// syscall number in w8
// 	mov	sc_nr, #4                   // number of syscalls
// 	bl	enable_irq
// 	cmp     scno, sc_nr             // check upper syscall limit
// 	b.hs	ni_sys
//
// 	ldr	x16, [stbl, scno, lsl #3]		// address in the syscall table
// 	blr	x16					// call sys_* routine
// 	b	ret_from_syscall
// ni_sys:
// 	handle_invalid_entry 0, 17
//
//
// ret_from_syscall:
// 	bl	disable_irq
// 	str	x0, [sp, #0]				// returned x0
// 	kernel_exit 0
//
// .globl ret_from_fork
// ret_from_fork:
// 	cbz	x19, ret_to_user			// not a kernel thread
// 	mov	x0, x20
// 	blr	x19
// ret_to_user:
// 	bl disable_irq
// 	kernel_exit 0
//
// .globl err_hang
// err_hang: b err_hang
//
//
// .globl delay
// delay:
// 	subs x0, x0, #1
// 	bne delay
// 	ret