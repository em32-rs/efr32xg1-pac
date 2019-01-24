#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x04 - Current Programming Register"]
    pub curprog: CURPROG,
    _reserved0: [u8; 4usize],
    #[doc = "0x0c - Duty Cycle Configuration Register"]
    pub dutyconfig: DUTYCONFIG,
    _reserved1: [u8; 8usize],
    #[doc = "0x18 - Status Register"]
    pub status: STATUS,
    _reserved2: [u8; 4usize],
    #[doc = "0x20 - Interrupt Flag Register"]
    pub if_: IF,
    #[doc = "0x24 - Interrupt Flag Set Register"]
    pub ifs: IFS,
    #[doc = "0x28 - Interrupt Flag Clear Register"]
    pub ifc: IFC,
    #[doc = "0x2c - Interrupt Enable Register"]
    pub ien: IEN,
    _reserved3: [u8; 4usize],
    #[doc = "0x34 - APORT Request Status Register"]
    pub aportreq: APORTREQ,
    #[doc = "0x38 - APORT Request Status Register"]
    pub aportconflict: APORTCONFLICT,
}
#[doc = "Control Register"]
pub struct CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "Current Programming Register"]
pub struct CURPROG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Current Programming Register"]
pub mod curprog;
#[doc = "Duty Cycle Configuration Register"]
pub struct DUTYCONFIG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Duty Cycle Configuration Register"]
pub mod dutyconfig;
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
#[doc = "APORT Request Status Register"]
pub struct APORTCONFLICT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "APORT Request Status Register"]
pub mod aportconflict;
