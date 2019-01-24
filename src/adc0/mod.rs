#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub ctrl: CTRL,
    _reserved0: [u8; 4usize],
    #[doc = "0x08 - Command Register"]
    pub cmd: CMD,
    #[doc = "0x0c - Status Register"]
    pub status: STATUS,
    #[doc = "0x10 - Single Channel Control Register"]
    pub singlectrl: SINGLECTRL,
    #[doc = "0x14 - Single Channel Control Register Continued"]
    pub singlectrlx: SINGLECTRLX,
    #[doc = "0x18 - Scan Control Register"]
    pub scanctrl: SCANCTRL,
    #[doc = "0x1c - Scan Control Register Continued"]
    pub scanctrlx: SCANCTRLX,
    #[doc = "0x20 - Scan Sequence Input Mask Register"]
    pub scanmask: SCANMASK,
    #[doc = "0x24 - Input Selection Register for Scan Mode"]
    pub scaninputsel: SCANINPUTSEL,
    #[doc = "0x28 - Negative Input Select Register for Scan"]
    pub scannegsel: SCANNEGSEL,
    #[doc = "0x2c - Compare Threshold Register"]
    pub cmpthr: CMPTHR,
    #[doc = "0x30 - Bias Programming Register for Various Analog Blocks Used in ADC Operation"]
    pub biasprog: BIASPROG,
    #[doc = "0x34 - Calibration Register"]
    pub cal: CAL,
    #[doc = "0x38 - Interrupt Flag Register"]
    pub if_: IF,
    #[doc = "0x3c - Interrupt Flag Set Register"]
    pub ifs: IFS,
    #[doc = "0x40 - Interrupt Flag Clear Register"]
    pub ifc: IFC,
    #[doc = "0x44 - Interrupt Enable Register"]
    pub ien: IEN,
    #[doc = "0x48 - Single Conversion Result Data"]
    pub singledata: SINGLEDATA,
    #[doc = "0x4c - Scan Conversion Result Data"]
    pub scandata: SCANDATA,
    #[doc = "0x50 - Single Conversion Result Data Peek Register"]
    pub singledatap: SINGLEDATAP,
    #[doc = "0x54 - Scan Sequence Result Data Peek Register"]
    pub scandatap: SCANDATAP,
    _reserved1: [u8; 16usize],
    #[doc = "0x68 - Scan Sequence Result Data + Data Source Register"]
    pub scandatax: SCANDATAX,
    #[doc = "0x6c - Scan Sequence Result Data + Data Source Peek Register"]
    pub scandataxp: SCANDATAXP,
    _reserved2: [u8; 12usize],
    #[doc = "0x7c - APORT Request Status Register"]
    pub aportreq: APORTREQ,
    #[doc = "0x80 - APORT Conflict Status Register"]
    pub aportconflict: APORTCONFLICT,
    #[doc = "0x84 - Single FIFO Count Register"]
    pub singlefifocount: SINGLEFIFOCOUNT,
    #[doc = "0x88 - Scan FIFO Count Register"]
    pub scanfifocount: SCANFIFOCOUNT,
    #[doc = "0x8c - Single FIFO Clear Register"]
    pub singlefifoclear: SINGLEFIFOCLEAR,
    #[doc = "0x90 - Scan FIFO Clear Register"]
    pub scanfifoclear: SCANFIFOCLEAR,
    #[doc = "0x94 - APORT Bus Master Disable Register"]
    pub aportmasterdis: APORTMASTERDIS,
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
#[doc = "Single Channel Control Register"]
pub struct SINGLECTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Single Channel Control Register"]
pub mod singlectrl;
#[doc = "Single Channel Control Register Continued"]
pub struct SINGLECTRLX {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Single Channel Control Register Continued"]
pub mod singlectrlx;
#[doc = "Scan Control Register"]
pub struct SCANCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Scan Control Register"]
pub mod scanctrl;
#[doc = "Scan Control Register Continued"]
pub struct SCANCTRLX {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Scan Control Register Continued"]
pub mod scanctrlx;
#[doc = "Scan Sequence Input Mask Register"]
pub struct SCANMASK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Scan Sequence Input Mask Register"]
pub mod scanmask;
#[doc = "Input Selection Register for Scan Mode"]
pub struct SCANINPUTSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Input Selection Register for Scan Mode"]
pub mod scaninputsel;
#[doc = "Negative Input Select Register for Scan"]
pub struct SCANNEGSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Negative Input Select Register for Scan"]
pub mod scannegsel;
#[doc = "Compare Threshold Register"]
pub struct CMPTHR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Compare Threshold Register"]
pub mod cmpthr;
#[doc = "Bias Programming Register for Various Analog Blocks Used in ADC Operation"]
pub struct BIASPROG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Bias Programming Register for Various Analog Blocks Used in ADC Operation"]
pub mod biasprog;
#[doc = "Calibration Register"]
pub struct CAL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Calibration Register"]
pub mod cal;
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
#[doc = "Single Conversion Result Data"]
pub struct SINGLEDATA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Single Conversion Result Data"]
pub mod singledata;
#[doc = "Scan Conversion Result Data"]
pub struct SCANDATA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Scan Conversion Result Data"]
pub mod scandata;
#[doc = "Single Conversion Result Data Peek Register"]
pub struct SINGLEDATAP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Single Conversion Result Data Peek Register"]
pub mod singledatap;
#[doc = "Scan Sequence Result Data Peek Register"]
pub struct SCANDATAP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Scan Sequence Result Data Peek Register"]
pub mod scandatap;
#[doc = "Scan Sequence Result Data + Data Source Register"]
pub struct SCANDATAX {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Scan Sequence Result Data + Data Source Register"]
pub mod scandatax;
#[doc = "Scan Sequence Result Data + Data Source Peek Register"]
pub struct SCANDATAXP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Scan Sequence Result Data + Data Source Peek Register"]
pub mod scandataxp;
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
#[doc = "Single FIFO Count Register"]
pub struct SINGLEFIFOCOUNT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Single FIFO Count Register"]
pub mod singlefifocount;
#[doc = "Scan FIFO Count Register"]
pub struct SCANFIFOCOUNT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Scan FIFO Count Register"]
pub mod scanfifocount;
#[doc = "Single FIFO Clear Register"]
pub struct SINGLEFIFOCLEAR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Single FIFO Clear Register"]
pub mod singlefifoclear;
#[doc = "Scan FIFO Clear Register"]
pub struct SCANFIFOCLEAR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Scan FIFO Clear Register"]
pub mod scanfifoclear;
#[doc = "APORT Bus Master Disable Register"]
pub struct APORTMASTERDIS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "APORT Bus Master Disable Register"]
pub mod aportmasterdis;
