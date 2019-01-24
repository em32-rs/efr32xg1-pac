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
    #[doc = "0x10 - Top Value Register"]
    pub top: TOP,
    #[doc = "0x14 - Top Value Buffer Register"]
    pub topb: TOPB,
    #[doc = "0x18 - Interrupt Flag Register"]
    pub if_: IF,
    #[doc = "0x1c - Interrupt Flag Set Register"]
    pub ifs: IFS,
    #[doc = "0x20 - Interrupt Flag Clear Register"]
    pub ifc: IFC,
    #[doc = "0x24 - Interrupt Enable Register"]
    pub ien: IEN,
    _reserved0: [u8; 4usize],
    #[doc = "0x2c - I/O Routing Location Register"]
    pub routeloc0: ROUTELOC0,
    _reserved1: [u8; 16usize],
    #[doc = "0x40 - Freeze Register"]
    pub freeze: FREEZE,
    #[doc = "0x44 - Synchronization Busy Register"]
    pub syncbusy: SYNCBUSY,
    _reserved2: [u8; 28usize],
    #[doc = "0x64 - Auxiliary Counter Value Register"]
    pub auxcnt: AUXCNT,
    #[doc = "0x68 - PCNT Input Register"]
    pub input: INPUT,
    #[doc = "0x6c - Oversampling Config Register"]
    pub ovscfg: OVSCFG,
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
#[doc = "Top Value Register"]
pub struct TOP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Top Value Register"]
pub mod top;
#[doc = "Top Value Buffer Register"]
pub struct TOPB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Top Value Buffer Register"]
pub mod topb;
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
#[doc = "I/O Routing Location Register"]
pub struct ROUTELOC0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O Routing Location Register"]
pub mod routeloc0;
#[doc = "Freeze Register"]
pub struct FREEZE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Freeze Register"]
pub mod freeze;
#[doc = "Synchronization Busy Register"]
pub struct SYNCBUSY {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Synchronization Busy Register"]
pub mod syncbusy;
#[doc = "Auxiliary Counter Value Register"]
pub struct AUXCNT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Auxiliary Counter Value Register"]
pub mod auxcnt;
#[doc = "PCNT Input Register"]
pub struct INPUT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PCNT Input Register"]
pub mod input;
#[doc = "Oversampling Config Register"]
pub struct OVSCFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Oversampling Config Register"]
pub mod ovscfg;
