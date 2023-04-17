use core::arch::global_asm;
use crate::memory::get_free_page;
use crate::println;
use crate::synchronization::interface::Mutex;
use crate::synchronization::NullLock;


global_asm!(include_str!("cpu_switch.s"));

extern "C" {
    fn ret_from_fork() -> !;
    fn cpu_switch_to(a: u64, b: u64);
}

#[repr(C)]
pub struct CPU_Context {
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
pub struct Task_Struct {
    context : CPU_Context,
    state : u64,
    counter : u64,
    priority : u64,
    preempt_count : u64,
}


struct ProcessesInner {
    currentTask : u64,
    Tasks : [u64; 5],
    PID : usize,
    cPID : usize
}

unsafe impl Send for ProcessesInner {}
unsafe impl Sync for ProcessesInner {}

pub struct Processes {
    inner : NullLock<ProcessesInner>,
}

impl Task_Struct {
    const fn new() -> Task_Struct {
        Task_Struct {
            context : CPU_Context {
                x19 : 0,
                x20 : 0,
                x21 : 0,
                x22 : 0,
                x23 : 0,
                x24 : 0,
                x25 : 0,
                x26 : 0,
                x27 : 0,
                x28 : 0,
                fp  : 0,
                sp  : 0,
                pc  : 0,
            },
            state: 0,
            counter: 0,
            priority: 0,
            preempt_count: 0,
        }
    }
}

fn init_processes_inner() -> ProcessesInner {
    let INIT_TASK = Task_Struct::new();
    ProcessesInner {
        currentTask: &INIT_TASK as *const _ as u64, // Cast to *mut Task_Struct
        Tasks: [
            &INIT_TASK as *const _ as u64, // Cast to *mut Task_Struct
            0,
            0,
            0,
            0,
        ],
        PID: 0,
        cPID: 0
    }
}


static mut PROCESSES: Option<Processes> = None;

pub fn init_processes() {
    unsafe {
        PROCESSES = Some(Processes {
            inner: NullLock::new(init_processes_inner()),
        });
    }
}

pub fn get_processes() -> &'static Processes {
    unsafe {
        match &PROCESSES {
            Some(processes) => processes,
            None => panic!("PROCESSES not initialized"),
        }
    }
}

enum ProcessKind {
    KThread,
    UThread
}

pub fn copy_process(function_pointer : u64, arg: u64) -> i32 {

    let Task : *mut Task_Struct;
    Task = get_free_page() as *mut Task_Struct;
    if Task.is_null() {
        return 1;
    }
    unsafe {
        get_processes().inner.lock(|inner| {
            let currentTask = inner.currentTask as *mut Task_Struct;
            (*Task).priority = (*currentTask).priority;
            (*Task).state = (*currentTask).state;
            (*Task).preempt_count = 1;
            (*Task).context.x19 = function_pointer;
            (*Task).context.x20 = arg;
            (*Task).context.sp = Task as u64 + 4096;
            (*Task).context.pc = ret_from_fork as u64;
            inner.PID += 1;
            inner.Tasks[inner.PID] = Task as *const _ as u64;
        })
    }

    return 0;
}

pub fn switch_to(next : u64) {
    get_processes().inner.lock(|inner| unsafe {
        if inner.currentTask == next {
            return;
        }
        let prev = inner.currentTask;
        inner.currentTask = next;
        cpu_switch_to(prev, next);
    })
}


pub fn schedule() {
    get_processes().inner.lock(|inner| {
         if inner.cPID == 0 {
             println!("switching to process Task 1");
             switch_to(inner.Tasks[1]);
             inner.cPID = 1;
         } else if inner.cPID == 1 {
             println!("switching to process Task 2");
             switch_to(inner.Tasks[2]);
             inner.cPID = 2;
         } else if inner.cPID == 2 {
             println!("switching to process kernel thread");
             switch_to(inner.Tasks[0]);
             inner.cPID = 0;
         }
    })
}
