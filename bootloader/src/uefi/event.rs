use crate::void;

pub type NotifyFn = Option<extern "efiapi" fn(Event, context: *mut void)>;
opaque! { Event }

#[repr(transparent)]
pub struct Priority(usize);

#[repr(transparent)]
pub struct Type(u32);
impl Type {
    pub const TIMER: Self = Self(0x80000000);
    pub const RUNTIME: Self = Self(0x40000000);
    pub const NOTIFY_WAIT: Self = Self(0x100);
    pub const NOTIFY_SIGNAL: Self = Self(0x200);
    pub const EXIT_BOOT_SERVICES: Self = Self(0x201);
    pub const VIRTUAL_ADDRESS_CHANGE: Self = Self(0x60000202);
}

#[repr(transparent)]
pub struct TimerType(u32);
impl Type {
    pub const CANCEL: Self = Self(1);
    pub const PERIODIC: Self = Self(2);
    pub const RELATIVE: Self = Self(3);
}