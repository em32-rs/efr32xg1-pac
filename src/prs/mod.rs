#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Software Pulse Register"]
    pub swpulse: SWPULSE,
    #[doc = "0x04 - Software Level Register"]
    pub swlevel: SWLEVEL,
    #[doc = "0x08 - I/O Routing Pin Enable Register"]
    pub routepen: ROUTEPEN,
    _reserved0: [u8; 4usize],
    #[doc = "0x10 - I/O Routing Location Register"]
    pub routeloc0: ROUTELOC0,
    #[doc = "0x14 - I/O Routing Location Register"]
    pub routeloc1: ROUTELOC1,
    #[doc = "0x18 - I/O Routing Location Register"]
    pub routeloc2: ROUTELOC2,
    _reserved1: [u8; 4usize],
    #[doc = "0x20 - Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x24 - DMA Request 0 Register"]
    pub dmareq0: DMAREQ0,
    #[doc = "0x28 - DMA Request 1 Register"]
    pub dmareq1: DMAREQ1,
    _reserved2: [u8; 4usize],
    #[doc = "0x30 - PRS Channel Values"]
    pub peek: PEEK,
    _reserved3: [u8; 12usize],
    #[doc = "0x40 - Channel Control Register"]
    pub ch0_ctrl: CH0_CTRL,
    #[doc = "0x44 - Channel Control Register"]
    pub ch1_ctrl: CH1_CTRL,
    #[doc = "0x48 - Channel Control Register"]
    pub ch2_ctrl: CH2_CTRL,
    #[doc = "0x4c - Channel Control Register"]
    pub ch3_ctrl: CH3_CTRL,
    #[doc = "0x50 - Channel Control Register"]
    pub ch4_ctrl: CH4_CTRL,
    #[doc = "0x54 - Channel Control Register"]
    pub ch5_ctrl: CH5_CTRL,
    #[doc = "0x58 - Channel Control Register"]
    pub ch6_ctrl: CH6_CTRL,
    #[doc = "0x5c - Channel Control Register"]
    pub ch7_ctrl: CH7_CTRL,
    #[doc = "0x60 - Channel Control Register"]
    pub ch8_ctrl: CH8_CTRL,
    #[doc = "0x64 - Channel Control Register"]
    pub ch9_ctrl: CH9_CTRL,
    #[doc = "0x68 - Channel Control Register"]
    pub ch10_ctrl: CH10_CTRL,
    #[doc = "0x6c - Channel Control Register"]
    pub ch11_ctrl: CH11_CTRL,
}
#[doc = "Software Pulse Register"]
pub struct SWPULSE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Software Pulse Register"]
pub mod swpulse;
#[doc = "Software Level Register"]
pub struct SWLEVEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Software Level Register"]
pub mod swlevel;
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
#[doc = "I/O Routing Location Register"]
pub struct ROUTELOC1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O Routing Location Register"]
pub mod routeloc1;
#[doc = "I/O Routing Location Register"]
pub struct ROUTELOC2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O Routing Location Register"]
pub mod routeloc2;
#[doc = "Control Register"]
pub struct CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "DMA Request 0 Register"]
pub struct DMAREQ0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Request 0 Register"]
pub mod dmareq0;
#[doc = "DMA Request 1 Register"]
pub struct DMAREQ1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Request 1 Register"]
pub mod dmareq1;
#[doc = "PRS Channel Values"]
pub struct PEEK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PRS Channel Values"]
pub mod peek;
#[doc = "Channel Control Register"]
pub struct CH0_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Control Register"]
pub mod ch0_ctrl;
#[doc = "Channel Control Register"]
pub struct CH1_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Control Register"]
pub mod ch1_ctrl;
#[doc = "Channel Control Register"]
pub struct CH2_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Control Register"]
pub mod ch2_ctrl;
#[doc = "Channel Control Register"]
pub struct CH3_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Control Register"]
pub mod ch3_ctrl;
#[doc = "Channel Control Register"]
pub struct CH4_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Control Register"]
pub mod ch4_ctrl;
#[doc = "Channel Control Register"]
pub struct CH5_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Control Register"]
pub mod ch5_ctrl;
#[doc = "Channel Control Register"]
pub struct CH6_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Control Register"]
pub mod ch6_ctrl;
#[doc = "Channel Control Register"]
pub struct CH7_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Control Register"]
pub mod ch7_ctrl;
#[doc = "Channel Control Register"]
pub struct CH8_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Control Register"]
pub mod ch8_ctrl;
#[doc = "Channel Control Register"]
pub struct CH9_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Control Register"]
pub mod ch9_ctrl;
#[doc = "Channel Control Register"]
pub struct CH10_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Control Register"]
pub mod ch10_ctrl;
#[doc = "Channel Control Register"]
pub struct CH11_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Control Register"]
pub mod ch11_ctrl;
