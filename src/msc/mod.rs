#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Memory System Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x04 - Read Control Register"]
    pub readctrl: READCTRL,
    #[doc = "0x08 - Write Control Register"]
    pub writectrl: WRITECTRL,
    #[doc = "0x0c - Write Command Register"]
    pub writecmd: WRITECMD,
    #[doc = "0x10 - Page Erase/Write Address Buffer"]
    pub addrb: ADDRB,
    _reserved0: [u8; 4usize],
    #[doc = "0x18 - Write Data Register"]
    pub wdata: WDATA,
    #[doc = "0x1c - Status Register"]
    pub status: STATUS,
    _reserved1: [u8; 16usize],
    #[doc = "0x30 - Interrupt Flag Register"]
    pub if_: IF,
    #[doc = "0x34 - Interrupt Flag Set Register"]
    pub ifs: IFS,
    #[doc = "0x38 - Interrupt Flag Clear Register"]
    pub ifc: IFC,
    #[doc = "0x3c - Interrupt Enable Register"]
    pub ien: IEN,
    #[doc = "0x40 - Configuration Lock Register"]
    pub lock: LOCK,
    #[doc = "0x44 - Flash Cache Command Register"]
    pub cachecmd: CACHECMD,
    #[doc = "0x48 - Cache Hits Performance Counter"]
    pub cachehits: CACHEHITS,
    #[doc = "0x4c - Cache Misses Performance Counter"]
    pub cachemisses: CACHEMISSES,
    _reserved2: [u8; 4usize],
    #[doc = "0x54 - Mass Erase Lock Register"]
    pub masslock: MASSLOCK,
    _reserved3: [u8; 4usize],
    #[doc = "0x5c - Startup Control"]
    pub startup: STARTUP,
    _reserved4: [u8; 20usize],
    #[doc = "0x74 - Command Register"]
    pub cmd: CMD,
}
#[doc = "Memory System Control Register"]
pub struct CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Memory System Control Register"]
pub mod ctrl;
#[doc = "Read Control Register"]
pub struct READCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Read Control Register"]
pub mod readctrl;
#[doc = "Write Control Register"]
pub struct WRITECTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Write Control Register"]
pub mod writectrl;
#[doc = "Write Command Register"]
pub struct WRITECMD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Write Command Register"]
pub mod writecmd;
#[doc = "Page Erase/Write Address Buffer"]
pub struct ADDRB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Page Erase/Write Address Buffer"]
pub mod addrb;
#[doc = "Write Data Register"]
pub struct WDATA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Write Data Register"]
pub mod wdata;
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
#[doc = "Configuration Lock Register"]
pub struct LOCK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Configuration Lock Register"]
pub mod lock;
#[doc = "Flash Cache Command Register"]
pub struct CACHECMD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flash Cache Command Register"]
pub mod cachecmd;
#[doc = "Cache Hits Performance Counter"]
pub struct CACHEHITS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Cache Hits Performance Counter"]
pub mod cachehits;
#[doc = "Cache Misses Performance Counter"]
pub struct CACHEMISSES {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Cache Misses Performance Counter"]
pub mod cachemisses;
#[doc = "Mass Erase Lock Register"]
pub struct MASSLOCK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Mass Erase Lock Register"]
pub mod masslock;
#[doc = "Startup Control"]
pub struct STARTUP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Startup Control"]
pub mod startup;
#[doc = "Command Register"]
pub struct CMD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Command Register"]
pub mod cmd;
