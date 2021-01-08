use core::ops::{Deref, DerefMut, BitOr, BitXor, BitAnd, Not, Index};
use crate::uefi;

#[repr(transparent)]
pub struct AllocateType(u32);
impl AllocateType {
    pub const ANY_PAGES: Self = Self(0);
    pub const MAX_ADDRESS: Self = Self(1);
    pub const ADDRESS: Self = Self(2);
}
#[repr(transparent)]
pub struct MemoryType(u32);
impl MemoryType {
    pub const RESERVED: Self = Self(0);
    pub const LOADER_CODE: Self = Self(1);
    pub const LOADER_DATA: Self = Self(2);
    pub const BOOT_SERVICES_CODE: Self = Self(3);
    pub const BOOT_SERVICES_DATA: Self = Self(4);
    pub const RUNTIME_SERVICES_CODE: Self = Self(5);
    pub const RUNTIME_SERVICES_DATA: Self = Self(6);
    pub const CONVENTIONAL: Self = Self(7);
    pub const UNUSABLE: Self = Self(8);
    pub const ACPI_RECLAIM: Self = Self(9);
    pub const ACPI: Self = Self(9);
    pub const MMIO: Self = Self(10);
    pub const MMIO_PORT: Self = Self(11);
    pub const PAL: Self = Self(12);
    pub const PERSISTENT: Self = Self(13);
    pub const MEMORY_MAP: Self = Self(-1i32 as u32);
}
#[repr(transparent)]
pub struct MemoryAttributes(u64);
impl MemoryAttributes {
    pub const NO_CACHE: Self = Self(0x1);
    pub const WRITE_COMBINING: Self = Self(0x2);
    pub const WRITE_THROUGH: Self = Self(0x4);
    pub const WRITE_BACK: Self = Self(0x8);
    pub const NO_CAHCE_EXPORTED: Self = Self(0x10);
    pub const WRITE_PROTECTED: Self = Self(0x1000);
    pub const READ_PROTECTED: Self = Self(0x2000);
    pub const EXECUTE_PROTECTED: Self = Self(0x4000);
    pub const NON_VOLATILE: Self = Self(0x8000);
    pub const RELIABLE: Self = Self(0x10000);
    pub const READ_ONLY: Self = Self(0x20000);
    pub const SPECIFIC_PURPOSE: Self = Self(0x40000);
    pub const CRYPTO_PROTECTED: Self = Self(0x80000);
    pub const RUNTIME: Self = Self(0x8000000000000000);
}
impl BitOr for MemoryAttributes {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(*self | *other)
    }
}
impl BitXor for MemoryAttributes {
    type Output = Self;
    fn bitxor(self, other: Self) -> Self {
        Self(*self ^ *other)
    }
}
impl BitAnd for MemoryAttributes {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(*self & *other)
    }
}
impl Not for MemoryAttributes {
    type Output = Self;
    fn not(self) -> Self {
        Self(!*self)
    }
}
impl Deref for MemoryAttributes {
    type Target = u64;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for MemoryAttributes {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub struct MemoryMap {
    pub(in crate::uefi) descriptors: *mut MemoryDescriptor,
    pub(in crate::uefi) total_size: usize,
    pub(in crate::uefi) descriptor_size: usize,
    pub(in crate::uefi) version: u32,
    pub(in crate::uefi) key: usize,
    pub(in crate::uefi) boot_services: &'static uefi::BootServices
}
impl MemoryMap {
    pub fn iter(&self) -> MemoryMapIter<'_> {
        MemoryMapIter(0, self)
    }
}
impl Drop for MemoryMap {
    fn drop(&mut self) {
        self.boot_services.free_pool(self.descriptors);
    }
}
pub struct MemoryMapIter<'a>(usize, &'a MemoryMap);
impl<'a> Iterator for MemoryMapIter<'a> {
    type Item = &'a MemoryDescriptor;
    fn next(&mut self) -> Option<Self::Item> {
        self.0 += 1;
        if self.1.descriptor_size * self.0 >= self.1.total_size {
            None
        } else {
            unsafe { ((self.1.descriptors as usize + self.1.descriptor_size * self.0) as *mut MemoryDescriptor).as_ref() }
        }
    }
}

#[repr(C)]
pub struct MemoryDescriptor {
    memory_type: MemoryType,
    physcial_start: u64,
    virtual_start: u64,
    pages: u64,
    attributes: MemoryAttributes
}