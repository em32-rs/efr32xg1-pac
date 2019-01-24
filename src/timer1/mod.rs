#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x04 - Command Register"]
    pub cmd: CMD,
    #[doc = "0x08 - Status Register"]
    pub status: STATUS,
    #[doc = "0x0c - Interrupt Flag Register"]
    pub if_: IF,
    #[doc = "0x10 - Interrupt Flag Set Register"]
    pub ifs: IFS,
    #[doc = "0x14 - Interrupt Flag Clear Register"]
    pub ifc: IFC,
    #[doc = "0x18 - Interrupt Enable Register"]
    pub ien: IEN,
    #[doc = "0x1c - Counter Top Value Register"]
    pub top: TOP,
    #[doc = "0x20 - Counter Top Value Buffer Register"]
    pub topb: TOPB,
    #[doc = "0x24 - Counter Value Register"]
    pub cnt: CNT,
    _reserved0: [u8; 4usize],
    #[doc = "0x2c - TIMER Configuration Lock Register"]
    pub lock: LOCK,
    #[doc = "0x30 - I/O Routing Pin Enable Register"]
    pub routepen: ROUTEPEN,
    #[doc = "0x34 - I/O Routing Location Register"]
    pub routeloc0: ROUTELOC0,
    _reserved1: [u8; 4usize],
    #[doc = "0x3c - I/O Routing Location Register"]
    pub routeloc2: ROUTELOC2,
    _reserved2: [u8; 32usize],
    #[doc = "0x60 - CC Channel Control Register"]
    pub cc0_ctrl: CC0_CTRL,
    #[doc = "0x64 - CC Channel Value Register"]
    pub cc0_ccv: CC0_CCV,
    #[doc = "0x68 - CC Channel Value Peek Register"]
    pub cc0_ccvp: CC0_CCVP,
    #[doc = "0x6c - CC Channel Buffer Register"]
    pub cc0_ccvb: CC0_CCVB,
    #[doc = "0x70 - CC Channel Control Register"]
    pub cc1_ctrl: CC1_CTRL,
    #[doc = "0x74 - CC Channel Value Register"]
    pub cc1_ccv: CC1_CCV,
    #[doc = "0x78 - CC Channel Value Peek Register"]
    pub cc1_ccvp: CC1_CCVP,
    #[doc = "0x7c - CC Channel Buffer Register"]
    pub cc1_ccvb: CC1_CCVB,
    #[doc = "0x80 - CC Channel Control Register"]
    pub cc2_ctrl: CC2_CTRL,
    #[doc = "0x84 - CC Channel Value Register"]
    pub cc2_ccv: CC2_CCV,
    #[doc = "0x88 - CC Channel Value Peek Register"]
    pub cc2_ccvp: CC2_CCVP,
    #[doc = "0x8c - CC Channel Buffer Register"]
    pub cc2_ccvb: CC2_CCVB,
    #[doc = "0x90 - CC Channel Control Register"]
    pub cc3_ctrl: CC3_CTRL,
    #[doc = "0x94 - CC Channel Value Register"]
    pub cc3_ccv: CC3_CCV,
    #[doc = "0x98 - CC Channel Value Peek Register"]
    pub cc3_ccvp: CC3_CCVP,
    #[doc = "0x9c - CC Channel Buffer Register"]
    pub cc3_ccvb: CC3_CCVB,
    #[doc = "0xa0 - DTI Control Register"]
    pub dtctrl: DTCTRL,
    #[doc = "0xa4 - DTI Time Control Register"]
    pub dttime: DTTIME,
    #[doc = "0xa8 - DTI Fault Configuration Register"]
    pub dtfc: DTFC,
    #[doc = "0xac - DTI Output Generation Enable Register"]
    pub dtogen: DTOGEN,
    #[doc = "0xb0 - DTI Fault Register"]
    pub dtfault: DTFAULT,
    #[doc = "0xb4 - DTI Fault Clear Register"]
    pub dtfaultc: DTFAULTC,
    #[doc = "0xb8 - DTI Configuration Lock Register"]
    pub dtlock: DTLOCK,
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
#[doc = "Counter Top Value Register"]
pub struct TOP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Counter Top Value Register"]
pub mod top;
#[doc = "Counter Top Value Buffer Register"]
pub struct TOPB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Counter Top Value Buffer Register"]
pub mod topb;
#[doc = "Counter Value Register"]
pub struct CNT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Counter Value Register"]
pub mod cnt;
#[doc = "TIMER Configuration Lock Register"]
pub struct LOCK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TIMER Configuration Lock Register"]
pub mod lock;
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
pub struct ROUTELOC2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O Routing Location Register"]
pub mod routeloc2;
#[doc = "CC Channel Control Register"]
pub struct CC0_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CC Channel Control Register"]
pub mod cc0_ctrl;
#[doc = "CC Channel Value Register"]
pub struct CC0_CCV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CC Channel Value Register"]
pub mod cc0_ccv;
#[doc = "CC Channel Value Peek Register"]
pub struct CC0_CCVP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CC Channel Value Peek Register"]
pub mod cc0_ccvp;
#[doc = "CC Channel Buffer Register"]
pub struct CC0_CCVB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CC Channel Buffer Register"]
pub mod cc0_ccvb;
#[doc = "CC Channel Control Register"]
pub struct CC1_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CC Channel Control Register"]
pub mod cc1_ctrl;
#[doc = "CC Channel Value Register"]
pub struct CC1_CCV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CC Channel Value Register"]
pub mod cc1_ccv;
#[doc = "CC Channel Value Peek Register"]
pub struct CC1_CCVP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CC Channel Value Peek Register"]
pub mod cc1_ccvp;
#[doc = "CC Channel Buffer Register"]
pub struct CC1_CCVB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CC Channel Buffer Register"]
pub mod cc1_ccvb;
#[doc = "CC Channel Control Register"]
pub struct CC2_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CC Channel Control Register"]
pub mod cc2_ctrl;
#[doc = "CC Channel Value Register"]
pub struct CC2_CCV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CC Channel Value Register"]
pub mod cc2_ccv;
#[doc = "CC Channel Value Peek Register"]
pub struct CC2_CCVP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CC Channel Value Peek Register"]
pub mod cc2_ccvp;
#[doc = "CC Channel Buffer Register"]
pub struct CC2_CCVB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CC Channel Buffer Register"]
pub mod cc2_ccvb;
#[doc = "CC Channel Control Register"]
pub struct CC3_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CC Channel Control Register"]
pub mod cc3_ctrl;
#[doc = "CC Channel Value Register"]
pub struct CC3_CCV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CC Channel Value Register"]
pub mod cc3_ccv;
#[doc = "CC Channel Value Peek Register"]
pub struct CC3_CCVP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CC Channel Value Peek Register"]
pub mod cc3_ccvp;
#[doc = "CC Channel Buffer Register"]
pub struct CC3_CCVB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CC Channel Buffer Register"]
pub mod cc3_ccvb;
#[doc = "DTI Control Register"]
pub struct DTCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DTI Control Register"]
pub mod dtctrl;
#[doc = "DTI Time Control Register"]
pub struct DTTIME {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DTI Time Control Register"]
pub mod dttime;
#[doc = "DTI Fault Configuration Register"]
pub struct DTFC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DTI Fault Configuration Register"]
pub mod dtfc;
#[doc = "DTI Output Generation Enable Register"]
pub struct DTOGEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DTI Output Generation Enable Register"]
pub mod dtogen;
#[doc = "DTI Fault Register"]
pub struct DTFAULT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DTI Fault Register"]
pub mod dtfault;
#[doc = "DTI Fault Clear Register"]
pub struct DTFAULTC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DTI Fault Clear Register"]
pub mod dtfaultc;
#[doc = "DTI Configuration Lock Register"]
pub struct DTLOCK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DTI Configuration Lock Register"]
pub mod dtlock;
