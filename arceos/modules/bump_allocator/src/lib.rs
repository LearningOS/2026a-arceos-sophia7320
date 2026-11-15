#![no_std]

use core::{ptr::NonNull, todo};

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

pub struct EarlyAllocator<const SIZE: usize> {
    total_size: usize,
    used_size: usize,
    inner: BumpAllocator<SIZE>,
}

struct BumpAllocator<const SIZE: usize> {
    start: usize,
    b_pos: usize,
    p_pos: usize,
    end: usize,
}

impl<const SIZE: usize> BumpAllocator<SIZE> {
    pub fn balloc(&mut self, size: usize, align: usize) -> usize {
        let addr = align_up(self.b_pos, align);
        let b_pos_new = addr + size;

        if b_pos_new > self.p_pos || b_pos_new > self.end {
            return 0;
        }

        self.b_pos = b_pos_new;

        addr
    }

    pub fn palloc(&mut self, num_pages: usize, align_pow2: usize) -> usize {
        if align_pow2 % num_pages != 0 {
            return 0;
        }

        let addr = self.p_pos - SIZE * num_pages;

        if addr < self.b_pos || addr < self.start {
            return 0;
        }

        self.p_pos = addr;
        addr
    }
}

impl<const SIZE: usize> EarlyAllocator<SIZE> {
    pub const fn new() -> Self {
        Self {
            total_size: 0,
            used_size: 0,
            inner: BumpAllocator {
                start: 0,
                b_pos: 0,
                p_pos: 0,
                end: 0,
            },
        }
    }
}

impl<const SIZE: usize> BaseAllocator for EarlyAllocator<SIZE> {
    fn init(&mut self, start: usize, size: usize) {
        self.inner = BumpAllocator {
            start,
            b_pos: start,
            p_pos: start + size,
            end: start + size,
        }
    }

    fn add_memory(&mut self, _start: usize, _size: usize) -> allocator::AllocResult {
        // ignore this
        Ok(())
    }
}

impl<const SIZE: usize> ByteAllocator for EarlyAllocator<SIZE> {
    fn alloc(
        &mut self,
        layout: core::alloc::Layout,
    ) -> allocator::AllocResult<core::ptr::NonNull<u8>> {
        let (size, align) = (layout.size(), layout.align());

        let addr = self.inner.balloc(size, align);

        if addr != 0 {
            Ok(unsafe { NonNull::new_unchecked(addr as *mut u8) })
        } else {
            Err(allocator::AllocError::NoMemory)
        }
    }

    fn dealloc(&mut self, _pos: core::ptr::NonNull<u8>, _layout: core::alloc::Layout) {
        // 不实现 ， 在reset的时候一次性复位
    }

    fn total_bytes(&self) -> usize {
        self.total_size
    }

    fn used_bytes(&self) -> usize {
        //
        0
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
        if num_pages == 0 {
            return Err(allocator::AllocError::InvalidParam);
        }

        let addr = self.inner.palloc(num_pages, align_pow2);

        if addr == 0 {
            Err(allocator::AllocError::NoMemory)
        } else {
            Ok(addr)
        }
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

fn align_up(addr: usize, align: usize) -> usize {
    (addr + align - 1) & !(align - 1)
}
