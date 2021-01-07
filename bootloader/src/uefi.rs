macro_rules! opaque {
    ($name:ident) => {
        #[repr(C)]
        pub struct $name(*mut [u8; 0]);
    };
}

mod console;
mod mem;

#[repr(transparent)]
pub struct Status(pub usize);
impl Status {
    pub const SUCCESS: Self = Self(0);
}

opaque! { Event }
opaque! { Protocol }
opaque! { ImageHandle }
opaque! { RuntimeServices }
opaque! { BootServices }
opaque! { ConfigurationTable }

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
    stdin_protocol: Protocol,
    pub stdin: &'static mut console::Input,
    stdout_protocol: Protocol,
    pub stdout: &'static mut console::Output,
    stderr_protocol: Protocol,
    pub stderr: &'static mut console::Output,
    runtime_services: &'static RuntimeServices,
    boot_services: &'static BootServices,
    table_entries: usize,
    configuration: ConfigurationTable
}

#[repr(C)]
pub struct BootServices {
    header: TableHeader,

    raise_tpl: extern "efiapi" fn(tpl: usize) -> usize,
    restore_tpl: extern "efiapi" fn(old_tpl: usize),

    allocate_pages: extern "efiapi" fn(mem::AllocateType, mem::MemoryType, pages: usize, memory: &mut u64) -> Status,
    free_pages: extern "efiapi" fn(memory: u64, pages: usize) -> Status,
    get_memory_map: ,
    allocate_pool: ,
    free_pool: ,

    create_event: ,
    set_timer: ,
    wait_for_event: ,
    signal_event: ,
    close_event: ,
    check_event: ,

    install_protocol: ,
    reinstall_protocol: ,
    uninstall_protocol: ,
    handle_protocol: ,
    _reserved: ,
    register_protocol_notify: ,
    locate_handle: ,
    locate_device_path: ,
    install_configuration_table: ,
    
    load_image: ,
    start_image: ,
    exit: ,
    unload_image: ,
    exit_boot_services: 
}