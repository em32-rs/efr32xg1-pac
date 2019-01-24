#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x04 - USART Frame Format Register"]
    pub frame: FRAME,
    #[doc = "0x08 - USART Trigger Control Register"]
    pub trigctrl: TRIGCTRL,
    #[doc = "0x0c - Command Register"]
    pub cmd: CMD,
    #[doc = "0x10 - USART Status Register"]
    pub status: STATUS,
    #[doc = "0x14 - Clock Control Register"]
    pub clkdiv: CLKDIV,
    #[doc = "0x18 - RX Buffer Data Extended Register"]
    pub rxdatax: RXDATAX,
    #[doc = "0x1c - RX Buffer Data Register"]
    pub rxdata: RXDATA,
    #[doc = "0x20 - RX Buffer Double Data Extended Register"]
    pub rxdoublex: RXDOUBLEX,
    #[doc = "0x24 - RX FIFO Double Data Register"]
    pub rxdouble: RXDOUBLE,
    #[doc = "0x28 - RX Buffer Data Extended Peek Register"]
    pub rxdataxp: RXDATAXP,
    #[doc = "0x2c - RX Buffer Double Data Extended Peek Register"]
    pub rxdoublexp: RXDOUBLEXP,
    #[doc = "0x30 - TX Buffer Data Extended Register"]
    pub txdatax: TXDATAX,
    #[doc = "0x34 - TX Buffer Data Register"]
    pub txdata: TXDATA,
    #[doc = "0x38 - TX Buffer Double Data Extended Register"]
    pub txdoublex: TXDOUBLEX,
    #[doc = "0x3c - TX Buffer Double Data Register"]
    pub txdouble: TXDOUBLE,
    #[doc = "0x40 - Interrupt Flag Register"]
    pub if_: IF,
    #[doc = "0x44 - Interrupt Flag Set Register"]
    pub ifs: IFS,
    #[doc = "0x48 - Interrupt Flag Clear Register"]
    pub ifc: IFC,
    #[doc = "0x4c - Interrupt Enable Register"]
    pub ien: IEN,
    #[doc = "0x50 - IrDA Control Register"]
    pub irctrl: IRCTRL,
    _reserved0: [u8; 4usize],
    #[doc = "0x58 - USART Input Register"]
    pub input: INPUT,
    #[doc = "0x5c - I2S Control Register"]
    pub i2sctrl: I2SCTRL,
    #[doc = "0x60 - Timing Register"]
    pub timing: TIMING,
    #[doc = "0x64 - Control Register Extended"]
    pub ctrlx: CTRLX,
    #[doc = "0x68 - Used to Generate Interrupts and Various Delays"]
    pub timecmp0: TIMECMP0,
    #[doc = "0x6c - Used to Generate Interrupts and Various Delays"]
    pub timecmp1: TIMECMP1,
    #[doc = "0x70 - Used to Generate Interrupts and Various Delays"]
    pub timecmp2: TIMECMP2,
    #[doc = "0x74 - I/O Routing Pin Enable Register"]
    pub routepen: ROUTEPEN,
    #[doc = "0x78 - I/O Routing Location Register"]
    pub routeloc0: ROUTELOC0,
    #[doc = "0x7c - I/O Routing Location Register"]
    pub routeloc1: ROUTELOC1,
}
#[doc = "Control Register"]
pub struct CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "USART Frame Format Register"]
pub struct FRAME {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USART Frame Format Register"]
pub mod frame;
#[doc = "USART Trigger Control Register"]
pub struct TRIGCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USART Trigger Control Register"]
pub mod trigctrl;
#[doc = "Command Register"]
pub struct CMD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Command Register"]
pub mod cmd;
#[doc = "USART Status Register"]
pub struct STATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USART Status Register"]
pub mod status;
#[doc = "Clock Control Register"]
pub struct CLKDIV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clock Control Register"]
pub mod clkdiv;
#[doc = "RX Buffer Data Extended Register"]
pub struct RXDATAX {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RX Buffer Data Extended Register"]
pub mod rxdatax;
#[doc = "RX Buffer Data Register"]
pub struct RXDATA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RX Buffer Data Register"]
pub mod rxdata;
#[doc = "RX Buffer Double Data Extended Register"]
pub struct RXDOUBLEX {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RX Buffer Double Data Extended Register"]
pub mod rxdoublex;
#[doc = "RX FIFO Double Data Register"]
pub struct RXDOUBLE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RX FIFO Double Data Register"]
pub mod rxdouble;
#[doc = "RX Buffer Data Extended Peek Register"]
pub struct RXDATAXP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RX Buffer Data Extended Peek Register"]
pub mod rxdataxp;
#[doc = "RX Buffer Double Data Extended Peek Register"]
pub struct RXDOUBLEXP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RX Buffer Double Data Extended Peek Register"]
pub mod rxdoublexp;
#[doc = "TX Buffer Data Extended Register"]
pub struct TXDATAX {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TX Buffer Data Extended Register"]
pub mod txdatax;
#[doc = "TX Buffer Data Register"]
pub struct TXDATA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TX Buffer Data Register"]
pub mod txdata;
#[doc = "TX Buffer Double Data Extended Register"]
pub struct TXDOUBLEX {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TX Buffer Double Data Extended Register"]
pub mod txdoublex;
#[doc = "TX Buffer Double Data Register"]
pub struct TXDOUBLE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TX Buffer Double Data Register"]
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
#[doc = "IrDA Control Register"]
pub struct IRCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IrDA Control Register"]
pub mod irctrl;
#[doc = "USART Input Register"]
pub struct INPUT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USART Input Register"]
pub mod input;
#[doc = "I2S Control Register"]
pub struct I2SCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2S Control Register"]
pub mod i2sctrl;
#[doc = "Timing Register"]
pub struct TIMING {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timing Register"]
pub mod timing;
#[doc = "Control Register Extended"]
pub struct CTRLX {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control Register Extended"]
pub mod ctrlx;
#[doc = "Used to Generate Interrupts and Various Delays"]
pub struct TIMECMP0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Used to Generate Interrupts and Various Delays"]
pub mod timecmp0;
#[doc = "Used to Generate Interrupts and Various Delays"]
pub struct TIMECMP1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Used to Generate Interrupts and Various Delays"]
pub mod timecmp1;
#[doc = "Used to Generate Interrupts and Various Delays"]
pub struct TIMECMP2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Used to Generate Interrupts and Various Delays"]
pub mod timecmp2;
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
