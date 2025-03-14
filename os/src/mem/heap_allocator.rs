use super::buddy_allocator::LockedHeap;
use crate::config::KERNEL_HEAP_SIZE;

#[global_allocator]
static HEAP_ALLOCATOR: LockedHeap = LockedHeap::new();

#[alloc_error_handler]
pub fn handle_alloc_error(layout:core::alloc::Layout)->!{
    panic!("Heap Allocation Error: {:?}", layout);
}

static mut HEAP_SPACE: [u8; KERNEL_HEAP_SIZE] = [0; KERNEL_HEAP_SIZE];

pub fn init_heap() {
    unsafe {
        HEAP_ALLOCATOR
            .lock()
            .init(&raw mut HEAP_SPACE as usize, KERNEL_HEAP_SIZE);
    }
}

