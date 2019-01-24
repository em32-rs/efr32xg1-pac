#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Port Control Register"]
    pub pa_ctrl: PA_CTRL,
    #[doc = "0x04 - Port Pin Mode Low Register"]
    pub pa_model: PA_MODEL,
    #[doc = "0x08 - Port Pin Mode High Register"]
    pub pa_modeh: PA_MODEH,
    #[doc = "0x0c - Port Data Out Register"]
    pub pa_dout: PA_DOUT,
    _reserved0: [u8; 8usize],
    #[doc = "0x18 - Port Data Out Toggle Register"]
    pub pa_douttgl: PA_DOUTTGL,
    #[doc = "0x1c - Port Data in Register"]
    pub pa_din: PA_DIN,
    #[doc = "0x20 - Port Unlocked Pins Register"]
    pub pa_pinlockn: PA_PINLOCKN,
    _reserved1: [u8; 4usize],
    #[doc = "0x28 - Over Voltage Disable for All Modes"]
    pub pa_ovtdis: PA_OVTDIS,
    _reserved2: [u8; 4usize],
    #[doc = "0x30 - Port Control Register"]
    pub pb_ctrl: PB_CTRL,
    #[doc = "0x34 - Port Pin Mode Low Register"]
    pub pb_model: PB_MODEL,
    #[doc = "0x38 - Port Pin Mode High Register"]
    pub pb_modeh: PB_MODEH,
    #[doc = "0x3c - Port Data Out Register"]
    pub pb_dout: PB_DOUT,
    _reserved3: [u8; 8usize],
    #[doc = "0x48 - Port Data Out Toggle Register"]
    pub pb_douttgl: PB_DOUTTGL,
    #[doc = "0x4c - Port Data in Register"]
    pub pb_din: PB_DIN,
    #[doc = "0x50 - Port Unlocked Pins Register"]
    pub pb_pinlockn: PB_PINLOCKN,
    _reserved4: [u8; 4usize],
    #[doc = "0x58 - Over Voltage Disable for All Modes"]
    pub pb_ovtdis: PB_OVTDIS,
    _reserved5: [u8; 4usize],
    #[doc = "0x60 - Port Control Register"]
    pub pc_ctrl: PC_CTRL,
    #[doc = "0x64 - Port Pin Mode Low Register"]
    pub pc_model: PC_MODEL,
    #[doc = "0x68 - Port Pin Mode High Register"]
    pub pc_modeh: PC_MODEH,
    #[doc = "0x6c - Port Data Out Register"]
    pub pc_dout: PC_DOUT,
    _reserved6: [u8; 8usize],
    #[doc = "0x78 - Port Data Out Toggle Register"]
    pub pc_douttgl: PC_DOUTTGL,
    #[doc = "0x7c - Port Data in Register"]
    pub pc_din: PC_DIN,
    #[doc = "0x80 - Port Unlocked Pins Register"]
    pub pc_pinlockn: PC_PINLOCKN,
    _reserved7: [u8; 4usize],
    #[doc = "0x88 - Over Voltage Disable for All Modes"]
    pub pc_ovtdis: PC_OVTDIS,
    _reserved8: [u8; 4usize],
    #[doc = "0x90 - Port Control Register"]
    pub pd_ctrl: PD_CTRL,
    #[doc = "0x94 - Port Pin Mode Low Register"]
    pub pd_model: PD_MODEL,
    #[doc = "0x98 - Port Pin Mode High Register"]
    pub pd_modeh: PD_MODEH,
    #[doc = "0x9c - Port Data Out Register"]
    pub pd_dout: PD_DOUT,
    _reserved9: [u8; 8usize],
    #[doc = "0xa8 - Port Data Out Toggle Register"]
    pub pd_douttgl: PD_DOUTTGL,
    #[doc = "0xac - Port Data in Register"]
    pub pd_din: PD_DIN,
    #[doc = "0xb0 - Port Unlocked Pins Register"]
    pub pd_pinlockn: PD_PINLOCKN,
    _reserved10: [u8; 4usize],
    #[doc = "0xb8 - Over Voltage Disable for All Modes"]
    pub pd_ovtdis: PD_OVTDIS,
    _reserved11: [u8; 4usize],
    #[doc = "0xc0 - Port Control Register"]
    pub pe_ctrl: PE_CTRL,
    #[doc = "0xc4 - Port Pin Mode Low Register"]
    pub pe_model: PE_MODEL,
    #[doc = "0xc8 - Port Pin Mode High Register"]
    pub pe_modeh: PE_MODEH,
    #[doc = "0xcc - Port Data Out Register"]
    pub pe_dout: PE_DOUT,
    _reserved12: [u8; 8usize],
    #[doc = "0xd8 - Port Data Out Toggle Register"]
    pub pe_douttgl: PE_DOUTTGL,
    #[doc = "0xdc - Port Data in Register"]
    pub pe_din: PE_DIN,
    #[doc = "0xe0 - Port Unlocked Pins Register"]
    pub pe_pinlockn: PE_PINLOCKN,
    _reserved13: [u8; 4usize],
    #[doc = "0xe8 - Over Voltage Disable for All Modes"]
    pub pe_ovtdis: PE_OVTDIS,
    _reserved14: [u8; 4usize],
    #[doc = "0xf0 - Port Control Register"]
    pub pf_ctrl: PF_CTRL,
    #[doc = "0xf4 - Port Pin Mode Low Register"]
    pub pf_model: PF_MODEL,
    #[doc = "0xf8 - Port Pin Mode High Register"]
    pub pf_modeh: PF_MODEH,
    #[doc = "0xfc - Port Data Out Register"]
    pub pf_dout: PF_DOUT,
    _reserved15: [u8; 8usize],
    #[doc = "0x108 - Port Data Out Toggle Register"]
    pub pf_douttgl: PF_DOUTTGL,
    #[doc = "0x10c - Port Data in Register"]
    pub pf_din: PF_DIN,
    #[doc = "0x110 - Port Unlocked Pins Register"]
    pub pf_pinlockn: PF_PINLOCKN,
    _reserved16: [u8; 4usize],
    #[doc = "0x118 - Over Voltage Disable for All Modes"]
    pub pf_ovtdis: PF_OVTDIS,
    _reserved17: [u8; 740usize],
    #[doc = "0x400 - External Interrupt Port Select Low Register"]
    pub extipsell: EXTIPSELL,
    #[doc = "0x404 - External Interrupt Port Select High Register"]
    pub extipselh: EXTIPSELH,
    #[doc = "0x408 - External Interrupt Pin Select Low Register"]
    pub extipinsell: EXTIPINSELL,
    #[doc = "0x40c - External Interrupt Pin Select High Register"]
    pub extipinselh: EXTIPINSELH,
    #[doc = "0x410 - External Interrupt Rising Edge Trigger Register"]
    pub extirise: EXTIRISE,
    #[doc = "0x414 - External Interrupt Falling Edge Trigger Register"]
    pub extifall: EXTIFALL,
    #[doc = "0x418 - External Interrupt Level Register"]
    pub extilevel: EXTILEVEL,
    #[doc = "0x41c - Interrupt Flag Register"]
    pub if_: IF,
    #[doc = "0x420 - Interrupt Flag Set Register"]
    pub ifs: IFS,
    #[doc = "0x424 - Interrupt Flag Clear Register"]
    pub ifc: IFC,
    #[doc = "0x428 - Interrupt Enable Register"]
    pub ien: IEN,
    #[doc = "0x42c - EM4 Wake Up Enable Register"]
    pub em4wuen: EM4WUEN,
    _reserved18: [u8; 16usize],
    #[doc = "0x440 - I/O Routing Pin Enable Register"]
    pub routepen: ROUTEPEN,
    #[doc = "0x444 - I/O Routing Location Register"]
    pub routeloc0: ROUTELOC0,
    _reserved19: [u8; 8usize],
    #[doc = "0x450 - Input Sense Register"]
    pub insense: INSENSE,
    #[doc = "0x454 - Configuration Lock Register"]
    pub lock: LOCK,
}
#[doc = "Port Control Register"]
pub struct PA_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port Control Register"]
pub mod pa_ctrl;
#[doc = "Port Pin Mode Low Register"]
pub struct PA_MODEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port Pin Mode Low Register"]
pub mod pa_model;
#[doc = "Port Pin Mode High Register"]
pub struct PA_MODEH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port Pin Mode High Register"]
pub mod pa_modeh;
#[doc = "Port Data Out Register"]
pub struct PA_DOUT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port Data Out Register"]
pub mod pa_dout;
#[doc = "Port Data Out Toggle Register"]
pub struct PA_DOUTTGL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port Data Out Toggle Register"]
pub mod pa_douttgl;
#[doc = "Port Data in Register"]
pub struct PA_DIN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port Data in Register"]
pub mod pa_din;
#[doc = "Port Unlocked Pins Register"]
pub struct PA_PINLOCKN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port Unlocked Pins Register"]
pub mod pa_pinlockn;
#[doc = "Over Voltage Disable for All Modes"]
pub struct PA_OVTDIS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Over Voltage Disable for All Modes"]
pub mod pa_ovtdis;
#[doc = "Port Control Register"]
pub struct PB_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port Control Register"]
pub mod pb_ctrl;
#[doc = "Port Pin Mode Low Register"]
pub struct PB_MODEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port Pin Mode Low Register"]
pub mod pb_model;
#[doc = "Port Pin Mode High Register"]
pub struct PB_MODEH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port Pin Mode High Register"]
pub mod pb_modeh;
#[doc = "Port Data Out Register"]
pub struct PB_DOUT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port Data Out Register"]
pub mod pb_dout;
#[doc = "Port Data Out Toggle Register"]
pub struct PB_DOUTTGL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port Data Out Toggle Register"]
pub mod pb_douttgl;
#[doc = "Port Data in Register"]
pub struct PB_DIN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port Data in Register"]
pub mod pb_din;
#[doc = "Port Unlocked Pins Register"]
pub struct PB_PINLOCKN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port Unlocked Pins Register"]
pub mod pb_pinlockn;
#[doc = "Over Voltage Disable for All Modes"]
pub struct PB_OVTDIS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Over Voltage Disable for All Modes"]
pub mod pb_ovtdis;
#[doc = "Port Control Register"]
pub struct PC_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port Control Register"]
pub mod pc_ctrl;
#[doc = "Port Pin Mode Low Register"]
pub struct PC_MODEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port Pin Mode Low Register"]
pub mod pc_model;
#[doc = "Port Pin Mode High Register"]
pub struct PC_MODEH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port Pin Mode High Register"]
pub mod pc_modeh;
#[doc = "Port Data Out Register"]
pub struct PC_DOUT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port Data Out Register"]
pub mod pc_dout;
#[doc = "Port Data Out Toggle Register"]
pub struct PC_DOUTTGL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port Data Out Toggle Register"]
pub mod pc_douttgl;
#[doc = "Port Data in Register"]
pub struct PC_DIN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port Data in Register"]
pub mod pc_din;
#[doc = "Port Unlocked Pins Register"]
pub struct PC_PINLOCKN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port Unlocked Pins Register"]
pub mod pc_pinlockn;
#[doc = "Over Voltage Disable for All Modes"]
pub struct PC_OVTDIS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Over Voltage Disable for All Modes"]
pub mod pc_ovtdis;
#[doc = "Port Control Register"]
pub struct PD_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port Control Register"]
pub mod pd_ctrl;
#[doc = "Port Pin Mode Low Register"]
pub struct PD_MODEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port Pin Mode Low Register"]
pub mod pd_model;
#[doc = "Port Pin Mode High Register"]
pub struct PD_MODEH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port Pin Mode High Register"]
pub mod pd_modeh;
#[doc = "Port Data Out Register"]
pub struct PD_DOUT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port Data Out Register"]
pub mod pd_dout;
#[doc = "Port Data Out Toggle Register"]
pub struct PD_DOUTTGL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port Data Out Toggle Register"]
pub mod pd_douttgl;
#[doc = "Port Data in Register"]
pub struct PD_DIN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port Data in Register"]
pub mod pd_din;
#[doc = "Port Unlocked Pins Register"]
pub struct PD_PINLOCKN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port Unlocked Pins Register"]
pub mod pd_pinlockn;
#[doc = "Over Voltage Disable for All Modes"]
pub struct PD_OVTDIS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Over Voltage Disable for All Modes"]
pub mod pd_ovtdis;
#[doc = "Port Control Register"]
pub struct PE_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port Control Register"]
pub mod pe_ctrl;
#[doc = "Port Pin Mode Low Register"]
pub struct PE_MODEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port Pin Mode Low Register"]
pub mod pe_model;
#[doc = "Port Pin Mode High Register"]
pub struct PE_MODEH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port Pin Mode High Register"]
pub mod pe_modeh;
#[doc = "Port Data Out Register"]
pub struct PE_DOUT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port Data Out Register"]
pub mod pe_dout;
#[doc = "Port Data Out Toggle Register"]
pub struct PE_DOUTTGL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port Data Out Toggle Register"]
pub mod pe_douttgl;
#[doc = "Port Data in Register"]
pub struct PE_DIN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port Data in Register"]
pub mod pe_din;
#[doc = "Port Unlocked Pins Register"]
pub struct PE_PINLOCKN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port Unlocked Pins Register"]
pub mod pe_pinlockn;
#[doc = "Over Voltage Disable for All Modes"]
pub struct PE_OVTDIS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Over Voltage Disable for All Modes"]
pub mod pe_ovtdis;
#[doc = "Port Control Register"]
pub struct PF_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port Control Register"]
pub mod pf_ctrl;
#[doc = "Port Pin Mode Low Register"]
pub struct PF_MODEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port Pin Mode Low Register"]
pub mod pf_model;
#[doc = "Port Pin Mode High Register"]
pub struct PF_MODEH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port Pin Mode High Register"]
pub mod pf_modeh;
#[doc = "Port Data Out Register"]
pub struct PF_DOUT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port Data Out Register"]
pub mod pf_dout;
#[doc = "Port Data Out Toggle Register"]
pub struct PF_DOUTTGL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port Data Out Toggle Register"]
pub mod pf_douttgl;
#[doc = "Port Data in Register"]
pub struct PF_DIN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port Data in Register"]
pub mod pf_din;
#[doc = "Port Unlocked Pins Register"]
pub struct PF_PINLOCKN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port Unlocked Pins Register"]
pub mod pf_pinlockn;
#[doc = "Over Voltage Disable for All Modes"]
pub struct PF_OVTDIS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Over Voltage Disable for All Modes"]
pub mod pf_ovtdis;
#[doc = "External Interrupt Port Select Low Register"]
pub struct EXTIPSELL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "External Interrupt Port Select Low Register"]
pub mod extipsell;
#[doc = "External Interrupt Port Select High Register"]
pub struct EXTIPSELH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "External Interrupt Port Select High Register"]
pub mod extipselh;
#[doc = "External Interrupt Pin Select Low Register"]
pub struct EXTIPINSELL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "External Interrupt Pin Select Low Register"]
pub mod extipinsell;
#[doc = "External Interrupt Pin Select High Register"]
pub struct EXTIPINSELH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "External Interrupt Pin Select High Register"]
pub mod extipinselh;
#[doc = "External Interrupt Rising Edge Trigger Register"]
pub struct EXTIRISE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "External Interrupt Rising Edge Trigger Register"]
pub mod extirise;
#[doc = "External Interrupt Falling Edge Trigger Register"]
pub struct EXTIFALL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "External Interrupt Falling Edge Trigger Register"]
pub mod extifall;
#[doc = "External Interrupt Level Register"]
pub struct EXTILEVEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "External Interrupt Level Register"]
pub mod extilevel;
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
#[doc = "EM4 Wake Up Enable Register"]
pub struct EM4WUEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EM4 Wake Up Enable Register"]
pub mod em4wuen;
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
#[doc = "Input Sense Register"]
pub struct INSENSE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Input Sense Register"]
pub mod insense;
#[doc = "Configuration Lock Register"]
pub struct LOCK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Configuration Lock Register"]
pub mod lock;
