#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x04 - Input Selection Register"]
    pub inputsel: INPUTSEL,
    #[doc = "0x08 - Status Register"]
    pub status: STATUS,
    #[doc = "0x0c - Interrupt Flag Register"]
    pub if_: IF,
    #[doc = "0x10 - Interrupt Flag Set Register"]
    pub ifs: IFS,
    #[doc = "0x14 - Interrupt Flag Clear Register"]
    pub ifc: IFC,
    #[doc = "0x18 - Interrupt Enable Register"]
    pub ien: IEN,
    _reserved0: [u8; 4usize],
    #[doc = "0x20 - APORT Request Status Register"]
    pub aportreq: APORTREQ,
    #[doc = "0x24 - APORT Conflict Status Register"]
    pub aportconflict: APORTCONFLICT,
    #[doc = "0x28 - Hysteresis 0 Register"]
    pub hysteresis0: HYSTERESIS0,
    #[doc = "0x2c - Hysteresis 1 Register"]
    pub hysteresis1: HYSTERESIS1,
    _reserved1: [u8; 16usize],
    #[doc = "0x40 - I/O Routing Pine Enable Register"]
    pub routepen: ROUTEPEN,
    #[doc = "0x44 - I/O Routing Location Register"]
    pub routeloc0: ROUTELOC0,
}
#[doc = "Control Register"]
pub struct CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "Input Selection Register"]
pub struct INPUTSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Input Selection Register"]
pub mod inputsel;
#[doc = "Status Register"]
pub struct STATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status Register"]
pub mod status;
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
#[doc = "APORT Request Status Register"]
pub struct APORTREQ {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "APORT Request Status Register"]
pub mod aportreq;
#[doc = "APORT Conflict Status Register"]
pub struct APORTCONFLICT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "APORT Conflict Status Register"]
pub mod aportconflict;
#[doc = "Hysteresis 0 Register"]
pub struct HYSTERESIS0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Hysteresis 0 Register"]
pub mod hysteresis0;
#[doc = "Hysteresis 1 Register"]
pub struct HYSTERESIS1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Hysteresis 1 Register"]
pub mod hysteresis1;
#[doc = "I/O Routing Pine Enable Register"]
pub struct ROUTEPEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O Routing Pine Enable Register"]
pub mod routepen;
#[doc = "I/O Routing Location Register"]
pub struct ROUTELOC0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O Routing Location Register"]
pub mod routeloc0;
