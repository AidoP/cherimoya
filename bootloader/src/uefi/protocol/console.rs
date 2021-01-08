use crate::uefi::{event, Status};

opaque! { Mode }

#[repr(C)]
pub struct Key {
    code: u16,
    utf16_char: u16
}

#[repr(C)]
pub struct Input {
    pub reset: extern "efiapi" fn(&mut Self, u8) -> Status,
    pub read_key: extern "efiapi" fn(&mut Self, key: *mut Key) -> Status,
    pub wait_for_key: event::Event
}
impl Input {
    #[inline]
    pub fn reset(&mut self, verified: bool) -> Status {
        (self.reset)(self, verified as _)
    }
}
#[repr(C)]
pub struct Output {
    reset: extern "efiapi" fn(&mut Self, u8) -> Status,
    print: extern "efiapi" fn(&mut Self, utf16_string: *const u16) -> Status,
    test_string: extern "efiapi" fn(&mut Self, utf16_string: *const u16) -> Status,
    query_mode: extern "efiapi" fn(&mut Self, mode: usize, columns: *mut usize, rows: *mut usize) -> Status,
    set_mode: extern "efiapi" fn(&mut Self, mode: usize) -> Status,
    set_attribute: extern "efiapi" fn(&mut Self, attribute: usize) -> Status,
    clear_screen: extern "efiapi" fn(&mut Self) -> Status,
    set_cursor_position: extern "efiapi" fn(&mut Self, x: usize, y: usize) -> Status,
    enable_cursor: extern "efiapi" fn(&mut Self, enabled: u8) -> Status,
    mode: *mut Mode
}
impl Output {
    #[inline]
    pub fn reset(&mut self, verified: bool) -> Status {
        (self.reset)(self, verified as _)
    }
    #[inline]
    pub fn print_utf16(&mut self, utf16_string: *const u16) -> Status {
        (self.print)(self, utf16_string)
    }
}