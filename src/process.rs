#[repr(C)]
struct CPU_Context {
     x19 : u64,
     x20 : u64,
     x21 : u64,
     x22 : u64,
     x23 : u64,
     x24 : u64,
     x25 : u64,
     x26 : u64,
     x27 : u64,
     x28 : u64,
     fp  : u64,
     sp  : u64,
     pc  : u64,
}

#[repr(C)]
struct Task_Struct {
    context : CPU_Context,
    state : u64,
    counter : u64,
    priority : u64,
    preempt_count : u64,
}


static currentTask: Task_Struct = Task_Struct {
    context : {},
    state: 0,
    counter: 0,
    priority: 0,
    preempt_count: 0,
};

// in
fn timer_tick() {

}