#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x04 - Status Register"]
    pub status: STATUS,
    #[doc = "0x08 - Configuration Lock Register"]
    pub lock: LOCK,
    #[doc = "0x0c - Memory Control Register"]
    pub ram0ctrl: RAM0CTRL,
    #[doc = "0x10 - Command Register"]
    pub cmd: CMD,
    _reserved0: [u8; 4usize],
    #[doc = "0x18 - EM4 Control Register"]
    pub em4ctrl: EM4CTRL,
    #[doc = "0x1c - Temperature Limits for Interrupt Generation"]
    pub templimits: TEMPLIMITS,
    #[doc = "0x20 - Value of Last Temperature Measurement"]
    pub temp: TEMP,
    #[doc = "0x24 - Interrupt Flag Register"]
    pub if_: IF,
    #[doc = "0x28 - Interrupt Flag Set Register"]
    pub ifs: IFS,
    #[doc = "0x2c - Interrupt Flag Clear Register"]
    pub ifc: IFC,
    #[doc = "0x30 - Interrupt Enable Register"]
    pub ien: IEN,
    #[doc = "0x34 - Regulator and Supply Lock Register"]
    pub pwrlock: PWRLOCK,
    #[doc = "0x38 - Power Configuration Register"]
    pub pwrcfg: PWRCFG,
    #[doc = "0x3c - Power Control Register"]
    pub pwrctrl: PWRCTRL,
    #[doc = "0x40 - DCDC Control"]
    pub dcdcctrl: DCDCCTRL,
    _reserved1: [u8; 8usize],
    #[doc = "0x4c - DCDC Miscellaneous Control Register"]
    pub dcdcmiscctrl: DCDCMISCCTRL,
    #[doc = "0x50 - DCDC Power Train NFET Zero Current Detector Control Register"]
    pub dcdczdetctrl: DCDCZDETCTRL,
    #[doc = "0x54 - DCDC Power Train PFET Current Limiter Control Register"]
    pub dcdcclimctrl: DCDCCLIMCTRL,
    #[doc = "0x58 - DCDC Low Noise Compensator Control Register"]
    pub dcdclncompctrl: DCDCLNCOMPCTRL,
    #[doc = "0x5c - DCDC Low Noise Voltage Register"]
    pub dcdclnvctrl: DCDCLNVCTRL,
    #[doc = "0x60 - DCDC Controller Timing Value Register"]
    pub dcdctiming: DCDCTIMING,
    #[doc = "0x64 - DCDC Low Power Voltage Register"]
    pub dcdclpvctrl: DCDCLPVCTRL,
    _reserved2: [u8; 4usize],
    #[doc = "0x6c - DCDC Low Power Control Register"]
    pub dcdclpctrl: DCDCLPCTRL,
    #[doc = "0x70 - DCDC Low Noise Controller Frequency Control"]
    pub dcdclnfreqctrl: DCDCLNFREQCTRL,
    _reserved3: [u8; 4usize],
    #[doc = "0x78 - DCDC Read Status Register"]
    pub dcdcsync: DCDCSYNC,
    _reserved4: [u8; 20usize],
    #[doc = "0x90 - VMON AVDD Channel Control"]
    pub vmonavddctrl: VMONAVDDCTRL,
    #[doc = "0x94 - Alternate VMON AVDD Channel Control"]
    pub vmonaltavddctrl: VMONALTAVDDCTRL,
    #[doc = "0x98 - VMON DVDD Channel Control"]
    pub vmondvddctrl: VMONDVDDCTRL,
    #[doc = "0x9c - VMON IOVDD0 Channel Control"]
    pub vmonio0ctrl: VMONIO0CTRL,
    _reserved5: [u8; 196usize],
    #[doc = "0x164 - Configurations Related to the Bias"]
    pub biasconf: BIASCONF,
    _reserved6: [u8; 40usize],
    #[doc = "0x190 - Test Lock Register"]
    pub testlock: TESTLOCK,
    _reserved7: [u8; 8usize],
    #[doc = "0x19c - Test Control Register for Regulator and BIAS"]
    pub biastestctrl: BIASTESTCTRL,
}
#[doc = "Control Register"]
pub struct CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "Status Register"]
pub struct STATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status Register"]
pub mod status;
#[doc = "Configuration Lock Register"]
pub struct LOCK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Configuration Lock Register"]
pub mod lock;
#[doc = "Memory Control Register"]
pub struct RAM0CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Memory Control Register"]
pub mod ram0ctrl;
#[doc = "Command Register"]
pub struct CMD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Command Register"]
pub mod cmd;
#[doc = "EM4 Control Register"]
pub struct EM4CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EM4 Control Register"]
pub mod em4ctrl;
#[doc = "Temperature Limits for Interrupt Generation"]
pub struct TEMPLIMITS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Temperature Limits for Interrupt Generation"]
pub mod templimits;
#[doc = "Value of Last Temperature Measurement"]
pub struct TEMP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of Last Temperature Measurement"]
pub mod temp;
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
#[doc = "Regulator and Supply Lock Register"]
pub struct PWRLOCK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Regulator and Supply Lock Register"]
pub mod pwrlock;
#[doc = "Power Configuration Register"]
pub struct PWRCFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Power Configuration Register"]
pub mod pwrcfg;
#[doc = "Power Control Register"]
pub struct PWRCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Power Control Register"]
pub mod pwrctrl;
#[doc = "DCDC Control"]
pub struct DCDCCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCDC Control"]
pub mod dcdcctrl;
#[doc = "DCDC Miscellaneous Control Register"]
pub struct DCDCMISCCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCDC Miscellaneous Control Register"]
pub mod dcdcmiscctrl;
#[doc = "DCDC Power Train NFET Zero Current Detector Control Register"]
pub struct DCDCZDETCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCDC Power Train NFET Zero Current Detector Control Register"]
pub mod dcdczdetctrl;
#[doc = "DCDC Power Train PFET Current Limiter Control Register"]
pub struct DCDCCLIMCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCDC Power Train PFET Current Limiter Control Register"]
pub mod dcdcclimctrl;
#[doc = "DCDC Low Noise Compensator Control Register"]
pub struct DCDCLNCOMPCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCDC Low Noise Compensator Control Register"]
pub mod dcdclncompctrl;
#[doc = "DCDC Low Noise Voltage Register"]
pub struct DCDCLNVCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCDC Low Noise Voltage Register"]
pub mod dcdclnvctrl;
#[doc = "DCDC Controller Timing Value Register"]
pub struct DCDCTIMING {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCDC Controller Timing Value Register"]
pub mod dcdctiming;
#[doc = "DCDC Low Power Voltage Register"]
pub struct DCDCLPVCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCDC Low Power Voltage Register"]
pub mod dcdclpvctrl;
#[doc = "DCDC Low Power Control Register"]
pub struct DCDCLPCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCDC Low Power Control Register"]
pub mod dcdclpctrl;
#[doc = "DCDC Low Noise Controller Frequency Control"]
pub struct DCDCLNFREQCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCDC Low Noise Controller Frequency Control"]
pub mod dcdclnfreqctrl;
#[doc = "DCDC Read Status Register"]
pub struct DCDCSYNC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCDC Read Status Register"]
pub mod dcdcsync;
#[doc = "VMON AVDD Channel Control"]
pub struct VMONAVDDCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "VMON AVDD Channel Control"]
pub mod vmonavddctrl;
#[doc = "Alternate VMON AVDD Channel Control"]
pub struct VMONALTAVDDCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Alternate VMON AVDD Channel Control"]
pub mod vmonaltavddctrl;
#[doc = "VMON DVDD Channel Control"]
pub struct VMONDVDDCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "VMON DVDD Channel Control"]
pub mod vmondvddctrl;
#[doc = "VMON IOVDD0 Channel Control"]
pub struct VMONIO0CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "VMON IOVDD0 Channel Control"]
pub mod vmonio0ctrl;
#[doc = "Configurations Related to the Bias"]
pub struct BIASCONF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Configurations Related to the Bias"]
pub mod biasconf;
#[doc = "Test Lock Register"]
pub struct TESTLOCK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Test Lock Register"]
pub mod testlock;
#[doc = "Test Control Register for Regulator and BIAS"]
pub struct BIASTESTCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Test Control Register for Regulator and BIAS"]
pub mod biastestctrl;
