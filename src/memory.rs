use crate::{println, registers};

const PAGE_SHIFT    : u64 = 12;
const TABLE_SHIFT 	: u64 =	9;
const SECTION_SHIFT	: u64 =	(PAGE_SHIFT + TABLE_SHIFT);
const PAGE_SIZE   	: u64 =	(1 << PAGE_SHIFT);
const SECTION_SIZE	: u64 =	(1 << SECTION_SHIFT);
const LOW_MEMORY    : u64 = (2 * SECTION_SIZE);
const HIGH_MEMORY   : u64 = registers::PBASE;
const PAGING_MEMORY : u64 = (HIGH_MEMORY - LOW_MEMORY);
const PAGING_PAGES  : u64 = (PAGING_MEMORY/PAGE_SIZE);
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