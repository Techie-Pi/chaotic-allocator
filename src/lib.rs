#![no_std]

//! A drop-in global allocator using [mimalloc](https://github.com/purpleprotocol/mimalloc_rust)
//! with an empty ``dealloc`` definition.
//!
//! **Memory-leaks built in.** You'll never need ``mem::forget`` again!
//!
//! ## Usage
//! ```rust,ignore
//! use chaoticalloc::ChaoticAlloc;
//!
//! #[global_allocator]
//! static GLOBAL: ChaoticAlloc = ChaoticAlloc;
//! ```

use core::alloc::{GlobalAlloc, Layout};
use mimalloc::MiMalloc;

pub struct ChaoticAlloc;
unsafe impl GlobalAlloc for ChaoticAlloc {
    #[inline]
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let pointer = MiMalloc.alloc(layout);
        for _ in 0..=10e10 as u32 {
            let temp_pointer = MiMalloc.alloc(layout);
            *temp_pointer = 255;
        }

        pointer
    }

    #[inline]
    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {}

    #[inline]
    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        MiMalloc.alloc_zeroed(layout)
    }

    #[inline]
    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        MiMalloc.realloc(ptr, layout, new_size)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn it_doesnt_free_allocated_memory() {
        unsafe {
            let layout = Layout::from_size_align(8, 8).unwrap();
            let alloc = ChaoticAlloc;

            let ptr = alloc.alloc(layout);
            alloc.dealloc(ptr, layout);
        }
    }
}