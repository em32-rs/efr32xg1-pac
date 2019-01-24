#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x04 - Command Register"]
    pub cmd: CMD,
    #[doc = "0x08 - Status Register"]
    pub status: STATUS,
    #[doc = "0x0c - Counter Value Register"]
    pub cnt: CNT,
    #[doc = "0x10 - Compare Value Register 0"]
    pub comp0: COMP0,
    #[doc = "0x14 - Compare Value Register 1"]
    pub comp1: COMP1,
    #[doc = "0x18 - Repeat Counter Register 0"]
    pub rep0: REP0,
    #[doc = "0x1c - Repeat Counter Register 1"]
    pub rep1: REP1,
    #[doc = "0x20 - Interrupt Flag Register"]
    pub if_: IF,
    #[doc = "0x24 - Interrupt Flag Set Register"]
    pub ifs: IFS,
    #[doc = "0x28 - Interrupt Flag Clear Register"]
    pub ifc: IFC,
    #[doc = "0x2c - Interrupt Enable Register"]
    pub ien: IEN,
    _reserved0: [u8; 4usize],
    #[doc = "0x34 - Synchronization Busy Register"]
    pub syncbusy: SYNCBUSY,
    _reserved1: [u8; 8usize],
    #[doc = "0x40 - I/O Routing Pin Enable Register"]
    pub routepen: ROUTEPEN,
    #[doc = "0x44 - I/O Routing Location Register"]
    pub routeloc0: ROUTELOC0,
    _reserved2: [u8; 8usize],
    #[doc = "0x50 - PRS Input Select Register"]
    pub prssel: PRSSEL,
}
#[doc = "Control Register"]
pub struct CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "Command Register"]
pub struct CMD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Command Register"]
pub mod cmd;
#[doc = "Status Register"]
pub struct STATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status Register"]
pub mod status;
#[doc = "Counter Value Register"]
pub struct CNT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Counter Value Register"]
pub mod cnt;
#[doc = "Compare Value Register 0"]
pub struct COMP0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Compare Value Register 0"]
pub mod comp0;
#[doc = "Compare Value Register 1"]
pub struct COMP1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Compare Value Register 1"]
pub mod comp1;
#[doc = "Repeat Counter Register 0"]
pub struct REP0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Repeat Counter Register 0"]
pub mod rep0;
#[doc = "Repeat Counter Register 1"]
pub struct REP1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Repeat Counter Register 1"]
pub mod rep1;
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
#[doc = "Synchronization Busy Register"]
pub struct SYNCBUSY {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Synchronization Busy Register"]
pub mod syncbusy;
#[doc = "I/O Routing Pin Enable Register"]
pub struct ROUTEPEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O Routing Pin Enable Register"]
pub mod routepen;
#[doc = "I/O Routing Location Register"]
pub struct ROUTELOC0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O Routing Location Register"]
pub mod routeloc0;
#[doc = "PRS Input Select Register"]
pub struct PRSSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PRS Input Select Register"]
pub mod prssel;
