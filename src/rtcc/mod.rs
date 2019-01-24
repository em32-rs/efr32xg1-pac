#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x04 - Pre-Counter Value Register"]
    pub precnt: PRECNT,
    #[doc = "0x08 - Counter Value Register"]
    pub cnt: CNT,
    #[doc = "0x0c - Combined Pre-Counter and Counter Value Register"]
    pub combcnt: COMBCNT,
    #[doc = "0x10 - Time of Day Register"]
    pub time: TIME,
    #[doc = "0x14 - Date Register"]
    pub date: DATE,
    #[doc = "0x18 - RTCC Interrupt Flags"]
    pub if_: IF,
    #[doc = "0x1c - Interrupt Flag Set Register"]
    pub ifs: IFS,
    #[doc = "0x20 - Interrupt Flag Clear Register"]
    pub ifc: IFC,
    #[doc = "0x24 - Interrupt Enable Register"]
    pub ien: IEN,
    #[doc = "0x28 - Status Register"]
    pub status: STATUS,
    #[doc = "0x2c - Command Register"]
    pub cmd: CMD,
    #[doc = "0x30 - Synchronization Busy Register"]
    pub syncbusy: SYNCBUSY,
    #[doc = "0x34 - Retention RAM Power-down Register"]
    pub powerdown: POWERDOWN,
    #[doc = "0x38 - Configuration Lock Register"]
    pub lock: LOCK,
    #[doc = "0x3c - Wake Up Enable"]
    pub em4wuen: EM4WUEN,
    #[doc = "0x40 - CC Channel Control Register"]
    pub cc0_ctrl: CC0_CTRL,
    #[doc = "0x44 - Capture/Compare Value Register"]
    pub cc0_ccv: CC0_CCV,
    #[doc = "0x48 - Capture/Compare Time Register"]
    pub cc0_time: CC0_TIME,
    #[doc = "0x4c - Capture/Compare Date Register"]
    pub cc0_date: CC0_DATE,
    #[doc = "0x50 - CC Channel Control Register"]
    pub cc1_ctrl: CC1_CTRL,
    #[doc = "0x54 - Capture/Compare Value Register"]
    pub cc1_ccv: CC1_CCV,
    #[doc = "0x58 - Capture/Compare Time Register"]
    pub cc1_time: CC1_TIME,
    #[doc = "0x5c - Capture/Compare Date Register"]
    pub cc1_date: CC1_DATE,
    #[doc = "0x60 - CC Channel Control Register"]
    pub cc2_ctrl: CC2_CTRL,
    #[doc = "0x64 - Capture/Compare Value Register"]
    pub cc2_ccv: CC2_CCV,
    #[doc = "0x68 - Capture/Compare Time Register"]
    pub cc2_time: CC2_TIME,
    #[doc = "0x6c - Capture/Compare Date Register"]
    pub cc2_date: CC2_DATE,
    _reserved0: [u8; 148usize],
    #[doc = "0x104 - Retention Register"]
    pub ret0_reg: RET0_REG,
    #[doc = "0x108 - Retention Register"]
    pub ret1_reg: RET1_REG,
    #[doc = "0x10c - Retention Register"]
    pub ret2_reg: RET2_REG,
    #[doc = "0x110 - Retention Register"]
    pub ret3_reg: RET3_REG,
    #[doc = "0x114 - Retention Register"]
    pub ret4_reg: RET4_REG,
    #[doc = "0x118 - Retention Register"]
    pub ret5_reg: RET5_REG,
    #[doc = "0x11c - Retention Register"]
    pub ret6_reg: RET6_REG,
    #[doc = "0x120 - Retention Register"]
    pub ret7_reg: RET7_REG,
    #[doc = "0x124 - Retention Register"]
    pub ret8_reg: RET8_REG,
    #[doc = "0x128 - Retention Register"]
    pub ret9_reg: RET9_REG,
    #[doc = "0x12c - Retention Register"]
    pub ret10_reg: RET10_REG,
    #[doc = "0x130 - Retention Register"]
    pub ret11_reg: RET11_REG,
    #[doc = "0x134 - Retention Register"]
    pub ret12_reg: RET12_REG,
    #[doc = "0x138 - Retention Register"]
    pub ret13_reg: RET13_REG,
    #[doc = "0x13c - Retention Register"]
    pub ret14_reg: RET14_REG,
    #[doc = "0x140 - Retention Register"]
    pub ret15_reg: RET15_REG,
    #[doc = "0x144 - Retention Register"]
    pub ret16_reg: RET16_REG,
    #[doc = "0x148 - Retention Register"]
    pub ret17_reg: RET17_REG,
    #[doc = "0x14c - Retention Register"]
    pub ret18_reg: RET18_REG,
    #[doc = "0x150 - Retention Register"]
    pub ret19_reg: RET19_REG,
    #[doc = "0x154 - Retention Register"]
    pub ret20_reg: RET20_REG,
    #[doc = "0x158 - Retention Register"]
    pub ret21_reg: RET21_REG,
    #[doc = "0x15c - Retention Register"]
    pub ret22_reg: RET22_REG,
    #[doc = "0x160 - Retention Register"]
    pub ret23_reg: RET23_REG,
    #[doc = "0x164 - Retention Register"]
    pub ret24_reg: RET24_REG,
    #[doc = "0x168 - Retention Register"]
    pub ret25_reg: RET25_REG,
    #[doc = "0x16c - Retention Register"]
    pub ret26_reg: RET26_REG,
    #[doc = "0x170 - Retention Register"]
    pub ret27_reg: RET27_REG,
    #[doc = "0x174 - Retention Register"]
    pub ret28_reg: RET28_REG,
    #[doc = "0x178 - Retention Register"]
    pub ret29_reg: RET29_REG,
    #[doc = "0x17c - Retention Register"]
    pub ret30_reg: RET30_REG,
    #[doc = "0x180 - Retention Register"]
    pub ret31_reg: RET31_REG,
}
#[doc = "Control Register"]
pub struct CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "Pre-Counter Value Register"]
pub struct PRECNT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre-Counter Value Register"]
pub mod precnt;
#[doc = "Counter Value Register"]
pub struct CNT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Counter Value Register"]
pub mod cnt;
#[doc = "Combined Pre-Counter and Counter Value Register"]
pub struct COMBCNT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Combined Pre-Counter and Counter Value Register"]
pub mod combcnt;
#[doc = "Time of Day Register"]
pub struct TIME {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Time of Day Register"]
pub mod time;
#[doc = "Date Register"]
pub struct DATE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Date Register"]
pub mod date;
#[doc = "RTCC Interrupt Flags"]
pub struct IF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTCC Interrupt Flags"]
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
#[doc = "Status Register"]
pub struct STATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status Register"]
pub mod status;
#[doc = "Command Register"]
pub struct CMD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Command Register"]
pub mod cmd;
#[doc = "Synchronization Busy Register"]
pub struct SYNCBUSY {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Synchronization Busy Register"]
pub mod syncbusy;
#[doc = "Retention RAM Power-down Register"]
pub struct POWERDOWN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Retention RAM Power-down Register"]
pub mod powerdown;
#[doc = "Configuration Lock Register"]
pub struct LOCK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Configuration Lock Register"]
pub mod lock;
#[doc = "Wake Up Enable"]
pub struct EM4WUEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Wake Up Enable"]
pub mod em4wuen;
#[doc = "CC Channel Control Register"]
pub struct CC0_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CC Channel Control Register"]
pub mod cc0_ctrl;
#[doc = "Capture/Compare Value Register"]
pub struct CC0_CCV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Capture/Compare Value Register"]
pub mod cc0_ccv;
#[doc = "Capture/Compare Time Register"]
pub struct CC0_TIME {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Capture/Compare Time Register"]
pub mod cc0_time;
#[doc = "Capture/Compare Date Register"]
pub struct CC0_DATE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Capture/Compare Date Register"]
pub mod cc0_date;
#[doc = "CC Channel Control Register"]
pub struct CC1_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CC Channel Control Register"]
pub mod cc1_ctrl;
#[doc = "Capture/Compare Value Register"]
pub struct CC1_CCV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Capture/Compare Value Register"]
pub mod cc1_ccv;
#[doc = "Capture/Compare Time Register"]
pub struct CC1_TIME {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Capture/Compare Time Register"]
pub mod cc1_time;
#[doc = "Capture/Compare Date Register"]
pub struct CC1_DATE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Capture/Compare Date Register"]
pub mod cc1_date;
#[doc = "CC Channel Control Register"]
pub struct CC2_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CC Channel Control Register"]
pub mod cc2_ctrl;
#[doc = "Capture/Compare Value Register"]
pub struct CC2_CCV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Capture/Compare Value Register"]
pub mod cc2_ccv;
#[doc = "Capture/Compare Time Register"]
pub struct CC2_TIME {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Capture/Compare Time Register"]
pub mod cc2_time;
#[doc = "Capture/Compare Date Register"]
pub struct CC2_DATE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Capture/Compare Date Register"]
pub mod cc2_date;
#[doc = "Retention Register"]
pub struct RET0_REG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Retention Register"]
pub mod ret0_reg;
#[doc = "Retention Register"]
pub struct RET1_REG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Retention Register"]
pub mod ret1_reg;
#[doc = "Retention Register"]
pub struct RET2_REG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Retention Register"]
pub mod ret2_reg;
#[doc = "Retention Register"]
pub struct RET3_REG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Retention Register"]
pub mod ret3_reg;
#[doc = "Retention Register"]
pub struct RET4_REG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Retention Register"]
pub mod ret4_reg;
#[doc = "Retention Register"]
pub struct RET5_REG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Retention Register"]
pub mod ret5_reg;
#[doc = "Retention Register"]
pub struct RET6_REG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Retention Register"]
pub mod ret6_reg;
#[doc = "Retention Register"]
pub struct RET7_REG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Retention Register"]
pub mod ret7_reg;
#[doc = "Retention Register"]
pub struct RET8_REG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Retention Register"]
pub mod ret8_reg;
#[doc = "Retention Register"]
pub struct RET9_REG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Retention Register"]
pub mod ret9_reg;
#[doc = "Retention Register"]
pub struct RET10_REG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Retention Register"]
pub mod ret10_reg;
#[doc = "Retention Register"]
pub struct RET11_REG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Retention Register"]
pub mod ret11_reg;
#[doc = "Retention Register"]
pub struct RET12_REG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Retention Register"]
pub mod ret12_reg;
#[doc = "Retention Register"]
pub struct RET13_REG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Retention Register"]
pub mod ret13_reg;
#[doc = "Retention Register"]
pub struct RET14_REG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Retention Register"]
pub mod ret14_reg;
#[doc = "Retention Register"]
pub struct RET15_REG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Retention Register"]
pub mod ret15_reg;
#[doc = "Retention Register"]
pub struct RET16_REG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Retention Register"]
pub mod ret16_reg;
#[doc = "Retention Register"]
pub struct RET17_REG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Retention Register"]
pub mod ret17_reg;
#[doc = "Retention Register"]
pub struct RET18_REG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Retention Register"]
pub mod ret18_reg;
#[doc = "Retention Register"]
pub struct RET19_REG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Retention Register"]
pub mod ret19_reg;
#[doc = "Retention Register"]
pub struct RET20_REG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Retention Register"]
pub mod ret20_reg;
#[doc = "Retention Register"]
pub struct RET21_REG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Retention Register"]
pub mod ret21_reg;
#[doc = "Retention Register"]
pub struct RET22_REG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Retention Register"]
pub mod ret22_reg;
#[doc = "Retention Register"]
pub struct RET23_REG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Retention Register"]
pub mod ret23_reg;
#[doc = "Retention Register"]
pub struct RET24_REG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Retention Register"]
pub mod ret24_reg;
#[doc = "Retention Register"]
pub struct RET25_REG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Retention Register"]
pub mod ret25_reg;
#[doc = "Retention Register"]
pub struct RET26_REG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Retention Register"]
pub mod ret26_reg;
#[doc = "Retention Register"]
pub struct RET27_REG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Retention Register"]
pub mod ret27_reg;
#[doc = "Retention Register"]
pub struct RET28_REG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Retention Register"]
pub mod ret28_reg;
#[doc = "Retention Register"]
pub struct RET29_REG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Retention Register"]
pub mod ret29_reg;
#[doc = "Retention Register"]
pub struct RET30_REG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Retention Register"]
pub mod ret30_reg;
#[doc = "Retention Register"]
pub struct RET31_REG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Retention Register"]
pub mod ret31_reg;
