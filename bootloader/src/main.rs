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

const MSG: *const u16 = [b'H', 0, b'e', 0, b'l', 0, b'l', 0, b'o', 0, b',', 0, b' ', 0, b'W', 0, b'o', 0, b'r', 0, b'l', 0, b'd', 0, b'!', 0, b'\n', 0, 0, 0].as_ptr() as _;

#[no_mangle]
extern "efiapi" fn uefi_start<'a>(handle: uefi::ImageHandle, system_table: &'static mut uefi::SystemTable) -> uefi::Status {
    system_table.stdout.print_utf16(MSG);
    uefi::Status::SUCCESS
}