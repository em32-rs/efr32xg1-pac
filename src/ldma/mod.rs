#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DMA Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x04 - DMA Status Register"]
    pub status: STATUS,
    #[doc = "0x08 - DMA Synchronization Trigger Register (Single-Cycle RMW)"]
    pub sync: SYNC,
    _reserved0: [u8; 20usize],
    #[doc = "0x20 - DMA Channel Enable Register (Single-Cycle RMW)"]
    pub chen: CHEN,
    #[doc = "0x24 - DMA Channel Busy Register"]
    pub chbusy: CHBUSY,
    #[doc = "0x28 - DMA Channel Linking Done Register (Single-Cycle RMW)"]
    pub chdone: CHDONE,
    #[doc = "0x2c - DMA Channel Debug Halt Register"]
    pub dbghalt: DBGHALT,
    #[doc = "0x30 - DMA Channel Software Transfer Request Register"]
    pub swreq: SWREQ,
    #[doc = "0x34 - DMA Channel Request Disable Register"]
    pub reqdis: REQDIS,
    #[doc = "0x38 - DMA Channel Requests Pending Register"]
    pub reqpend: REQPEND,
    #[doc = "0x3c - DMA Channel Link Load Register"]
    pub linkload: LINKLOAD,
    #[doc = "0x40 - DMA Channel Request Clear Register"]
    pub reqclear: REQCLEAR,
    _reserved1: [u8; 28usize],
    #[doc = "0x60 - Interrupt Flag Register"]
    pub if_: IF,
    #[doc = "0x64 - Interrupt Flag Set Register"]
    pub ifs: IFS,
    #[doc = "0x68 - Interrupt Flag Clear Register"]
    pub ifc: IFC,
    #[doc = "0x6c - Interrupt Enable Register"]
    pub ien: IEN,
    _reserved2: [u8; 16usize],
    #[doc = "0x80 - Channel Peripheral Request Select Register"]
    pub ch0_reqsel: CH0_REQSEL,
    #[doc = "0x84 - Channel Configuration Register"]
    pub ch0_cfg: CH0_CFG,
    #[doc = "0x88 - Channel Loop Counter Register"]
    pub ch0_loop: CH0_LOOP,
    #[doc = "0x8c - Channel Descriptor Control Word Register"]
    pub ch0_ctrl: CH0_CTRL,
    #[doc = "0x90 - Channel Descriptor Source Data Address Register"]
    pub ch0_src: CH0_SRC,
    #[doc = "0x94 - Channel Descriptor Destination Data Address Register"]
    pub ch0_dst: CH0_DST,
    #[doc = "0x98 - Channel Descriptor Link Structure Address Register"]
    pub ch0_link: CH0_LINK,
    _reserved3: [u8; 20usize],
    #[doc = "0xb0 - Channel Peripheral Request Select Register"]
    pub ch1_reqsel: CH1_REQSEL,
    #[doc = "0xb4 - Channel Configuration Register"]
    pub ch1_cfg: CH1_CFG,
    #[doc = "0xb8 - Channel Loop Counter Register"]
    pub ch1_loop: CH1_LOOP,
    #[doc = "0xbc - Channel Descriptor Control Word Register"]
    pub ch1_ctrl: CH1_CTRL,
    #[doc = "0xc0 - Channel Descriptor Source Data Address Register"]
    pub ch1_src: CH1_SRC,
    #[doc = "0xc4 - Channel Descriptor Destination Data Address Register"]
    pub ch1_dst: CH1_DST,
    #[doc = "0xc8 - Channel Descriptor Link Structure Address Register"]
    pub ch1_link: CH1_LINK,
    _reserved4: [u8; 20usize],
    #[doc = "0xe0 - Channel Peripheral Request Select Register"]
    pub ch2_reqsel: CH2_REQSEL,
    #[doc = "0xe4 - Channel Configuration Register"]
    pub ch2_cfg: CH2_CFG,
    #[doc = "0xe8 - Channel Loop Counter Register"]
    pub ch2_loop: CH2_LOOP,
    #[doc = "0xec - Channel Descriptor Control Word Register"]
    pub ch2_ctrl: CH2_CTRL,
    #[doc = "0xf0 - Channel Descriptor Source Data Address Register"]
    pub ch2_src: CH2_SRC,
    #[doc = "0xf4 - Channel Descriptor Destination Data Address Register"]
    pub ch2_dst: CH2_DST,
    #[doc = "0xf8 - Channel Descriptor Link Structure Address Register"]
    pub ch2_link: CH2_LINK,
    _reserved5: [u8; 20usize],
    #[doc = "0x110 - Channel Peripheral Request Select Register"]
    pub ch3_reqsel: CH3_REQSEL,
    #[doc = "0x114 - Channel Configuration Register"]
    pub ch3_cfg: CH3_CFG,
    #[doc = "0x118 - Channel Loop Counter Register"]
    pub ch3_loop: CH3_LOOP,
    #[doc = "0x11c - Channel Descriptor Control Word Register"]
    pub ch3_ctrl: CH3_CTRL,
    #[doc = "0x120 - Channel Descriptor Source Data Address Register"]
    pub ch3_src: CH3_SRC,
    #[doc = "0x124 - Channel Descriptor Destination Data Address Register"]
    pub ch3_dst: CH3_DST,
    #[doc = "0x128 - Channel Descriptor Link Structure Address Register"]
    pub ch3_link: CH3_LINK,
    _reserved6: [u8; 20usize],
    #[doc = "0x140 - Channel Peripheral Request Select Register"]
    pub ch4_reqsel: CH4_REQSEL,
    #[doc = "0x144 - Channel Configuration Register"]
    pub ch4_cfg: CH4_CFG,
    #[doc = "0x148 - Channel Loop Counter Register"]
    pub ch4_loop: CH4_LOOP,
    #[doc = "0x14c - Channel Descriptor Control Word Register"]
    pub ch4_ctrl: CH4_CTRL,
    #[doc = "0x150 - Channel Descriptor Source Data Address Register"]
    pub ch4_src: CH4_SRC,
    #[doc = "0x154 - Channel Descriptor Destination Data Address Register"]
    pub ch4_dst: CH4_DST,
    #[doc = "0x158 - Channel Descriptor Link Structure Address Register"]
    pub ch4_link: CH4_LINK,
    _reserved7: [u8; 20usize],
    #[doc = "0x170 - Channel Peripheral Request Select Register"]
    pub ch5_reqsel: CH5_REQSEL,
    #[doc = "0x174 - Channel Configuration Register"]
    pub ch5_cfg: CH5_CFG,
    #[doc = "0x178 - Channel Loop Counter Register"]
    pub ch5_loop: CH5_LOOP,
    #[doc = "0x17c - Channel Descriptor Control Word Register"]
    pub ch5_ctrl: CH5_CTRL,
    #[doc = "0x180 - Channel Descriptor Source Data Address Register"]
    pub ch5_src: CH5_SRC,
    #[doc = "0x184 - Channel Descriptor Destination Data Address Register"]
    pub ch5_dst: CH5_DST,
    #[doc = "0x188 - Channel Descriptor Link Structure Address Register"]
    pub ch5_link: CH5_LINK,
    _reserved8: [u8; 20usize],
    #[doc = "0x1a0 - Channel Peripheral Request Select Register"]
    pub ch6_reqsel: CH6_REQSEL,
    #[doc = "0x1a4 - Channel Configuration Register"]
    pub ch6_cfg: CH6_CFG,
    #[doc = "0x1a8 - Channel Loop Counter Register"]
    pub ch6_loop: CH6_LOOP,
    #[doc = "0x1ac - Channel Descriptor Control Word Register"]
    pub ch6_ctrl: CH6_CTRL,
    #[doc = "0x1b0 - Channel Descriptor Source Data Address Register"]
    pub ch6_src: CH6_SRC,
    #[doc = "0x1b4 - Channel Descriptor Destination Data Address Register"]
    pub ch6_dst: CH6_DST,
    #[doc = "0x1b8 - Channel Descriptor Link Structure Address Register"]
    pub ch6_link: CH6_LINK,
    _reserved9: [u8; 20usize],
    #[doc = "0x1d0 - Channel Peripheral Request Select Register"]
    pub ch7_reqsel: CH7_REQSEL,
    #[doc = "0x1d4 - Channel Configuration Register"]
    pub ch7_cfg: CH7_CFG,
    #[doc = "0x1d8 - Channel Loop Counter Register"]
    pub ch7_loop: CH7_LOOP,
    #[doc = "0x1dc - Channel Descriptor Control Word Register"]
    pub ch7_ctrl: CH7_CTRL,
    #[doc = "0x1e0 - Channel Descriptor Source Data Address Register"]
    pub ch7_src: CH7_SRC,
    #[doc = "0x1e4 - Channel Descriptor Destination Data Address Register"]
    pub ch7_dst: CH7_DST,
    #[doc = "0x1e8 - Channel Descriptor Link Structure Address Register"]
    pub ch7_link: CH7_LINK,
}
#[doc = "DMA Control Register"]
pub struct CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Control Register"]
pub mod ctrl;
#[doc = "DMA Status Register"]
pub struct STATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Status Register"]
pub mod status;
#[doc = "DMA Synchronization Trigger Register (Single-Cycle RMW)"]
pub struct SYNC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Synchronization Trigger Register (Single-Cycle RMW)"]
pub mod sync;
#[doc = "DMA Channel Enable Register (Single-Cycle RMW)"]
pub struct CHEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Channel Enable Register (Single-Cycle RMW)"]
pub mod chen;
#[doc = "DMA Channel Busy Register"]
pub struct CHBUSY {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Channel Busy Register"]
pub mod chbusy;
#[doc = "DMA Channel Linking Done Register (Single-Cycle RMW)"]
pub struct CHDONE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Channel Linking Done Register (Single-Cycle RMW)"]
pub mod chdone;
#[doc = "DMA Channel Debug Halt Register"]
pub struct DBGHALT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Channel Debug Halt Register"]
pub mod dbghalt;
#[doc = "DMA Channel Software Transfer Request Register"]
pub struct SWREQ {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Channel Software Transfer Request Register"]
pub mod swreq;
#[doc = "DMA Channel Request Disable Register"]
pub struct REQDIS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Channel Request Disable Register"]
pub mod reqdis;
#[doc = "DMA Channel Requests Pending Register"]
pub struct REQPEND {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Channel Requests Pending Register"]
pub mod reqpend;
#[doc = "DMA Channel Link Load Register"]
pub struct LINKLOAD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Channel Link Load Register"]
pub mod linkload;
#[doc = "DMA Channel Request Clear Register"]
pub struct REQCLEAR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Channel Request Clear Register"]
pub mod reqclear;
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
#[doc = "Channel Peripheral Request Select Register"]
pub struct CH0_REQSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Peripheral Request Select Register"]
pub mod ch0_reqsel;
#[doc = "Channel Configuration Register"]
pub struct CH0_CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Configuration Register"]
pub mod ch0_cfg;
#[doc = "Channel Loop Counter Register"]
pub struct CH0_LOOP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Loop Counter Register"]
pub mod ch0_loop;
#[doc = "Channel Descriptor Control Word Register"]
pub struct CH0_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Descriptor Control Word Register"]
pub mod ch0_ctrl;
#[doc = "Channel Descriptor Source Data Address Register"]
pub struct CH0_SRC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Descriptor Source Data Address Register"]
pub mod ch0_src;
#[doc = "Channel Descriptor Destination Data Address Register"]
pub struct CH0_DST {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Descriptor Destination Data Address Register"]
pub mod ch0_dst;
#[doc = "Channel Descriptor Link Structure Address Register"]
pub struct CH0_LINK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Descriptor Link Structure Address Register"]
pub mod ch0_link;
#[doc = "Channel Peripheral Request Select Register"]
pub struct CH1_REQSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Peripheral Request Select Register"]
pub mod ch1_reqsel;
#[doc = "Channel Configuration Register"]
pub struct CH1_CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Configuration Register"]
pub mod ch1_cfg;
#[doc = "Channel Loop Counter Register"]
pub struct CH1_LOOP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Loop Counter Register"]
pub mod ch1_loop;
#[doc = "Channel Descriptor Control Word Register"]
pub struct CH1_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Descriptor Control Word Register"]
pub mod ch1_ctrl;
#[doc = "Channel Descriptor Source Data Address Register"]
pub struct CH1_SRC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Descriptor Source Data Address Register"]
pub mod ch1_src;
#[doc = "Channel Descriptor Destination Data Address Register"]
pub struct CH1_DST {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Descriptor Destination Data Address Register"]
pub mod ch1_dst;
#[doc = "Channel Descriptor Link Structure Address Register"]
pub struct CH1_LINK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Descriptor Link Structure Address Register"]
pub mod ch1_link;
#[doc = "Channel Peripheral Request Select Register"]
pub struct CH2_REQSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Peripheral Request Select Register"]
pub mod ch2_reqsel;
#[doc = "Channel Configuration Register"]
pub struct CH2_CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Configuration Register"]
pub mod ch2_cfg;
#[doc = "Channel Loop Counter Register"]
pub struct CH2_LOOP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Loop Counter Register"]
pub mod ch2_loop;
#[doc = "Channel Descriptor Control Word Register"]
pub struct CH2_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Descriptor Control Word Register"]
pub mod ch2_ctrl;
#[doc = "Channel Descriptor Source Data Address Register"]
pub struct CH2_SRC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Descriptor Source Data Address Register"]
pub mod ch2_src;
#[doc = "Channel Descriptor Destination Data Address Register"]
pub struct CH2_DST {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Descriptor Destination Data Address Register"]
pub mod ch2_dst;
#[doc = "Channel Descriptor Link Structure Address Register"]
pub struct CH2_LINK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Descriptor Link Structure Address Register"]
pub mod ch2_link;
#[doc = "Channel Peripheral Request Select Register"]
pub struct CH3_REQSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Peripheral Request Select Register"]
pub mod ch3_reqsel;
#[doc = "Channel Configuration Register"]
pub struct CH3_CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Configuration Register"]
pub mod ch3_cfg;
#[doc = "Channel Loop Counter Register"]
pub struct CH3_LOOP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Loop Counter Register"]
pub mod ch3_loop;
#[doc = "Channel Descriptor Control Word Register"]
pub struct CH3_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Descriptor Control Word Register"]
pub mod ch3_ctrl;
#[doc = "Channel Descriptor Source Data Address Register"]
pub struct CH3_SRC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Descriptor Source Data Address Register"]
pub mod ch3_src;
#[doc = "Channel Descriptor Destination Data Address Register"]
pub struct CH3_DST {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Descriptor Destination Data Address Register"]
pub mod ch3_dst;
#[doc = "Channel Descriptor Link Structure Address Register"]
pub struct CH3_LINK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Descriptor Link Structure Address Register"]
pub mod ch3_link;
#[doc = "Channel Peripheral Request Select Register"]
pub struct CH4_REQSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Peripheral Request Select Register"]
pub mod ch4_reqsel;
#[doc = "Channel Configuration Register"]
pub struct CH4_CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Configuration Register"]
pub mod ch4_cfg;
#[doc = "Channel Loop Counter Register"]
pub struct CH4_LOOP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Loop Counter Register"]
pub mod ch4_loop;
#[doc = "Channel Descriptor Control Word Register"]
pub struct CH4_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Descriptor Control Word Register"]
pub mod ch4_ctrl;
#[doc = "Channel Descriptor Source Data Address Register"]
pub struct CH4_SRC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Descriptor Source Data Address Register"]
pub mod ch4_src;
#[doc = "Channel Descriptor Destination Data Address Register"]
pub struct CH4_DST {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Descriptor Destination Data Address Register"]
pub mod ch4_dst;
#[doc = "Channel Descriptor Link Structure Address Register"]
pub struct CH4_LINK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Descriptor Link Structure Address Register"]
pub mod ch4_link;
#[doc = "Channel Peripheral Request Select Register"]
pub struct CH5_REQSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Peripheral Request Select Register"]
pub mod ch5_reqsel;
#[doc = "Channel Configuration Register"]
pub struct CH5_CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Configuration Register"]
pub mod ch5_cfg;
#[doc = "Channel Loop Counter Register"]
pub struct CH5_LOOP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Loop Counter Register"]
pub mod ch5_loop;
#[doc = "Channel Descriptor Control Word Register"]
pub struct CH5_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Descriptor Control Word Register"]
pub mod ch5_ctrl;
#[doc = "Channel Descriptor Source Data Address Register"]
pub struct CH5_SRC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Descriptor Source Data Address Register"]
pub mod ch5_src;
#[doc = "Channel Descriptor Destination Data Address Register"]
pub struct CH5_DST {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Descriptor Destination Data Address Register"]
pub mod ch5_dst;
#[doc = "Channel Descriptor Link Structure Address Register"]
pub struct CH5_LINK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Descriptor Link Structure Address Register"]
pub mod ch5_link;
#[doc = "Channel Peripheral Request Select Register"]
pub struct CH6_REQSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Peripheral Request Select Register"]
pub mod ch6_reqsel;
#[doc = "Channel Configuration Register"]
pub struct CH6_CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Configuration Register"]
pub mod ch6_cfg;
#[doc = "Channel Loop Counter Register"]
pub struct CH6_LOOP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Loop Counter Register"]
pub mod ch6_loop;
#[doc = "Channel Descriptor Control Word Register"]
pub struct CH6_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Descriptor Control Word Register"]
pub mod ch6_ctrl;
#[doc = "Channel Descriptor Source Data Address Register"]
pub struct CH6_SRC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Descriptor Source Data Address Register"]
pub mod ch6_src;
#[doc = "Channel Descriptor Destination Data Address Register"]
pub struct CH6_DST {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Descriptor Destination Data Address Register"]
pub mod ch6_dst;
#[doc = "Channel Descriptor Link Structure Address Register"]
pub struct CH6_LINK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Descriptor Link Structure Address Register"]
pub mod ch6_link;
#[doc = "Channel Peripheral Request Select Register"]
pub struct CH7_REQSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Peripheral Request Select Register"]
pub mod ch7_reqsel;
#[doc = "Channel Configuration Register"]
pub struct CH7_CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Configuration Register"]
pub mod ch7_cfg;
#[doc = "Channel Loop Counter Register"]
pub struct CH7_LOOP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Loop Counter Register"]
pub mod ch7_loop;
#[doc = "Channel Descriptor Control Word Register"]
pub struct CH7_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Descriptor Control Word Register"]
pub mod ch7_ctrl;
#[doc = "Channel Descriptor Source Data Address Register"]
pub struct CH7_SRC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Descriptor Source Data Address Register"]
pub mod ch7_src;
#[doc = "Channel Descriptor Destination Data Address Register"]
pub struct CH7_DST {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Descriptor Destination Data Address Register"]
pub mod ch7_dst;
#[doc = "Channel Descriptor Link Structure Address Register"]
pub struct CH7_LINK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Descriptor Link Structure Address Register"]
pub mod ch7_link;
