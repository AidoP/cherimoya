use crate::void;

macro_rules! opaque {
    ($name:ident) => {
        #[repr(C)]
        pub struct $name(*mut [u8; 0]);
    };
}

mod protocol;
mod mem;
mod event;

#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(transparent)]
pub struct Status(pub usize);
impl Status {
    pub const SUCCESS: Self = Self(0);
    pub const INVALID_PARAMETER: Self = Self(2 + (isize::MIN as usize));
    pub const OUT_OF_RESOURCES: Self = Self(9 + isize::MIN as usize);
}

opaque! { ImageHandle }
opaque! { RuntimeServices }
opaque! { ConfigurationTable }

#[repr(C, align(64))]
pub struct Guid(u32, u16, u16, [u8; 8]);

#[repr(C)]
pub struct TableHeader {
    signature: u64,
    version: u32,
    table_size: u32,
    crc: u32,
    _reserved: u32
}

#[repr(C)]
pub struct SystemTable {
    header: TableHeader,
    firmware_vendor: *const u16,
    firmware_version: u32,
    stdin_protocol: protocol::Protocol,
    pub stdin: &'static mut protocol::console::Input,
    stdout_protocol: protocol::Protocol,
    pub stdout: &'static mut protocol::console::Output,
    stderr_protocol: protocol::Protocol,
    pub stderr: &'static mut protocol::console::Output,
    runtime_services: &'static RuntimeServices,
    pub boot_services: &'static BootServices,
    table_entries: usize,
    configuration: ConfigurationTable
}

#[repr(C)]
pub struct BootServices {
    header: TableHeader,

    raise_tpl: extern "efiapi" fn(tpl: event::Priority) -> usize,
    restore_tpl: extern "efiapi" fn(old_tpl: event::Priority),

    allocate_pages: extern "efiapi" fn(mem::AllocateType, mem::MemoryType, pages: usize, memory: &mut u64) -> Status,
    free_pages: extern "efiapi" fn(memory: u64, pages: usize) -> Status,
    get_memory_map: extern "efiapi" fn(total_size: &mut usize, memory_map: *mut mem::MemoryDescriptor, map_key: &mut usize, descriptor_size: &mut usize, descriptor_version: &mut u32) -> Status,
    allocate_pool: extern "efiapi" fn(pool_type: mem::MemoryType, size: usize, buffer: &mut *mut void) -> Status,
    free_pool: extern "efiapi" fn(buffer: *mut void) -> Status,

    create_event: extern "efiapi" fn(event_type: event::Type, notify_priority: event::Priority, notify_fn: event::NotifyFn, context: *mut void, event: &mut event::Event) -> Status,
    set_timer: extern "efiapi" fn(event::Event, event::TimerType, time: u64) -> Status,
    wait_for_event: extern "efiapi" fn(count: usize, events: event::Event, waited: &mut usize) -> Status,
    signal_event: extern "efiapi" fn(event::Event) -> Status,
    close_event: extern "efiapi" fn(event::Event) -> Status,
    check_event: extern "efiapi" fn(event::Event) -> Status,

    install_protocol:  extern "efiapi" fn(&mut protocol::Protocol, protocol: &Guid, protocol::InterfaceType, protocol::Interface) -> Status,
    reinstall_protocol: extern "efiapi" fn(protocol::Protocol, protocol: &Guid, old: protocol::Interface, new: protocol::Interface) -> Status,
    uninstall_protocol: extern "efiapi" fn(protocol::Protocol, protocol: &Guid, protocol::Interface) -> Status,
    handle_protocol: extern "efiapi" fn(protocol::Protocol, protocol: &Guid, &mut protocol::Interface) -> Status,
    _reserved: *const void,
    register_protocol_notify: extern "efiapi" fn(protocol: &Guid, event::Event, registration: *mut *const void) -> Status,
    locate_handle: extern "efiapi" fn(protocol::SearchType, protocol: Option<&Guid>, key: *const void, buffer_size: &mut usize, buffer: &mut protocol::Protocol) -> Status,
    locate_device_path: extern "efiapi" fn(protocol: &Guid, &mut &protocol::device::Path, device: &mut protocol::device::Device) -> Status,
    install_configuration_table: extern "efiapi" fn() -> Status,
    
    load_image: extern "efiapi" fn() -> Status,
    start_image: extern "efiapi" fn() -> Status,
    exit: extern "efiapi" fn() -> Status,
    unload_image: extern "efiapi" fn() -> Status,
    exit_boot_services: extern "efiapi" fn(ImageHandle, usize) -> Status,

    next_monotonic_count: extern "efiapi" fn() -> Status,
    stall: extern "efiapi" fn() -> Status,
    set_watchdog: extern "efiapi" fn() -> Status,

    connect_controller: extern "efiapi" fn(protocol::Controller, drivers: *const ImageHandle, Option<&protocol::device::RemainingPath>, recursive: u8) -> Status,
    disconnect_controller: extern "efiapi" fn(protocol::Controller, driver: ImageHandle, child: ImageHandle) -> Status,

    open_protocol: extern "efiapi" fn(protocol::Protocol, protocol: &Guid, Option<&mut protocol::Interface>, protocol::Agent, protocol::Controller, attributes: protocol::Attributes) -> Status,
    close_protocol: extern "efiapi" fn(protocol::Protocol, protocol: &Guid, protocol::Agent, protocol::Controller) -> Status,
    open_protocol_info: extern "efiapi" fn(protocol::Protocol, protocol: &Guid, entries: &mut *const protocol::Information, entry_count: &mut usize) -> Status,

    protocols_per_handle: extern "efiapi" fn() -> Status,
    locate_handle_buffer: extern "efiapi" fn() -> Status,
    locate_protocol: extern "efiapi" fn() -> Status,
    install_multiple_protocols: extern "efiapi" fn() -> Status,
    uninstall_multiple_protocols: extern "efiapi" fn() -> Status,

    calculate_crc32: extern "efiapi" fn() -> Status,

    memcpy: extern "efiapi" fn() -> Status,
    memset: extern "efiapi" fn() -> Status,
    create_event_ex: extern "efiapi" fn(event_type: event::Type, notify_priority: event::Priority, notify_fn: event::NotifyFn, context: *const void, group: &Guid, event: &mut event::Event) -> Status
}
impl BootServices {
    pub fn get_memory_map(&'static self) -> Option<mem::MemoryMap> {
        let mut total_size = 0;
        let mut descriptors = 0 as *mut _;
        let mut descriptor_size = 0;
        let mut version = 0;
        let mut key = 0;
        (self.get_memory_map)(&mut total_size, descriptors, &mut key, &mut descriptor_size, &mut version);

        total_size += 2 * descriptor_size;

        descriptors = self.allocate_pool_untyped(total_size, mem::MemoryType::MEMORY_MAP)? as _;

        if (self.get_memory_map)(&mut total_size, descriptors, &mut key, &mut descriptor_size, &mut version) == Status::SUCCESS {
            Some(mem::MemoryMap {
                total_size,
                descriptors,
                descriptor_size,
                version,
                key,
                boot_services: self
            })
        } else {
            self.free_pool(descriptors);
            None
        }
    }
    pub fn allocate_pool<T>(&self, count: usize, memory_type: mem::MemoryType) -> Option<*mut T> {
        let mut buffer = 0 as *mut T;
        if (self.allocate_pool)(memory_type, core::mem::size_of::<T>() * count, &mut (buffer as _)) == Status::SUCCESS {
            Some(buffer)
        } else {
            None
        }
    }
    pub fn allocate_pool_untyped(&self, bytes: usize, memory_type: mem::MemoryType) -> Option<*mut void> {
        let mut buffer = 0 as *mut void;
        if (self.allocate_pool)(memory_type, bytes, &mut buffer) == Status::SUCCESS {
            Some(buffer)
        } else {
            None
        }
    }
    pub fn free_pool<T>(&self, pool: *mut T) -> Status {
        (self.free_pool)(pool as _)
    }
    pub fn exit_boot_services(&self, program: ImageHandle, memory_map: &mem::MemoryMap) -> Status {
        (self.exit_boot_services)(program, memory_map.key)
    }
}