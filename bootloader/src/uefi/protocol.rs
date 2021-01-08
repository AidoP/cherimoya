pub mod console;
pub mod device;

opaque! { Protocol }
opaque! { Interface }
opaque! { Agent }
opaque! { Controller }

#[repr(transparent)]
pub struct InterfaceType(u32);
impl InterfaceType {
    pub const NATIVE: Self = Self(0);
}
#[repr(transparent)]
pub struct SearchType(u32);
impl SearchType {
    pub const ALL: Self = Self(0);
    pub const BY_REGISTER_NOTIFY: Self = Self(1);
    pub const BY_PROTOCOL: Self = Self(2);
}
#[repr(transparent)]
pub struct Attributes(u32);
impl Attributes {
    pub const BY_PROTOCOL: Self = Self(0x1);
    pub const GET_PROTOCOL: Self = Self(0x2);
    pub const TEST_PROTOCOL: Self = Self(0x4);
    pub const BY_CHILD_CONTROLLER: Self = Self(0x8);
    pub const BY_DRIVER: Self = Self(0x10);
    pub const EXCLUSIVE: Self = Self(0x20);
}

#[repr(C)]
pub struct Information {
    agent: Agent,
    controller: Controller,
    attributes: Attributes,
    open_count: u32
}