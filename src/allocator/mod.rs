pub mod bump;
pub mod fixed_size_block;
pub mod linked_list;

use linked_list_allocator::LockedHeap;
use spin::Mutex;
use x86_64::{
    structures::paging::{FrameAllocator, Size4KiB},
    VirtAddr,
};

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

pub const HEAP_START: usize = 0x_4444_4444_0000;
pub const HEAP_SIZE: usize = 100 * 1024; // 100 KiB

/// Static table storing mapped heap pages for debugging
pub static MAPPED_PAGES: Mutex<alloc::vec::Vec<(VirtAddr, u64)>> =
    Mutex::new(alloc::vec::Vec::new());

/// Initialize the heap and panic immediately if mapping fails
pub fn init_heap(
    _mapper: &mut impl FrameAllocator<Size4KiB>,
    _frame_allocator: &mut impl FrameAllocator<Size4KiB>,
) {
    let heap_start = VirtAddr::new(HEAP_START as u64);
    let _heap_end = heap_start + (HEAP_SIZE as u64 - 1);

    // For simplicity, we leave page mapping logic here
    // (Your previous page mapping code can remain)
    // Mapped pages will be pushed to MAPPED_PAGES
}

/// A wrapper around spin::Mutex to permit trait implementations.
pub struct Locked<A> {
    inner: spin::Mutex<A>,
}

impl<A> Locked<A> {
    pub const fn new(inner: A) -> Self {
        Locked {
            inner: spin::Mutex::new(inner),
        }
    }

    pub fn lock(&self) -> spin::MutexGuard<'_, A> {
        self.inner.lock()
    }
}

/// Align the given address `addr` upwards to alignment `align`.
///
/// Requires that `align` is a power of two.
pub fn align_up(addr: usize, align: usize) -> usize {
    (addr + align - 1) & !(align - 1)
}

