#![no_std]
#![no_main]

#![feature(asm)]
#![feature(start)]

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn uefi_start() -> ! {
    
    loop {}
}
