#![no_std]

use core::ops::{Deref, DerefMut};

#[repr(u8)]
pub enum MemoryUsage {
    Reserved,
    MemoryMap,
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

#[repr(C)]
pub struct Page {
    pub page: usize,
    pub pages: usize,
    pub usage: MemoryUsage,
    pub properties: MemoryProperties
}

#[repr(C)]
pub struct MemoryMap {
    pages: *mut Page
}