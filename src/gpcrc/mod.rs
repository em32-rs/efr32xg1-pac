#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x04 - Command Register"]
    pub cmd: CMD,
    #[doc = "0x08 - CRC Init Value"]
    pub init: INIT,
    #[doc = "0x0c - CRC Polynomial Value"]
    pub poly: POLY,
    #[doc = "0x10 - Input 32-bit Data Register"]
    pub inputdata: INPUTDATA,
    #[doc = "0x14 - Input 16-bit Data Register"]
    pub inputdatahword: INPUTDATAHWORD,
    #[doc = "0x18 - Input 8-bit Data Register"]
    pub inputdatabyte: INPUTDATABYTE,
    #[doc = "0x1c - CRC Data Register"]
    pub data: DATA,
    #[doc = "0x20 - CRC Data Reverse Register"]
    pub datarev: DATAREV,
    #[doc = "0x24 - CRC Data Byte Reverse Register"]
    pub databyterev: DATABYTEREV,
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
#[doc = "CRC Init Value"]
pub struct INIT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CRC Init Value"]
pub mod init;
#[doc = "CRC Polynomial Value"]
pub struct POLY {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CRC Polynomial Value"]
pub mod poly;
#[doc = "Input 32-bit Data Register"]
pub struct INPUTDATA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Input 32-bit Data Register"]
pub mod inputdata;
#[doc = "Input 16-bit Data Register"]
pub struct INPUTDATAHWORD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Input 16-bit Data Register"]
pub mod inputdatahword;
#[doc = "Input 8-bit Data Register"]
pub struct INPUTDATABYTE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Input 8-bit Data Register"]
pub mod inputdatabyte;
#[doc = "CRC Data Register"]
pub struct DATA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CRC Data Register"]
pub mod data;
#[doc = "CRC Data Reverse Register"]
pub struct DATAREV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CRC Data Reverse Register"]
pub mod datarev;
#[doc = "CRC Data Byte Reverse Register"]
pub struct DATABYTEREV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CRC Data Byte Reverse Register"]
pub mod databyterev;
