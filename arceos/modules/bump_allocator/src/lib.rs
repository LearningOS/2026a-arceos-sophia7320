#![no_std]

use core::todo;

use allocator::{BaseAllocator, ByteAllocator, PageAllocator};

/// Early memory allocator
/// Use it before formal bytes-allocator and pages-allocator can work!
/// This is a double-end memory range:
/// - Alloc bytes forward
/// - Alloc pages backward
///
/// [ bytes-used | avail-area | pages-used ]
/// |            | -->    <-- |            |
/// start       b_pos        p_pos       end
///
/// For bytes area, 'count' records number of allocations.
/// When it goes down to ZERO, free bytes-used area.
/// For pages area, it will never be freed!
///

#[derive(Debug, Default)]
pub struct EarlyAllocator<const SIZE: usize> {
    start: usize,
    size: usize,
    end: usize,
    b_pos: usize,
    p_pos: usize,
}

struct BlockNode {
    size: usize,
    next: usize,
}

impl<const SIZE: usize> EarlyAllocator<SIZE> {
    pub const fn new() -> Self {
        Self {
            start: 0,
            size: SIZE,
            end: SIZE,
            b_pos: 0,
            p_pos: SIZE,
        }
    }
}

impl<const SIZE: usize> BaseAllocator for EarlyAllocator<SIZE> {
    fn init(&mut self, start: usize, size: usize) {
        self.start = start;
        self.size = size;
        self.end = start + size;
        self.b_pos = start;
        self.p_pos = self.end;
    }

    fn add_memory(&mut self, start: usize, size: usize) -> allocator::AllocResult {
        todo!()
    }
}

impl<const SIZE: usize> ByteAllocator for EarlyAllocator<SIZE> {
    fn alloc(
        &mut self,
        layout: core::alloc::Layout,
    ) -> allocator::AllocResult<core::ptr::NonNull<u8>> {
        todo!()
    }

    fn dealloc(&mut self, pos: core::ptr::NonNull<u8>, layout: core::alloc::Layout) {
        todo!()
    }

    fn total_bytes(&self) -> usize {
        self.size
    }

    fn used_bytes(&self) -> usize {
        todo!()
    }

    fn available_bytes(&self) -> usize {
        self.total_bytes() - self.used_bytes()
    }
}

impl<const SIZE: usize> PageAllocator for EarlyAllocator<SIZE> {
    const PAGE_SIZE: usize = SIZE;

    fn alloc_pages(
        &mut self,
        num_pages: usize,
        align_pow2: usize,
    ) -> allocator::AllocResult<usize> {
        todo!()
    }

    fn dealloc_pages(&mut self, pos: usize, num_pages: usize) {
        todo!()
    }

    fn total_pages(&self) -> usize {
        todo!()
    }

    fn used_pages(&self) -> usize {
        todo!()
    }

    fn available_pages(&self) -> usize {
        todo!()
    }
}
