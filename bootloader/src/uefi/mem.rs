use super::{Event, Status};

#[repr(transparent)]
struct AllocateType(u32);
impl AllocateType {
    const ANY_PAGES: Self = Self(0);
    const MAX_ADDRESS: Self = Self(1);
    const ADDRESS: Self = Self(2);
}
#[repr(transparent)]
struct MemoryType(u32);
impl MemoryType {
    const RESERVED: Self = Self(0);
    const LOADER_CODE: Self = Self(1);
    const LOADER_DATA: Self = Self(2);
    const BOOT_SERVICES_CODE: Self = Self(3);
    const BOOT_SERVICES_DATA: Self = Self(4);
    const RUNTIME_SERVICES_CODE: Self = Self(5);
    const RUNTIME_SERVICES_DATA: Self = Self(6);
    const CONVENTIONAL: Self = Self(7);
    const UNUSABLE: Self = Self(8);
    const ACPI_RECLAIM: Self = Self(9);
    const ACPI: Self = Self(9);
    const MMIO: Self = Self(10);
    const MMIO_PORT: Self = Self(11);
    const PAL: Self = Self(12);
    const PERSISTENT: Self = Self(13);
}