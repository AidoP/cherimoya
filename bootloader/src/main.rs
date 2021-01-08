#![no_std]
#![no_main]

#![feature(asm)]
#![feature(start)]
#![feature(abi_efiapi)]

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

extern crate kstate;

mod uefi;

#[no_mangle]
extern "efiapi" fn uefi_start<'a>(handle: uefi::ImageHandle, system_table: &'static mut uefi::SystemTable) -> ! {
    let uefi_memory_map = system_table.boot_services.get_memory_map().unwrap();
    //system_table.boot_services.exit_boot_services(handle, &uefi_memory_map);
    
    for descriptor in uefi_memory_map.iter() {
        
    }
}

#[allow(non_camel_case_types)]
pub struct void {
    _opaque: [u8; 0]
}