use crate::{println, registers};

pub const PAGE_SHIFT    : u64 = 12;
pub const TABLE_SHIFT 	: u64 =	9;
pub const SECTION_SHIFT	: u64 =	(PAGE_SHIFT + TABLE_SHIFT);
pub const PAGE_SIZE   	: u64 =	(1 << PAGE_SHIFT);
pub const SECTION_SIZE	: u64 =	(1 << SECTION_SHIFT);
pub const LOW_MEMORY    : u64 = (2 * SECTION_SIZE);
pub const HIGH_MEMORY   : u64 = registers::PBASE;
pub const PAGING_MEMORY : u64 = (HIGH_MEMORY - LOW_MEMORY);
pub const PAGING_PAGES  : u64 = (PAGING_MEMORY/PAGE_SIZE);
static mut MEM_MAP : [u64; PAGING_PAGES as usize] = [0; PAGING_PAGES as usize];

pub fn get_free_page() -> u64 {
    unsafe {
        for i in 0..PAGING_PAGES {
            if MEM_MAP[i as usize] == 0 {
                MEM_MAP[i as usize] = 1;
                return LOW_MEMORY + (i as u64 * PAGE_SIZE);
            }
        }
        0
    }
}

pub fn free_page(p: u64) {
    unsafe {
        let index = (p - LOW_MEMORY) / PAGE_SIZE;
        MEM_MAP[index as usize] = 0;
    }
}