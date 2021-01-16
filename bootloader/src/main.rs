#![no_std]
#![no_main]

#![feature(asm)]
#![feature(start)]
#![feature(abi_efiapi)]

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

mod uefi;

#[no_mangle]
extern "efiapi" fn uefi_start<'a>(handle: uefi::ImageHandle, system_table: &'static mut uefi::SystemTable) -> ! {
    let uefi_memory_map = system_table.boot_services.get_memory_map().unwrap();



    let msg = &[0xFF, 0xFF, 0xFF, 0xFF, core::mem::size_of::<Option<kalloc::Page>>() as u8 + b'0', 0,0,0, 0xFF, 0xFF, 0xFF, 0xFF];
    system_table.stdout.print_utf16(msg.as_ptr() as _);

    system_table.boot_services.exit_boot_services(handle, &uefi_memory_map);

    let map_count = uefi_memory_map.total_size / uefi_memory_map.descriptor_size;
    // 2 extra pages for the kernel 
    const EXTRA_PAGES: usize = 2 + 1;
    let required_pages = (core::mem::size_of::<kalloc::Page>() * (map_count + EXTRA_PAGES)) / 0x1000;
    uefi_memory_map.iter().find(|d| d.pages as usize >= required_pages );
    for descriptor in uefi_memory_map.iter() {
        
    }
    loop {}
}

#[allow(non_camel_case_types)]
pub struct void {
    _opaque: [u8; 0]
}