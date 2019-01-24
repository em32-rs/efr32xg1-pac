#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x04 - Interrupt Duration"]
    pub periodsel: PERIODSEL,
    #[doc = "0x08 - Counter Value"]
    pub cnt: CNT,
    #[doc = "0x0c - Wake Up Enable"]
    pub em4wuen: EM4WUEN,
    #[doc = "0x10 - Interrupt Flag Register"]
    pub if_: IF,
    #[doc = "0x14 - Interrupt Flag Set Register"]
    pub ifs: IFS,
    #[doc = "0x18 - Interrupt Flag Clear Register"]
    pub ifc: IFC,
    #[doc = "0x1c - Interrupt Enable Register"]
    pub ien: IEN,
}
#[doc = "Control Register"]
pub struct CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "Interrupt Duration"]
pub struct PERIODSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Duration"]
pub mod periodsel;
#[doc = "Counter Value"]
pub struct CNT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Counter Value"]
pub mod cnt;
#[doc = "Wake Up Enable"]
pub struct EM4WUEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Wake Up Enable"]
pub mod em4wuen;
#[doc = "Interrupt Flag Register"]
pub struct IF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Flag Register"]
pub mod if_;
#[doc = "Interrupt Flag Set Register"]
pub struct IFS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Flag Set Register"]
pub mod ifs;
#[doc = "Interrupt Flag Clear Register"]
pub struct IFC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Flag Clear Register"]
pub mod ifc;
#[doc = "Interrupt Enable Register"]
pub struct IEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Enable Register"]
pub mod ien;
