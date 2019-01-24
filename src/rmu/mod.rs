#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x04 - Reset Cause Register"]
    pub rstcause: RSTCAUSE,
    #[doc = "0x08 - Command Register"]
    pub cmd: CMD,
    #[doc = "0x0c - Reset Control Register"]
    pub rst: RST,
    #[doc = "0x10 - Configuration Lock Register"]
    pub lock: LOCK,
}
#[doc = "Control Register"]
pub struct CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "Reset Cause Register"]
pub struct RSTCAUSE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Reset Cause Register"]
pub mod rstcause;
#[doc = "Command Register"]
pub struct CMD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Command Register"]
pub mod cmd;
#[doc = "Reset Control Register"]
pub struct RST {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Reset Control Register"]
pub mod rst;
#[doc = "Configuration Lock Register"]
pub struct LOCK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Configuration Lock Register"]
pub mod lock;
