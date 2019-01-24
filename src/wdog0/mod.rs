#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x04 - Command Register"]
    pub cmd: CMD,
    #[doc = "0x08 - Synchronization Busy Register"]
    pub syncbusy: SYNCBUSY,
    #[doc = "0x0c - PRS Control Register"]
    pub pch0_prsctrl: PCH0_PRSCTRL,
    #[doc = "0x10 - PRS Control Register"]
    pub pch1_prsctrl: PCH1_PRSCTRL,
    _reserved0: [u8; 8usize],
    #[doc = "0x1c - Watchdog Interrupt Flags"]
    pub if_: IF,
    #[doc = "0x20 - Interrupt Flag Set Register"]
    pub ifs: IFS,
    #[doc = "0x24 - Interrupt Flag Clear Register"]
    pub ifc: IFC,
    #[doc = "0x28 - Interrupt Enable Register"]
    pub ien: IEN,
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
#[doc = "Synchronization Busy Register"]
pub struct SYNCBUSY {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Synchronization Busy Register"]
pub mod syncbusy;
#[doc = "PRS Control Register"]
pub struct PCH0_PRSCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PRS Control Register"]
pub mod pch0_prsctrl;
#[doc = "PRS Control Register"]
pub struct PCH1_PRSCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PRS Control Register"]
pub mod pch1_prsctrl;
#[doc = "Watchdog Interrupt Flags"]
pub struct IF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Watchdog Interrupt Flags"]
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
