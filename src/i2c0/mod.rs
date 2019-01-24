#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x04 - Command Register"]
    pub cmd: CMD,
    #[doc = "0x08 - State Register"]
    pub state: STATE,
    #[doc = "0x0c - Status Register"]
    pub status: STATUS,
    #[doc = "0x10 - Clock Division Register"]
    pub clkdiv: CLKDIV,
    #[doc = "0x14 - Slave Address Register"]
    pub saddr: SADDR,
    #[doc = "0x18 - Slave Address Mask Register"]
    pub saddrmask: SADDRMASK,
    #[doc = "0x1c - Receive Buffer Data Register"]
    pub rxdata: RXDATA,
    #[doc = "0x20 - Receive Buffer Double Data Register"]
    pub rxdouble: RXDOUBLE,
    #[doc = "0x24 - Receive Buffer Data Peek Register"]
    pub rxdatap: RXDATAP,
    #[doc = "0x28 - Receive Buffer Double Data Peek Register"]
    pub rxdoublep: RXDOUBLEP,
    #[doc = "0x2c - Transmit Buffer Data Register"]
    pub txdata: TXDATA,
    #[doc = "0x30 - Transmit Buffer Double Data Register"]
    pub txdouble: TXDOUBLE,
    #[doc = "0x34 - Interrupt Flag Register"]
    pub if_: IF,
    #[doc = "0x38 - Interrupt Flag Set Register"]
    pub ifs: IFS,
    #[doc = "0x3c - Interrupt Flag Clear Register"]
    pub ifc: IFC,
    #[doc = "0x40 - Interrupt Enable Register"]
    pub ien: IEN,
    #[doc = "0x44 - I/O Routing Pin Enable Register"]
    pub routepen: ROUTEPEN,
    #[doc = "0x48 - I/O Routing Location Register"]
    pub routeloc0: ROUTELOC0,
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
#[doc = "State Register"]
pub struct STATE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "State Register"]
pub mod state;
#[doc = "Status Register"]
pub struct STATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status Register"]
pub mod status;
#[doc = "Clock Division Register"]
pub struct CLKDIV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clock Division Register"]
pub mod clkdiv;
#[doc = "Slave Address Register"]
pub struct SADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Slave Address Register"]
pub mod saddr;
#[doc = "Slave Address Mask Register"]
pub struct SADDRMASK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Slave Address Mask Register"]
pub mod saddrmask;
#[doc = "Receive Buffer Data Register"]
pub struct RXDATA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive Buffer Data Register"]
pub mod rxdata;
#[doc = "Receive Buffer Double Data Register"]
pub struct RXDOUBLE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive Buffer Double Data Register"]
pub mod rxdouble;
#[doc = "Receive Buffer Data Peek Register"]
pub struct RXDATAP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive Buffer Data Peek Register"]
pub mod rxdatap;
#[doc = "Receive Buffer Double Data Peek Register"]
pub struct RXDOUBLEP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive Buffer Double Data Peek Register"]
pub mod rxdoublep;
#[doc = "Transmit Buffer Data Register"]
pub struct TXDATA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit Buffer Data Register"]
pub mod txdata;
#[doc = "Transmit Buffer Double Data Register"]
pub struct TXDOUBLE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit Buffer Double Data Register"]
pub mod txdouble;
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
