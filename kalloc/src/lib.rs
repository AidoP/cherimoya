#![no_std]

#![feature(asm)]

mod paging;
pub use paging::*;

#[repr(C)]
pub struct Allocator {
    free: &'static mut page::Table<page::Level4Entry>,
    last_free: VirtualAddress,
    /// The last page table allocated for the allocator
    last_page_table: VirtualAddress,
}
impl Allocator {
    /// Page table must be a valid level4 page table with a level 1 page table for address zero
    pub fn new(free: &'static mut page::Table::<page::Level4Entry>) -> Self {
        unsafe {
            // Ensure the free page table is free to start using
            assert!(!free[VirtualAddress::NULL].address().is_null());
            let free_lvl3 = &*free[VirtualAddress::NULL].address();
            assert!(!free_lvl3[VirtualAddress::NULL].address().is_null());
            let free_lvl2 = &*free_lvl3[VirtualAddress::NULL].address();
            assert!(!free_lvl2[VirtualAddress::NULL].address().is_null());
            let free_lvl1 = &*free_lvl2[VirtualAddress::NULL].address();
            assert!(!free_lvl1[VirtualAddress::NULL].address().is_null());
        }

        Allocator {
            free,
            last_free: VirtualAddress::NULL,
            last_page_table: VirtualAddress::NULL
        }
    }
    /// Allow the allocator to discover pages by walking through a series of memory segments.
    /// # Safety
    /// Undefined behaviour if the allocator discovers the same page more than once.
    /// Once discovered pages shall not be used until allocated.
    /// The initial pages given in `Allocator::new(pages)` must be marked as Allocator.
    /// 
    /// It is not safe to use pages marked as free as they may be allocated by the allocator itself. Instead call `Allocator::reclaim`.
    pub unsafe fn discover_pages(&mut self, memory_segments: impl Iterator<Item=MemorySegment>) {
        for segment in memory_segments {
            if let MemorySegment {
                usage: MemoryUsage::Free,
                page,
                count,
                ..
            } = segment{
                for i in 0..count {
                    let page = page.add(i * core::mem::size_of::<Page>());
                    self.reclaim(page)
                }
            }
        }
    }
    /// # Safety
    /// Page is a valid page physical address.
    /// This page cannot be used after this point
    pub unsafe fn reclaim(&mut self, page: *mut Page) {
        if self.last_free.page_table() >= self.last_page_table.page_table() {
            // Need more memory for the free page table itself, use this page
        } else {
            // Add to the free page table
            self.last_free.increment_page();
            self.free.page_entry(self.last_free).unwrap().set_address(page.into());
        }
    }
}

pub struct MemorySegment {
    pub page: *mut Page,
    pub count: usize,
    pub usage: MemoryUsage,
    pub properties: MemoryProperties
} 

use core::ops::{Deref, DerefMut};

#[repr(u8)]
pub enum MemoryUsage {
    Reserved,
    Allocator,
    Free,
    Unusable,
    Mmio
}

#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct MemoryProperties(u32);
impl MemoryProperties {
    pub const READ: u32 = 1 << 0;
    pub const WRITE: u32 = 1 << 1;
    pub const EXECUTE: u32 = 1 << 2;

    pub fn all(self, bits: Self) -> bool {
        *self & *bits == *bits
    }
    pub fn none(self, bits: Self) -> bool {
        *self & *bits == 0
    }
    pub fn any(self, bits: Self) -> bool {
        *self & *bits > 0
    }
}
impl Deref for MemoryProperties {
    type Target = u32;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for MemoryProperties {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}