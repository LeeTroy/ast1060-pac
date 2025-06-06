#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    udma000: Udma000,
    udma004: Udma004,
    udma008: Udma008,
    udma00c: Udma00c,
    _reserved4: [u8; 0x10],
    udma020: Udma020,
    udma024: Udma024,
    _reserved6: [u8; 0x08],
    udma30: Udma30,
    udma034: Udma034,
    udma038: Udma038,
    udma03c: Udma03c,
    udma040: Udma040,
    udma044: Udma044,
    udma048: Udma048,
    udma0c4: Udma0c4,
    udma050: Udma050,
    udma054: Udma054,
    udma058: Udma058,
    _reserved17: [u8; 0x04],
    udma060: Udma060,
    udma064: Udma064,
    udma068: Udma068,
    _reserved20: [u8; 0x04],
    udma070: Udma070,
    udma074: Udma074,
    udma078: Udma078,
    _reserved23: [u8; 0x04],
    udma080: Udma080,
    udma084: Udma084,
    udma088: Udma088,
    _reserved26: [u8; 0x04],
    udma090: Udma090,
    udma094: Udma094,
    udma098: Udma098,
    _reserved29: [u8; 0x04],
    udma0a0: Udma0a0,
    udma0a4: Udma0a4,
    udma0a8: Udma0a8,
    _reserved32: [u8; 0x04],
    udma0b0: Udma0b0,
    udma0b4: Udma0b4,
    udma0b8: Udma0b8,
    _reserved35: [u8; 0x04],
    udma0c0: Udma0c0,
    _reserved36: [u8; 0x04],
    udma0c8: Udma0c8,
    _reserved37: [u8; 0x04],
    udma0d0: Udma0d0,
    udma0d4: Udma0d4,
    udma0d8: Udma0d8,
    _reserved40: [u8; 0x04],
    udma0e0: Udma0e0,
    udma0e4: Udma0e4,
    udma0e8: Udma0e8,
    _reserved43: [u8; 0x04],
    udma0f0: Udma0f0,
    udma0f4: Udma0f4,
    udma0f8: Udma0f8,
    _reserved46: [u8; 0x04],
    udma100: Udma100,
    udma104: Udma104,
    udma108: Udma108,
    _reserved49: [u8; 0x04],
    udma110: Udma110,
    udma114: Udma114,
    udma118: Udma118,
    _reserved52: [u8; 0x04],
    udma120: Udma120,
    udma124: Udma124,
    udma128: Udma128,
    _reserved55: [u8; 0x04],
    udma130: Udma130,
    udma134: Udma134,
    udma138: Udma138,
    _reserved58: [u8; 0x04],
    udma140: Udma140,
    udma144: Udma144,
    udma148: Udma148,
    _reserved61: [u8; 0x04],
    udma150: Udma150,
    udma154: Udma154,
    udma158: Udma158,
    _reserved64: [u8; 0x04],
    udma160: Udma160,
    udma164: Udma164,
    udma168: Udma168,
    _reserved67: [u8; 0x04],
    udma170: Udma170,
    udma174: Udma174,
    udma178: Udma178,
    _reserved70: [u8; 0x04],
    udma180: Udma180,
    udma184: Udma184,
    udma188: Udma188,
    _reserved73: [u8; 0x04],
    udma190: Udma190,
    udma194: Udma194,
    udma198: Udma198,
    _reserved76: [u8; 0x04],
    udma1a0: Udma1a0,
    udma1a4: Udma1a4,
    udma1a8: Udma1a8,
    _reserved79: [u8; 0x04],
    udma1b0: Udma1b0,
    udma1b4: Udma1b4,
    udma1b8: Udma1b8,
}
impl RegisterBlock {
    #[doc = "0x00 - UART TX DMA enable"]
    #[inline(always)]
    pub const fn udma000(&self) -> &Udma000 {
        &self.udma000
    }
    #[doc = "0x04 - UART RX DMA enable"]
    #[inline(always)]
    pub const fn udma004(&self) -> &Udma004 {
        &self.udma004
    }
    #[doc = "0x08 - Misc, buffer size"]
    #[inline(always)]
    pub const fn udma008(&self) -> &Udma008 {
        &self.udma008
    }
    #[doc = "0x0c - UART DMA time out timer"]
    #[inline(always)]
    pub const fn udma00c(&self) -> &Udma00c {
        &self.udma00c
    }
    #[doc = "0x20 - UART TX DMA reset"]
    #[inline(always)]
    pub const fn udma020(&self) -> &Udma020 {
        &self.udma020
    }
    #[doc = "0x24 - UART RX DMA reset"]
    #[inline(always)]
    pub const fn udma024(&self) -> &Udma024 {
        &self.udma024
    }
    #[doc = "0x30 - UART TX DMA interrupt enable"]
    #[inline(always)]
    pub const fn udma30(&self) -> &Udma30 {
        &self.udma30
    }
    #[doc = "0x34 - UART TX DMA interrupt status"]
    #[inline(always)]
    pub const fn udma034(&self) -> &Udma034 {
        &self.udma034
    }
    #[doc = "0x38 - UART RX DMA interrupt enable"]
    #[inline(always)]
    pub const fn udma038(&self) -> &Udma038 {
        &self.udma038
    }
    #[doc = "0x3c - UART RX DMA interrupt status"]
    #[inline(always)]
    pub const fn udma03c(&self) -> &Udma03c {
        &self.udma03c
    }
    #[doc = "0x40 - UART1 TX read pointer"]
    #[inline(always)]
    pub const fn udma040(&self) -> &Udma040 {
        &self.udma040
    }
    #[doc = "0x44 - UART1 TX write pointer"]
    #[inline(always)]
    pub const fn udma044(&self) -> &Udma044 {
        &self.udma044
    }
    #[doc = "0x48 - UART1 TX buffer base address"]
    #[inline(always)]
    pub const fn udma048(&self) -> &Udma048 {
        &self.udma048
    }
    #[doc = "0x4c - UART5 TX write pointer"]
    #[inline(always)]
    pub const fn udma0c4(&self) -> &Udma0c4 {
        &self.udma0c4
    }
    #[doc = "0x50 - UART1 RX read pointer"]
    #[inline(always)]
    pub const fn udma050(&self) -> &Udma050 {
        &self.udma050
    }
    #[doc = "0x54 - UART1 RX write pointer"]
    #[inline(always)]
    pub const fn udma054(&self) -> &Udma054 {
        &self.udma054
    }
    #[doc = "0x58 - UART1 RX buffer base address"]
    #[inline(always)]
    pub const fn udma058(&self) -> &Udma058 {
        &self.udma058
    }
    #[doc = "0x60 - UART2 TX read pointer"]
    #[inline(always)]
    pub const fn udma060(&self) -> &Udma060 {
        &self.udma060
    }
    #[doc = "0x64 - UART2 TX write pointer"]
    #[inline(always)]
    pub const fn udma064(&self) -> &Udma064 {
        &self.udma064
    }
    #[doc = "0x68 - UART2 TX buffer base address"]
    #[inline(always)]
    pub const fn udma068(&self) -> &Udma068 {
        &self.udma068
    }
    #[doc = "0x70 - UART2 RX read pointer"]
    #[inline(always)]
    pub const fn udma070(&self) -> &Udma070 {
        &self.udma070
    }
    #[doc = "0x74 - UART2 RX write pointer"]
    #[inline(always)]
    pub const fn udma074(&self) -> &Udma074 {
        &self.udma074
    }
    #[doc = "0x78 - UART2 RX buffer base address"]
    #[inline(always)]
    pub const fn udma078(&self) -> &Udma078 {
        &self.udma078
    }
    #[doc = "0x80 - UART3 TX read pointer"]
    #[inline(always)]
    pub const fn udma080(&self) -> &Udma080 {
        &self.udma080
    }
    #[doc = "0x84 - UART3 TX write pointer"]
    #[inline(always)]
    pub const fn udma084(&self) -> &Udma084 {
        &self.udma084
    }
    #[doc = "0x88 - UART3 TX buffer base address"]
    #[inline(always)]
    pub const fn udma088(&self) -> &Udma088 {
        &self.udma088
    }
    #[doc = "0x90 - UART3 RX read pointer"]
    #[inline(always)]
    pub const fn udma090(&self) -> &Udma090 {
        &self.udma090
    }
    #[doc = "0x94 - UART3 RX write pointer"]
    #[inline(always)]
    pub const fn udma094(&self) -> &Udma094 {
        &self.udma094
    }
    #[doc = "0x98 - UART3 RX buffer base address"]
    #[inline(always)]
    pub const fn udma098(&self) -> &Udma098 {
        &self.udma098
    }
    #[doc = "0xa0 - UART4 TX read pointer"]
    #[inline(always)]
    pub const fn udma0a0(&self) -> &Udma0a0 {
        &self.udma0a0
    }
    #[doc = "0xa4 - UART4 TX write pointer"]
    #[inline(always)]
    pub const fn udma0a4(&self) -> &Udma0a4 {
        &self.udma0a4
    }
    #[doc = "0xa8 - UART4 TX buffer base address"]
    #[inline(always)]
    pub const fn udma0a8(&self) -> &Udma0a8 {
        &self.udma0a8
    }
    #[doc = "0xb0 - UART4 RX read pointer"]
    #[inline(always)]
    pub const fn udma0b0(&self) -> &Udma0b0 {
        &self.udma0b0
    }
    #[doc = "0xb4 - UART4 RX write pointer"]
    #[inline(always)]
    pub const fn udma0b4(&self) -> &Udma0b4 {
        &self.udma0b4
    }
    #[doc = "0xb8 - UART4 RX buffer base address"]
    #[inline(always)]
    pub const fn udma0b8(&self) -> &Udma0b8 {
        &self.udma0b8
    }
    #[doc = "0xc0 - UART5 TX read pointer"]
    #[inline(always)]
    pub const fn udma0c0(&self) -> &Udma0c0 {
        &self.udma0c0
    }
    #[doc = "0xc8 - UART5 TX buffer base address"]
    #[inline(always)]
    pub const fn udma0c8(&self) -> &Udma0c8 {
        &self.udma0c8
    }
    #[doc = "0xd0 - UART5 RX read pointer"]
    #[inline(always)]
    pub const fn udma0d0(&self) -> &Udma0d0 {
        &self.udma0d0
    }
    #[doc = "0xd4 - UART5 RX write pointer"]
    #[inline(always)]
    pub const fn udma0d4(&self) -> &Udma0d4 {
        &self.udma0d4
    }
    #[doc = "0xd8 - UART5 RX buffer base address"]
    #[inline(always)]
    pub const fn udma0d8(&self) -> &Udma0d8 {
        &self.udma0d8
    }
    #[doc = "0xe0 - UART6 TX read pointer"]
    #[inline(always)]
    pub const fn udma0e0(&self) -> &Udma0e0 {
        &self.udma0e0
    }
    #[doc = "0xe4 - UART6 TX write pointer"]
    #[inline(always)]
    pub const fn udma0e4(&self) -> &Udma0e4 {
        &self.udma0e4
    }
    #[doc = "0xe8 - UART6 TX buffer base address"]
    #[inline(always)]
    pub const fn udma0e8(&self) -> &Udma0e8 {
        &self.udma0e8
    }
    #[doc = "0xf0 - UART6 RX read pointer"]
    #[inline(always)]
    pub const fn udma0f0(&self) -> &Udma0f0 {
        &self.udma0f0
    }
    #[doc = "0xf4 - UART6 RX write pointer"]
    #[inline(always)]
    pub const fn udma0f4(&self) -> &Udma0f4 {
        &self.udma0f4
    }
    #[doc = "0xf8 - UART6 RX buffer base address"]
    #[inline(always)]
    pub const fn udma0f8(&self) -> &Udma0f8 {
        &self.udma0f8
    }
    #[doc = "0x100 - UART7 TX read pointer"]
    #[inline(always)]
    pub const fn udma100(&self) -> &Udma100 {
        &self.udma100
    }
    #[doc = "0x104 - UART7 TX write pointer"]
    #[inline(always)]
    pub const fn udma104(&self) -> &Udma104 {
        &self.udma104
    }
    #[doc = "0x108 - UART7 TX buffer base address"]
    #[inline(always)]
    pub const fn udma108(&self) -> &Udma108 {
        &self.udma108
    }
    #[doc = "0x110 - UART7 RX read pointer"]
    #[inline(always)]
    pub const fn udma110(&self) -> &Udma110 {
        &self.udma110
    }
    #[doc = "0x114 - UART7 RX write pointer"]
    #[inline(always)]
    pub const fn udma114(&self) -> &Udma114 {
        &self.udma114
    }
    #[doc = "0x118 - UART7 RX buffer base address"]
    #[inline(always)]
    pub const fn udma118(&self) -> &Udma118 {
        &self.udma118
    }
    #[doc = "0x120 - UART8 TX read pointer"]
    #[inline(always)]
    pub const fn udma120(&self) -> &Udma120 {
        &self.udma120
    }
    #[doc = "0x124 - UART8 TX write pointer"]
    #[inline(always)]
    pub const fn udma124(&self) -> &Udma124 {
        &self.udma124
    }
    #[doc = "0x128 - UART8 TX buffer base address"]
    #[inline(always)]
    pub const fn udma128(&self) -> &Udma128 {
        &self.udma128
    }
    #[doc = "0x130 - UART8 RX read pointer"]
    #[inline(always)]
    pub const fn udma130(&self) -> &Udma130 {
        &self.udma130
    }
    #[doc = "0x134 - UART8 RX write pointer"]
    #[inline(always)]
    pub const fn udma134(&self) -> &Udma134 {
        &self.udma134
    }
    #[doc = "0x138 - UART8 RX buffer base address"]
    #[inline(always)]
    pub const fn udma138(&self) -> &Udma138 {
        &self.udma138
    }
    #[doc = "0x140 - UART9 TX read pointer"]
    #[inline(always)]
    pub const fn udma140(&self) -> &Udma140 {
        &self.udma140
    }
    #[doc = "0x144 - UART9 TX write pointer"]
    #[inline(always)]
    pub const fn udma144(&self) -> &Udma144 {
        &self.udma144
    }
    #[doc = "0x148 - UART9 TX buffer base address"]
    #[inline(always)]
    pub const fn udma148(&self) -> &Udma148 {
        &self.udma148
    }
    #[doc = "0x150 - UART9 RX read pointer"]
    #[inline(always)]
    pub const fn udma150(&self) -> &Udma150 {
        &self.udma150
    }
    #[doc = "0x154 - UART9 RX write pointer"]
    #[inline(always)]
    pub const fn udma154(&self) -> &Udma154 {
        &self.udma154
    }
    #[doc = "0x158 - UART9 RX buffer base address"]
    #[inline(always)]
    pub const fn udma158(&self) -> &Udma158 {
        &self.udma158
    }
    #[doc = "0x160 - UART10 TX read pointer"]
    #[inline(always)]
    pub const fn udma160(&self) -> &Udma160 {
        &self.udma160
    }
    #[doc = "0x164 - UART10 TX write pointer"]
    #[inline(always)]
    pub const fn udma164(&self) -> &Udma164 {
        &self.udma164
    }
    #[doc = "0x168 - UART10 TX buffer base address"]
    #[inline(always)]
    pub const fn udma168(&self) -> &Udma168 {
        &self.udma168
    }
    #[doc = "0x170 - UART10 RX read pointer"]
    #[inline(always)]
    pub const fn udma170(&self) -> &Udma170 {
        &self.udma170
    }
    #[doc = "0x174 - UART10 RX write pointer"]
    #[inline(always)]
    pub const fn udma174(&self) -> &Udma174 {
        &self.udma174
    }
    #[doc = "0x178 - UART10 RX buffer base address"]
    #[inline(always)]
    pub const fn udma178(&self) -> &Udma178 {
        &self.udma178
    }
    #[doc = "0x180 - UART11 TX read pointer"]
    #[inline(always)]
    pub const fn udma180(&self) -> &Udma180 {
        &self.udma180
    }
    #[doc = "0x184 - UART11 TX write pointer"]
    #[inline(always)]
    pub const fn udma184(&self) -> &Udma184 {
        &self.udma184
    }
    #[doc = "0x188 - UART11 TX buffer base address"]
    #[inline(always)]
    pub const fn udma188(&self) -> &Udma188 {
        &self.udma188
    }
    #[doc = "0x190 - UART11 RX read pointer"]
    #[inline(always)]
    pub const fn udma190(&self) -> &Udma190 {
        &self.udma190
    }
    #[doc = "0x194 - UART11 RX write pointer"]
    #[inline(always)]
    pub const fn udma194(&self) -> &Udma194 {
        &self.udma194
    }
    #[doc = "0x198 - UART11 RX buffer base address"]
    #[inline(always)]
    pub const fn udma198(&self) -> &Udma198 {
        &self.udma198
    }
    #[doc = "0x1a0 - UART12 TX read pointer"]
    #[inline(always)]
    pub const fn udma1a0(&self) -> &Udma1a0 {
        &self.udma1a0
    }
    #[doc = "0x1a4 - UART12 TX write pointer"]
    #[inline(always)]
    pub const fn udma1a4(&self) -> &Udma1a4 {
        &self.udma1a4
    }
    #[doc = "0x1a8 - UART12 TX buffer base address"]
    #[inline(always)]
    pub const fn udma1a8(&self) -> &Udma1a8 {
        &self.udma1a8
    }
    #[doc = "0x1b0 - UART12 RX read pointer"]
    #[inline(always)]
    pub const fn udma1b0(&self) -> &Udma1b0 {
        &self.udma1b0
    }
    #[doc = "0x1b4 - UART12 RX write pointer"]
    #[inline(always)]
    pub const fn udma1b4(&self) -> &Udma1b4 {
        &self.udma1b4
    }
    #[doc = "0x1b8 - UART12 RX buffer base address"]
    #[inline(always)]
    pub const fn udma1b8(&self) -> &Udma1b8 {
        &self.udma1b8
    }
}
#[doc = "UDMA000 (rw) register accessor: UART TX DMA enable\n\nYou can [`read`](crate::Reg::read) this register and get [`udma000::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma000::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udma000`] module"]
#[doc(alias = "UDMA000")]
pub type Udma000 = crate::Reg<udma000::Udma000Spec>;
#[doc = "UART TX DMA enable"]
pub mod udma000;
#[doc = "UDMA004 (rw) register accessor: UART RX DMA enable\n\nYou can [`read`](crate::Reg::read) this register and get [`udma004::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma004::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udma004`] module"]
#[doc(alias = "UDMA004")]
pub type Udma004 = crate::Reg<udma004::Udma004Spec>;
#[doc = "UART RX DMA enable"]
pub mod udma004;
#[doc = "UDMA008 (rw) register accessor: Misc, buffer size\n\nYou can [`read`](crate::Reg::read) this register and get [`udma008::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma008::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udma008`] module"]
#[doc(alias = "UDMA008")]
pub type Udma008 = crate::Reg<udma008::Udma008Spec>;
#[doc = "Misc, buffer size"]
pub mod udma008;
#[doc = "UDMA00C (rw) register accessor: UART DMA time out timer\n\nYou can [`read`](crate::Reg::read) this register and get [`udma00c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma00c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udma00c`] module"]
#[doc(alias = "UDMA00C")]
pub type Udma00c = crate::Reg<udma00c::Udma00cSpec>;
#[doc = "UART DMA time out timer"]
pub mod udma00c;
#[doc = "UDMA020 (rw) register accessor: UART TX DMA reset\n\nYou can [`read`](crate::Reg::read) this register and get [`udma020::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma020::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udma020`] module"]
#[doc(alias = "UDMA020")]
pub type Udma020 = crate::Reg<udma020::Udma020Spec>;
#[doc = "UART TX DMA reset"]
pub mod udma020;
#[doc = "UDMA024 (rw) register accessor: UART RX DMA reset\n\nYou can [`read`](crate::Reg::read) this register and get [`udma024::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma024::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udma024`] module"]
#[doc(alias = "UDMA024")]
pub type Udma024 = crate::Reg<udma024::Udma024Spec>;
#[doc = "UART RX DMA reset"]
pub mod udma024;
#[doc = "UDMA30 (rw) register accessor: UART TX DMA interrupt enable\n\nYou can [`read`](crate::Reg::read) this register and get [`udma30::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma30::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udma30`] module"]
#[doc(alias = "UDMA30")]
pub type Udma30 = crate::Reg<udma30::Udma30Spec>;
#[doc = "UART TX DMA interrupt enable"]
pub mod udma30;
#[doc = "UDMA034 (rw) register accessor: UART TX DMA interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`udma034::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma034::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udma034`] module"]
#[doc(alias = "UDMA034")]
pub type Udma034 = crate::Reg<udma034::Udma034Spec>;
#[doc = "UART TX DMA interrupt status"]
pub mod udma034;
#[doc = "UDMA038 (rw) register accessor: UART RX DMA interrupt enable\n\nYou can [`read`](crate::Reg::read) this register and get [`udma038::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma038::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udma038`] module"]
#[doc(alias = "UDMA038")]
pub type Udma038 = crate::Reg<udma038::Udma038Spec>;
#[doc = "UART RX DMA interrupt enable"]
pub mod udma038;
#[doc = "UDMA03C (rw) register accessor: UART RX DMA interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`udma03c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma03c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udma03c`] module"]
#[doc(alias = "UDMA03C")]
pub type Udma03c = crate::Reg<udma03c::Udma03cSpec>;
#[doc = "UART RX DMA interrupt status"]
pub mod udma03c;
#[doc = "UDMA040 (rw) register accessor: UART1 TX read pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`udma040::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma040::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udma040`] module"]
#[doc(alias = "UDMA040")]
pub type Udma040 = crate::Reg<udma040::Udma040Spec>;
#[doc = "UART1 TX read pointer"]
pub mod udma040;
#[doc = "UDMA044 (rw) register accessor: UART1 TX write pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`udma044::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma044::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udma044`] module"]
#[doc(alias = "UDMA044")]
pub type Udma044 = crate::Reg<udma044::Udma044Spec>;
#[doc = "UART1 TX write pointer"]
pub mod udma044;
#[doc = "UDMA048 (rw) register accessor: UART1 TX buffer base address\n\nYou can [`read`](crate::Reg::read) this register and get [`udma048::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma048::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udma048`] module"]
#[doc(alias = "UDMA048")]
pub type Udma048 = crate::Reg<udma048::Udma048Spec>;
#[doc = "UART1 TX buffer base address"]
pub mod udma048;
#[doc = "UDMA050 (rw) register accessor: UART1 RX read pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`udma050::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma050::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udma050`] module"]
#[doc(alias = "UDMA050")]
pub type Udma050 = crate::Reg<udma050::Udma050Spec>;
#[doc = "UART1 RX read pointer"]
pub mod udma050;
#[doc = "UDMA054 (rw) register accessor: UART1 RX write pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`udma054::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma054::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udma054`] module"]
#[doc(alias = "UDMA054")]
pub type Udma054 = crate::Reg<udma054::Udma054Spec>;
#[doc = "UART1 RX write pointer"]
pub mod udma054;
#[doc = "UDMA058 (rw) register accessor: UART1 RX buffer base address\n\nYou can [`read`](crate::Reg::read) this register and get [`udma058::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma058::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udma058`] module"]
#[doc(alias = "UDMA058")]
pub type Udma058 = crate::Reg<udma058::Udma058Spec>;
#[doc = "UART1 RX buffer base address"]
pub mod udma058;
#[doc = "UDMA060 (rw) register accessor: UART2 TX read pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`udma060::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma060::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udma060`] module"]
#[doc(alias = "UDMA060")]
pub type Udma060 = crate::Reg<udma060::Udma060Spec>;
#[doc = "UART2 TX read pointer"]
pub mod udma060;
#[doc = "UDMA064 (rw) register accessor: UART2 TX write pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`udma064::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma064::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udma064`] module"]
#[doc(alias = "UDMA064")]
pub type Udma064 = crate::Reg<udma064::Udma064Spec>;
#[doc = "UART2 TX write pointer"]
pub mod udma064;
#[doc = "UDMA068 (rw) register accessor: UART2 TX buffer base address\n\nYou can [`read`](crate::Reg::read) this register and get [`udma068::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma068::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udma068`] module"]
#[doc(alias = "UDMA068")]
pub type Udma068 = crate::Reg<udma068::Udma068Spec>;
#[doc = "UART2 TX buffer base address"]
pub mod udma068;
#[doc = "UDMA070 (rw) register accessor: UART2 RX read pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`udma070::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma070::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udma070`] module"]
#[doc(alias = "UDMA070")]
pub type Udma070 = crate::Reg<udma070::Udma070Spec>;
#[doc = "UART2 RX read pointer"]
pub mod udma070;
#[doc = "UDMA074 (rw) register accessor: UART2 RX write pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`udma074::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma074::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udma074`] module"]
#[doc(alias = "UDMA074")]
pub type Udma074 = crate::Reg<udma074::Udma074Spec>;
#[doc = "UART2 RX write pointer"]
pub mod udma074;
#[doc = "UDMA078 (rw) register accessor: UART2 RX buffer base address\n\nYou can [`read`](crate::Reg::read) this register and get [`udma078::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma078::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udma078`] module"]
#[doc(alias = "UDMA078")]
pub type Udma078 = crate::Reg<udma078::Udma078Spec>;
#[doc = "UART2 RX buffer base address"]
pub mod udma078;
#[doc = "UDMA080 (rw) register accessor: UART3 TX read pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`udma080::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma080::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udma080`] module"]
#[doc(alias = "UDMA080")]
pub type Udma080 = crate::Reg<udma080::Udma080Spec>;
#[doc = "UART3 TX read pointer"]
pub mod udma080;
#[doc = "UDMA084 (rw) register accessor: UART3 TX write pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`udma084::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma084::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udma084`] module"]
#[doc(alias = "UDMA084")]
pub type Udma084 = crate::Reg<udma084::Udma084Spec>;
#[doc = "UART3 TX write pointer"]
pub mod udma084;
#[doc = "UDMA088 (rw) register accessor: UART3 TX buffer base address\n\nYou can [`read`](crate::Reg::read) this register and get [`udma088::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma088::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udma088`] module"]
#[doc(alias = "UDMA088")]
pub type Udma088 = crate::Reg<udma088::Udma088Spec>;
#[doc = "UART3 TX buffer base address"]
pub mod udma088;
#[doc = "UDMA090 (rw) register accessor: UART3 RX read pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`udma090::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma090::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udma090`] module"]
#[doc(alias = "UDMA090")]
pub type Udma090 = crate::Reg<udma090::Udma090Spec>;
#[doc = "UART3 RX read pointer"]
pub mod udma090;
#[doc = "UDMA094 (rw) register accessor: UART3 RX write pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`udma094::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma094::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udma094`] module"]
#[doc(alias = "UDMA094")]
pub type Udma094 = crate::Reg<udma094::Udma094Spec>;
#[doc = "UART3 RX write pointer"]
pub mod udma094;
#[doc = "UDMA098 (rw) register accessor: UART3 RX buffer base address\n\nYou can [`read`](crate::Reg::read) this register and get [`udma098::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma098::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udma098`] module"]
#[doc(alias = "UDMA098")]
pub type Udma098 = crate::Reg<udma098::Udma098Spec>;
#[doc = "UART3 RX buffer base address"]
pub mod udma098;
#[doc = "UDMA0A0 (rw) register accessor: UART4 TX read pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`udma0a0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma0a0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udma0a0`] module"]
#[doc(alias = "UDMA0A0")]
pub type Udma0a0 = crate::Reg<udma0a0::Udma0a0Spec>;
#[doc = "UART4 TX read pointer"]
pub mod udma0a0;
#[doc = "UDMA0A4 (rw) register accessor: UART4 TX write pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`udma0a4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma0a4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udma0a4`] module"]
#[doc(alias = "UDMA0A4")]
pub type Udma0a4 = crate::Reg<udma0a4::Udma0a4Spec>;
#[doc = "UART4 TX write pointer"]
pub mod udma0a4;
#[doc = "UDMA0A8 (rw) register accessor: UART4 TX buffer base address\n\nYou can [`read`](crate::Reg::read) this register and get [`udma0a8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma0a8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udma0a8`] module"]
#[doc(alias = "UDMA0A8")]
pub type Udma0a8 = crate::Reg<udma0a8::Udma0a8Spec>;
#[doc = "UART4 TX buffer base address"]
pub mod udma0a8;
#[doc = "UDMA0B0 (rw) register accessor: UART4 RX read pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`udma0b0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma0b0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udma0b0`] module"]
#[doc(alias = "UDMA0B0")]
pub type Udma0b0 = crate::Reg<udma0b0::Udma0b0Spec>;
#[doc = "UART4 RX read pointer"]
pub mod udma0b0;
#[doc = "UDMA0B4 (rw) register accessor: UART4 RX write pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`udma0b4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma0b4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udma0b4`] module"]
#[doc(alias = "UDMA0B4")]
pub type Udma0b4 = crate::Reg<udma0b4::Udma0b4Spec>;
#[doc = "UART4 RX write pointer"]
pub mod udma0b4;
#[doc = "UDMA0B8 (rw) register accessor: UART4 RX buffer base address\n\nYou can [`read`](crate::Reg::read) this register and get [`udma0b8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma0b8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udma0b8`] module"]
#[doc(alias = "UDMA0B8")]
pub type Udma0b8 = crate::Reg<udma0b8::Udma0b8Spec>;
#[doc = "UART4 RX buffer base address"]
pub mod udma0b8;
#[doc = "UDMA0C0 (rw) register accessor: UART5 TX read pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`udma0c0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma0c0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udma0c0`] module"]
#[doc(alias = "UDMA0C0")]
pub type Udma0c0 = crate::Reg<udma0c0::Udma0c0Spec>;
#[doc = "UART5 TX read pointer"]
pub mod udma0c0;
#[doc = "UDMA0C4 (rw) register accessor: UART5 TX write pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`udma0c4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma0c4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udma0c4`] module"]
#[doc(alias = "UDMA0C4")]
pub type Udma0c4 = crate::Reg<udma0c4::Udma0c4Spec>;
#[doc = "UART5 TX write pointer"]
pub mod udma0c4;
#[doc = "UDMA0C8 (rw) register accessor: UART5 TX buffer base address\n\nYou can [`read`](crate::Reg::read) this register and get [`udma0c8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma0c8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udma0c8`] module"]
#[doc(alias = "UDMA0C8")]
pub type Udma0c8 = crate::Reg<udma0c8::Udma0c8Spec>;
#[doc = "UART5 TX buffer base address"]
pub mod udma0c8;
#[doc = "UDMA0D0 (rw) register accessor: UART5 RX read pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`udma0d0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma0d0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udma0d0`] module"]
#[doc(alias = "UDMA0D0")]
pub type Udma0d0 = crate::Reg<udma0d0::Udma0d0Spec>;
#[doc = "UART5 RX read pointer"]
pub mod udma0d0;
#[doc = "UDMA0D4 (rw) register accessor: UART5 RX write pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`udma0d4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma0d4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udma0d4`] module"]
#[doc(alias = "UDMA0D4")]
pub type Udma0d4 = crate::Reg<udma0d4::Udma0d4Spec>;
#[doc = "UART5 RX write pointer"]
pub mod udma0d4;
#[doc = "UDMA0D8 (rw) register accessor: UART5 RX buffer base address\n\nYou can [`read`](crate::Reg::read) this register and get [`udma0d8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma0d8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udma0d8`] module"]
#[doc(alias = "UDMA0D8")]
pub type Udma0d8 = crate::Reg<udma0d8::Udma0d8Spec>;
#[doc = "UART5 RX buffer base address"]
pub mod udma0d8;
#[doc = "UDMA0E0 (rw) register accessor: UART6 TX read pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`udma0e0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma0e0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udma0e0`] module"]
#[doc(alias = "UDMA0E0")]
pub type Udma0e0 = crate::Reg<udma0e0::Udma0e0Spec>;
#[doc = "UART6 TX read pointer"]
pub mod udma0e0;
#[doc = "UDMA0E4 (rw) register accessor: UART6 TX write pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`udma0e4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma0e4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udma0e4`] module"]
#[doc(alias = "UDMA0E4")]
pub type Udma0e4 = crate::Reg<udma0e4::Udma0e4Spec>;
#[doc = "UART6 TX write pointer"]
pub mod udma0e4;
#[doc = "UDMA0E8 (rw) register accessor: UART6 TX buffer base address\n\nYou can [`read`](crate::Reg::read) this register and get [`udma0e8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma0e8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udma0e8`] module"]
#[doc(alias = "UDMA0E8")]
pub type Udma0e8 = crate::Reg<udma0e8::Udma0e8Spec>;
#[doc = "UART6 TX buffer base address"]
pub mod udma0e8;
#[doc = "UDMA0F0 (rw) register accessor: UART6 RX read pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`udma0f0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma0f0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udma0f0`] module"]
#[doc(alias = "UDMA0F0")]
pub type Udma0f0 = crate::Reg<udma0f0::Udma0f0Spec>;
#[doc = "UART6 RX read pointer"]
pub mod udma0f0;
#[doc = "UDMA0F4 (rw) register accessor: UART6 RX write pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`udma0f4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma0f4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udma0f4`] module"]
#[doc(alias = "UDMA0F4")]
pub type Udma0f4 = crate::Reg<udma0f4::Udma0f4Spec>;
#[doc = "UART6 RX write pointer"]
pub mod udma0f4;
#[doc = "UDMA0F8 (rw) register accessor: UART6 RX buffer base address\n\nYou can [`read`](crate::Reg::read) this register and get [`udma0f8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma0f8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udma0f8`] module"]
#[doc(alias = "UDMA0F8")]
pub type Udma0f8 = crate::Reg<udma0f8::Udma0f8Spec>;
#[doc = "UART6 RX buffer base address"]
pub mod udma0f8;
#[doc = "UDMA100 (rw) register accessor: UART7 TX read pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`udma100::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma100::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udma100`] module"]
#[doc(alias = "UDMA100")]
pub type Udma100 = crate::Reg<udma100::Udma100Spec>;
#[doc = "UART7 TX read pointer"]
pub mod udma100;
#[doc = "UDMA104 (rw) register accessor: UART7 TX write pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`udma104::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma104::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udma104`] module"]
#[doc(alias = "UDMA104")]
pub type Udma104 = crate::Reg<udma104::Udma104Spec>;
#[doc = "UART7 TX write pointer"]
pub mod udma104;
#[doc = "UDMA108 (rw) register accessor: UART7 TX buffer base address\n\nYou can [`read`](crate::Reg::read) this register and get [`udma108::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma108::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udma108`] module"]
#[doc(alias = "UDMA108")]
pub type Udma108 = crate::Reg<udma108::Udma108Spec>;
#[doc = "UART7 TX buffer base address"]
pub mod udma108;
#[doc = "UDMA110 (rw) register accessor: UART7 RX read pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`udma110::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma110::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udma110`] module"]
#[doc(alias = "UDMA110")]
pub type Udma110 = crate::Reg<udma110::Udma110Spec>;
#[doc = "UART7 RX read pointer"]
pub mod udma110;
#[doc = "UDMA114 (rw) register accessor: UART7 RX write pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`udma114::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma114::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udma114`] module"]
#[doc(alias = "UDMA114")]
pub type Udma114 = crate::Reg<udma114::Udma114Spec>;
#[doc = "UART7 RX write pointer"]
pub mod udma114;
#[doc = "UDMA118 (rw) register accessor: UART7 RX buffer base address\n\nYou can [`read`](crate::Reg::read) this register and get [`udma118::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma118::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udma118`] module"]
#[doc(alias = "UDMA118")]
pub type Udma118 = crate::Reg<udma118::Udma118Spec>;
#[doc = "UART7 RX buffer base address"]
pub mod udma118;
#[doc = "UDMA120 (rw) register accessor: UART8 TX read pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`udma120::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma120::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udma120`] module"]
#[doc(alias = "UDMA120")]
pub type Udma120 = crate::Reg<udma120::Udma120Spec>;
#[doc = "UART8 TX read pointer"]
pub mod udma120;
#[doc = "UDMA124 (rw) register accessor: UART8 TX write pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`udma124::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma124::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udma124`] module"]
#[doc(alias = "UDMA124")]
pub type Udma124 = crate::Reg<udma124::Udma124Spec>;
#[doc = "UART8 TX write pointer"]
pub mod udma124;
#[doc = "UDMA128 (rw) register accessor: UART8 TX buffer base address\n\nYou can [`read`](crate::Reg::read) this register and get [`udma128::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma128::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udma128`] module"]
#[doc(alias = "UDMA128")]
pub type Udma128 = crate::Reg<udma128::Udma128Spec>;
#[doc = "UART8 TX buffer base address"]
pub mod udma128;
#[doc = "UDMA130 (rw) register accessor: UART8 RX read pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`udma130::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma130::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udma130`] module"]
#[doc(alias = "UDMA130")]
pub type Udma130 = crate::Reg<udma130::Udma130Spec>;
#[doc = "UART8 RX read pointer"]
pub mod udma130;
#[doc = "UDMA134 (rw) register accessor: UART8 RX write pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`udma134::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma134::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udma134`] module"]
#[doc(alias = "UDMA134")]
pub type Udma134 = crate::Reg<udma134::Udma134Spec>;
#[doc = "UART8 RX write pointer"]
pub mod udma134;
#[doc = "UDMA138 (rw) register accessor: UART8 RX buffer base address\n\nYou can [`read`](crate::Reg::read) this register and get [`udma138::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma138::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udma138`] module"]
#[doc(alias = "UDMA138")]
pub type Udma138 = crate::Reg<udma138::Udma138Spec>;
#[doc = "UART8 RX buffer base address"]
pub mod udma138;
#[doc = "UDMA140 (rw) register accessor: UART9 TX read pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`udma140::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma140::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udma140`] module"]
#[doc(alias = "UDMA140")]
pub type Udma140 = crate::Reg<udma140::Udma140Spec>;
#[doc = "UART9 TX read pointer"]
pub mod udma140;
#[doc = "UDMA144 (rw) register accessor: UART9 TX write pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`udma144::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma144::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udma144`] module"]
#[doc(alias = "UDMA144")]
pub type Udma144 = crate::Reg<udma144::Udma144Spec>;
#[doc = "UART9 TX write pointer"]
pub mod udma144;
#[doc = "UDMA148 (rw) register accessor: UART9 TX buffer base address\n\nYou can [`read`](crate::Reg::read) this register and get [`udma148::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma148::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udma148`] module"]
#[doc(alias = "UDMA148")]
pub type Udma148 = crate::Reg<udma148::Udma148Spec>;
#[doc = "UART9 TX buffer base address"]
pub mod udma148;
#[doc = "UDMA150 (rw) register accessor: UART9 RX read pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`udma150::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma150::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udma150`] module"]
#[doc(alias = "UDMA150")]
pub type Udma150 = crate::Reg<udma150::Udma150Spec>;
#[doc = "UART9 RX read pointer"]
pub mod udma150;
#[doc = "UDMA154 (rw) register accessor: UART9 RX write pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`udma154::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma154::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udma154`] module"]
#[doc(alias = "UDMA154")]
pub type Udma154 = crate::Reg<udma154::Udma154Spec>;
#[doc = "UART9 RX write pointer"]
pub mod udma154;
#[doc = "UDMA158 (rw) register accessor: UART9 RX buffer base address\n\nYou can [`read`](crate::Reg::read) this register and get [`udma158::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma158::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udma158`] module"]
#[doc(alias = "UDMA158")]
pub type Udma158 = crate::Reg<udma158::Udma158Spec>;
#[doc = "UART9 RX buffer base address"]
pub mod udma158;
#[doc = "UDMA160 (rw) register accessor: UART10 TX read pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`udma160::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma160::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udma160`] module"]
#[doc(alias = "UDMA160")]
pub type Udma160 = crate::Reg<udma160::Udma160Spec>;
#[doc = "UART10 TX read pointer"]
pub mod udma160;
#[doc = "UDMA164 (rw) register accessor: UART10 TX write pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`udma164::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma164::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udma164`] module"]
#[doc(alias = "UDMA164")]
pub type Udma164 = crate::Reg<udma164::Udma164Spec>;
#[doc = "UART10 TX write pointer"]
pub mod udma164;
#[doc = "UDMA168 (rw) register accessor: UART10 TX buffer base address\n\nYou can [`read`](crate::Reg::read) this register and get [`udma168::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma168::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udma168`] module"]
#[doc(alias = "UDMA168")]
pub type Udma168 = crate::Reg<udma168::Udma168Spec>;
#[doc = "UART10 TX buffer base address"]
pub mod udma168;
#[doc = "UDMA170 (rw) register accessor: UART10 RX read pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`udma170::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma170::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udma170`] module"]
#[doc(alias = "UDMA170")]
pub type Udma170 = crate::Reg<udma170::Udma170Spec>;
#[doc = "UART10 RX read pointer"]
pub mod udma170;
#[doc = "UDMA174 (rw) register accessor: UART10 RX write pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`udma174::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma174::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udma174`] module"]
#[doc(alias = "UDMA174")]
pub type Udma174 = crate::Reg<udma174::Udma174Spec>;
#[doc = "UART10 RX write pointer"]
pub mod udma174;
#[doc = "UDMA178 (rw) register accessor: UART10 RX buffer base address\n\nYou can [`read`](crate::Reg::read) this register and get [`udma178::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma178::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udma178`] module"]
#[doc(alias = "UDMA178")]
pub type Udma178 = crate::Reg<udma178::Udma178Spec>;
#[doc = "UART10 RX buffer base address"]
pub mod udma178;
#[doc = "UDMA180 (rw) register accessor: UART11 TX read pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`udma180::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma180::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udma180`] module"]
#[doc(alias = "UDMA180")]
pub type Udma180 = crate::Reg<udma180::Udma180Spec>;
#[doc = "UART11 TX read pointer"]
pub mod udma180;
#[doc = "UDMA184 (rw) register accessor: UART11 TX write pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`udma184::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma184::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udma184`] module"]
#[doc(alias = "UDMA184")]
pub type Udma184 = crate::Reg<udma184::Udma184Spec>;
#[doc = "UART11 TX write pointer"]
pub mod udma184;
#[doc = "UDMA188 (rw) register accessor: UART11 TX buffer base address\n\nYou can [`read`](crate::Reg::read) this register and get [`udma188::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma188::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udma188`] module"]
#[doc(alias = "UDMA188")]
pub type Udma188 = crate::Reg<udma188::Udma188Spec>;
#[doc = "UART11 TX buffer base address"]
pub mod udma188;
#[doc = "UDMA190 (rw) register accessor: UART11 RX read pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`udma190::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma190::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udma190`] module"]
#[doc(alias = "UDMA190")]
pub type Udma190 = crate::Reg<udma190::Udma190Spec>;
#[doc = "UART11 RX read pointer"]
pub mod udma190;
#[doc = "UDMA194 (rw) register accessor: UART11 RX write pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`udma194::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma194::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udma194`] module"]
#[doc(alias = "UDMA194")]
pub type Udma194 = crate::Reg<udma194::Udma194Spec>;
#[doc = "UART11 RX write pointer"]
pub mod udma194;
#[doc = "UDMA198 (rw) register accessor: UART11 RX buffer base address\n\nYou can [`read`](crate::Reg::read) this register and get [`udma198::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma198::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udma198`] module"]
#[doc(alias = "UDMA198")]
pub type Udma198 = crate::Reg<udma198::Udma198Spec>;
#[doc = "UART11 RX buffer base address"]
pub mod udma198;
#[doc = "UDMA1A0 (rw) register accessor: UART12 TX read pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`udma1a0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma1a0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udma1a0`] module"]
#[doc(alias = "UDMA1A0")]
pub type Udma1a0 = crate::Reg<udma1a0::Udma1a0Spec>;
#[doc = "UART12 TX read pointer"]
pub mod udma1a0;
#[doc = "UDMA1A4 (rw) register accessor: UART12 TX write pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`udma1a4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma1a4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udma1a4`] module"]
#[doc(alias = "UDMA1A4")]
pub type Udma1a4 = crate::Reg<udma1a4::Udma1a4Spec>;
#[doc = "UART12 TX write pointer"]
pub mod udma1a4;
#[doc = "UDMA1A8 (rw) register accessor: UART12 TX buffer base address\n\nYou can [`read`](crate::Reg::read) this register and get [`udma1a8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma1a8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udma1a8`] module"]
#[doc(alias = "UDMA1A8")]
pub type Udma1a8 = crate::Reg<udma1a8::Udma1a8Spec>;
#[doc = "UART12 TX buffer base address"]
pub mod udma1a8;
#[doc = "UDMA1B0 (rw) register accessor: UART12 RX read pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`udma1b0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma1b0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udma1b0`] module"]
#[doc(alias = "UDMA1B0")]
pub type Udma1b0 = crate::Reg<udma1b0::Udma1b0Spec>;
#[doc = "UART12 RX read pointer"]
pub mod udma1b0;
#[doc = "UDMA1B4 (rw) register accessor: UART12 RX write pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`udma1b4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma1b4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udma1b4`] module"]
#[doc(alias = "UDMA1B4")]
pub type Udma1b4 = crate::Reg<udma1b4::Udma1b4Spec>;
#[doc = "UART12 RX write pointer"]
pub mod udma1b4;
#[doc = "UDMA1B8 (rw) register accessor: UART12 RX buffer base address\n\nYou can [`read`](crate::Reg::read) this register and get [`udma1b8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma1b8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udma1b8`] module"]
#[doc(alias = "UDMA1B8")]
pub type Udma1b8 = crate::Reg<udma1b8::Udma1b8Spec>;
#[doc = "UART12 RX buffer base address"]
pub mod udma1b8;
