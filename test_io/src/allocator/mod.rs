use x86_64::{
    addr::VirtAddr,
    registers::control::Cr3,
    structures::paging::{
        mapper::MapToError, FrameAllocator, Mapper, Page, PageTableFlags, Size4KiB,
    },
};
//use core::ptr::null_mut;
//Will be removed in favor of a custom allocator in the future
pub mod linked_list;
use crate::syscall;
use core;
/// The start adress of the heap.
pub const HEAP_START: usize = 0x8888_0000;
/// The size of the heap. It is for now pretty small.
pub const HEAP_SIZE: usize = 100 * 1024;

#[alloc_error_handler]
fn alloc_error_handler(layout: core::alloc::Layout) -> ! {
    panic!("Allocation error : {:?}", layout)
}

use linked_list::LinkedListAllocator;

#[global_allocator]
static ALLOCATOR: Locked<LinkedListAllocator> = Locked::new(LinkedListAllocator::new());

/// Inits the Allocator, responsible for the...
///
/// TODO : continue working on this
pub fn init() {
    unsafe {
        crate::syscall(20, 69, 0, 0);
        let mut a = ALLOCATOR.lock();
        crate::syscall(20, 70, 0, 0);
        a.init(HEAP_START, HEAP_SIZE);
    }
}

pub struct Locked<A> {
    inner: spin::Mutex<A>,
}

impl<A> Locked<A> {
    pub const fn new(inner: A) -> Self {
        Locked {
            inner: spin::Mutex::new(inner),
        }
    }

    pub fn lock(&self) -> spin::MutexGuard<A> {
        self.inner.lock()
    }
}

fn align_up(addr: usize, align: usize) -> usize {
    (addr + align - 1) & !(align - 1)
}
