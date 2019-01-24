#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x04 - Wide Arithmetic Configuration"]
    pub wac: WAC,
    #[doc = "0x08 - Command Register"]
    pub cmd: CMD,
    _reserved0: [u8; 4usize],
    #[doc = "0x10 - Status Register"]
    pub status: STATUS,
    #[doc = "0x14 - Data Status Register"]
    pub dstatus: DSTATUS,
    #[doc = "0x18 - Control Status Register"]
    pub cstatus: CSTATUS,
    _reserved1: [u8; 4usize],
    #[doc = "0x20 - KEY Register Access"]
    pub key: KEY,
    #[doc = "0x24 - KEY Buffer Register Access"]
    pub keybuf: KEYBUF,
    _reserved2: [u8; 8usize],
    #[doc = "0x30 - Sequence Control"]
    pub seqctrl: SEQCTRL,
    #[doc = "0x34 - Sequence Control B"]
    pub seqctrlb: SEQCTRLB,
    _reserved3: [u8; 8usize],
    #[doc = "0x40 - AES Interrupt Flags"]
    pub if_: IF,
    #[doc = "0x44 - Interrupt Flag Set Register"]
    pub ifs: IFS,
    #[doc = "0x48 - Interrupt Flag Clear Register"]
    pub ifc: IFC,
    #[doc = "0x4c - Interrupt Enable Register"]
    pub ien: IEN,
    #[doc = "0x50 - Sequence Register 0"]
    pub seq0: SEQ0,
    #[doc = "0x54 - Sequence Register 1"]
    pub seq1: SEQ1,
    #[doc = "0x58 - Sequence Register 2"]
    pub seq2: SEQ2,
    #[doc = "0x5c - Sequence Register 3"]
    pub seq3: SEQ3,
    #[doc = "0x60 - Sequence Register 4"]
    pub seq4: SEQ4,
    _reserved4: [u8; 28usize],
    #[doc = "0x80 - DATA0 Register Access"]
    pub data0: DATA0,
    #[doc = "0x84 - DATA1 Register Access"]
    pub data1: DATA1,
    #[doc = "0x88 - DATA2 Register Access"]
    pub data2: DATA2,
    #[doc = "0x8c - DATA3 Register Access"]
    pub data3: DATA3,
    _reserved5: [u8; 16usize],
    #[doc = "0xa0 - DATA0XOR Register Access"]
    pub data0xor: DATA0XOR,
    _reserved6: [u8; 12usize],
    #[doc = "0xb0 - DATA0 Register Byte Access"]
    pub data0byte: DATA0BYTE,
    #[doc = "0xb4 - DATA1 Register Byte Access"]
    pub data1byte: DATA1BYTE,
    _reserved7: [u8; 4usize],
    #[doc = "0xbc - DATA0 Register Byte XOR Access"]
    pub data0xorbyte: DATA0XORBYTE,
    #[doc = "0xc0 - DATA0 Register Byte 12 Access"]
    pub data0byte12: DATA0BYTE12,
    #[doc = "0xc4 - DATA0 Register Byte 13 Access"]
    pub data0byte13: DATA0BYTE13,
    #[doc = "0xc8 - DATA0 Register Byte 14 Access"]
    pub data0byte14: DATA0BYTE14,
    #[doc = "0xcc - DATA0 Register Byte 15 Access"]
    pub data0byte15: DATA0BYTE15,
    _reserved8: [u8; 48usize],
    #[doc = "0x100 - DDATA0 Register Access"]
    pub ddata0: DDATA0,
    #[doc = "0x104 - DDATA1 Register Access"]
    pub ddata1: DDATA1,
    #[doc = "0x108 - DDATA2 Register Access"]
    pub ddata2: DDATA2,
    #[doc = "0x10c - DDATA3 Register Access"]
    pub ddata3: DDATA3,
    #[doc = "0x110 - DDATA4 Register Access"]
    pub ddata4: DDATA4,
    _reserved9: [u8; 28usize],
    #[doc = "0x130 - DDATA0 Register Big Endian Access"]
    pub ddata0big: DDATA0BIG,
    _reserved10: [u8; 12usize],
    #[doc = "0x140 - DDATA0 Register Byte Access"]
    pub ddata0byte: DDATA0BYTE,
    #[doc = "0x144 - DDATA1 Register Byte Access"]
    pub ddata1byte: DDATA1BYTE,
    #[doc = "0x148 - DDATA0 Register Byte 32 Access"]
    pub ddata0byte32: DDATA0BYTE32,
    _reserved11: [u8; 52usize],
    #[doc = "0x180 - QDATA0 Register Access"]
    pub qdata0: QDATA0,
    #[doc = "0x184 - QDATA1 Register Access"]
    pub qdata1: QDATA1,
    _reserved12: [u8; 28usize],
    #[doc = "0x1a4 - QDATA1 Register Big Endian Access"]
    pub qdata1big: QDATA1BIG,
    _reserved13: [u8; 24usize],
    #[doc = "0x1c0 - QDATA0 Register Byte Access"]
    pub qdata0byte: QDATA0BYTE,
    #[doc = "0x1c4 - QDATA1 Register Byte Access"]
    pub qdata1byte: QDATA1BYTE,
}
#[doc = "Control Register"]
pub struct CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "Wide Arithmetic Configuration"]
pub struct WAC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Wide Arithmetic Configuration"]
pub mod wac;
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
#[doc = "Data Status Register"]
pub struct DSTATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Data Status Register"]
pub mod dstatus;
#[doc = "Control Status Register"]
pub struct CSTATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control Status Register"]
pub mod cstatus;
#[doc = "KEY Register Access"]
pub struct KEY {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "KEY Register Access"]
pub mod key;
#[doc = "KEY Buffer Register Access"]
pub struct KEYBUF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "KEY Buffer Register Access"]
pub mod keybuf;
#[doc = "Sequence Control"]
pub struct SEQCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Sequence Control"]
pub mod seqctrl;
#[doc = "Sequence Control B"]
pub struct SEQCTRLB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Sequence Control B"]
pub mod seqctrlb;
#[doc = "AES Interrupt Flags"]
pub struct IF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AES Interrupt Flags"]
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
#[doc = "Sequence Register 0"]
pub struct SEQ0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Sequence Register 0"]
pub mod seq0;
#[doc = "Sequence Register 1"]
pub struct SEQ1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Sequence Register 1"]
pub mod seq1;
#[doc = "Sequence Register 2"]
pub struct SEQ2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Sequence Register 2"]
pub mod seq2;
#[doc = "Sequence Register 3"]
pub struct SEQ3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Sequence Register 3"]
pub mod seq3;
#[doc = "Sequence Register 4"]
pub struct SEQ4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Sequence Register 4"]
pub mod seq4;
#[doc = "DATA0 Register Access"]
pub struct DATA0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DATA0 Register Access"]
pub mod data0;
#[doc = "DATA1 Register Access"]
pub struct DATA1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DATA1 Register Access"]
pub mod data1;
#[doc = "DATA2 Register Access"]
pub struct DATA2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DATA2 Register Access"]
pub mod data2;
#[doc = "DATA3 Register Access"]
pub struct DATA3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DATA3 Register Access"]
pub mod data3;
#[doc = "DATA0XOR Register Access"]
pub struct DATA0XOR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DATA0XOR Register Access"]
pub mod data0xor;
#[doc = "DATA0 Register Byte Access"]
pub struct DATA0BYTE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DATA0 Register Byte Access"]
pub mod data0byte;
#[doc = "DATA1 Register Byte Access"]
pub struct DATA1BYTE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DATA1 Register Byte Access"]
pub mod data1byte;
#[doc = "DATA0 Register Byte XOR Access"]
pub struct DATA0XORBYTE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DATA0 Register Byte XOR Access"]
pub mod data0xorbyte;
#[doc = "DATA0 Register Byte 12 Access"]
pub struct DATA0BYTE12 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DATA0 Register Byte 12 Access"]
pub mod data0byte12;
#[doc = "DATA0 Register Byte 13 Access"]
pub struct DATA0BYTE13 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DATA0 Register Byte 13 Access"]
pub mod data0byte13;
#[doc = "DATA0 Register Byte 14 Access"]
pub struct DATA0BYTE14 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DATA0 Register Byte 14 Access"]
pub mod data0byte14;
#[doc = "DATA0 Register Byte 15 Access"]
pub struct DATA0BYTE15 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DATA0 Register Byte 15 Access"]
pub mod data0byte15;
#[doc = "DDATA0 Register Access"]
pub struct DDATA0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DDATA0 Register Access"]
pub mod ddata0;
#[doc = "DDATA1 Register Access"]
pub struct DDATA1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DDATA1 Register Access"]
pub mod ddata1;
#[doc = "DDATA2 Register Access"]
pub struct DDATA2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DDATA2 Register Access"]
pub mod ddata2;
#[doc = "DDATA3 Register Access"]
pub struct DDATA3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DDATA3 Register Access"]
pub mod ddata3;
#[doc = "DDATA4 Register Access"]
pub struct DDATA4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DDATA4 Register Access"]
pub mod ddata4;
#[doc = "DDATA0 Register Big Endian Access"]
pub struct DDATA0BIG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DDATA0 Register Big Endian Access"]
pub mod ddata0big;
#[doc = "DDATA0 Register Byte Access"]
pub struct DDATA0BYTE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DDATA0 Register Byte Access"]
pub mod ddata0byte;
#[doc = "DDATA1 Register Byte Access"]
pub struct DDATA1BYTE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DDATA1 Register Byte Access"]
pub mod ddata1byte;
#[doc = "DDATA0 Register Byte 32 Access"]
pub struct DDATA0BYTE32 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DDATA0 Register Byte 32 Access"]
pub mod ddata0byte32;
#[doc = "QDATA0 Register Access"]
pub struct QDATA0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "QDATA0 Register Access"]
pub mod qdata0;
#[doc = "QDATA1 Register Access"]
pub struct QDATA1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "QDATA1 Register Access"]
pub mod qdata1;
#[doc = "QDATA1 Register Big Endian Access"]
pub struct QDATA1BIG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "QDATA1 Register Big Endian Access"]
pub mod qdata1big;
#[doc = "QDATA0 Register Byte Access"]
pub struct QDATA0BYTE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "QDATA0 Register Byte Access"]
pub mod qdata0byte;
#[doc = "QDATA1 Register Byte Access"]
pub struct QDATA1BYTE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "QDATA1 Register Byte Access"]
pub mod qdata1byte;
