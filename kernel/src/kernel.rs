#![no_std]
#![no_main]

#![feature(asm)]

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

/// Although we are already within Rust, kernel() must use a stable ABI as the uefi-stub is a seperate compilation unit
#[no_mangle]
pub extern "C" fn kernel(allocator: kalloc::Allocator) -> ! {
    
    loop { }
}