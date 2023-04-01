pub const PBASE : u64 = 0x3F000000;
// interrupt registers
pub const IRQ_BASIC_PENDING	    : u64 = PBASE+0x0000B200;
pub const IRQ_PENDING_1		    : u64 = PBASE+0x0000B204;
pub const IRQ_PENDING_2		    : u64 = PBASE+0x0000B208;
pub const FIQ_CONTROL		    : u64 = PBASE+0x0000B20C;
pub const ENABLE_IRQS_1		    : u64 = PBASE+0x0000B210;
pub const ENABLE_IRQS_2		    : u64 = PBASE+0x0000B214;
pub const ENABLE_BASIC_IRQS	    : u64 = PBASE+0x0000B218;
pub const DISABLE_IRQS_1		: u64 = PBASE+0x0000B21C;
pub const DISABLE_IRQS_2		: u64 = PBASE+0x0000B220;
pub const DISABLE_BASIC_IRQS	: u64 = PBASE+0x0000B224; // timer registers
pub const TIMER_CS              : u64 =  PBASE+0x00003000;
pub const TIMER_CLO             : u64 =  PBASE+0x00003004;
pub const TIMER_CHI             : u64 =  PBASE+0x00003008;
pub const TIMER_C0              : u64 =  PBASE+0x0000300C;
pub const TIMER_C1              : u64 =  PBASE+0x00003010;
pub const TIMER_C2              : u64 =  PBASE+0x00003014;
pub const TIMER_C3              : u64 =  PBASE+0x00003018;
// bit masks to mask over interrupt registers
pub const SYSTEM_TIMER_IRQ_0	: u32  = 1 << 0;
pub const SYSTEM_TIMER_IRQ_1	: u32 = 1 << 1;
pub const SYSTEM_TIMER_IRQ_2	: u32 = 1 << 2;
pub const SYSTEM_TIMER_IRQ_3	: u32 = 1 << 3;
// bit mask to mask over timers registers
pub const TIMER_CS_M0	 : u32 =  1 << 0;
pub const TIMER_CS_M1	 : u32 =  1 << 1;
pub const TIMER_CS_M2	 : u32 =  1 << 2;
pub const TIMER_CS_M3	 : u32 =  1 << 3;