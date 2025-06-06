#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    gpio500: Gpio500,
    gpio504: Gpio504,
    gpio508: Gpio508,
    gpio50c: Gpio50c,
    gpio510: Gpio510,
    gpio514: Gpio514,
    gpio518: Gpio518,
    gpio51c: Gpio51c,
    gpio520: Gpio520,
    gpio524: Gpio524,
    gpio528: Gpio528,
    gpio52c: Gpio52c,
    gpio530: Gpio530,
    gpio534: Gpio534,
    gpio538: Gpio538,
    gpio53c: Gpio53c,
    gpio540: Gpio540,
    gpio544: Gpio544,
    gpio548: Gpio548,
    gpio54c: Gpio54c,
    gpio550: Gpio550,
    gpio554: Gpio554,
    gpio558: Gpio558,
    gpio55c: Gpio55c,
    gpio560: Gpio560,
    gpio564: Gpio564,
    _reserved26: [u8; 0x08],
    gpio570: Gpio570,
    gpio574: Gpio574,
    gpio578: Gpio578,
    _reserved29: [u8; 0x14],
    gpio590: Gpio590,
    gpio594: Gpio594,
    gpio598: Gpio598,
    gpio59c: Gpio59c,
    gpio5a0: Gpio5a0,
    gpio5a4: Gpio5a4,
    gpio5a8: Gpio5a8,
}
impl RegisterBlock {
    #[doc = "0x00 - Serial GPIO\\_A/B/C/D 1 Data Value Register"]
    #[inline(always)]
    pub const fn gpio500(&self) -> &Gpio500 {
        &self.gpio500
    }
    #[doc = "0x04 - Serial GPIO\\_A/B/C/D 1 Interrupt Enable Register"]
    #[inline(always)]
    pub const fn gpio504(&self) -> &Gpio504 {
        &self.gpio504
    }
    #[doc = "0x08 - Serial GPIO\\_A/B/C/D 1 Interrupt Sensitivity Type 0 Register"]
    #[inline(always)]
    pub const fn gpio508(&self) -> &Gpio508 {
        &self.gpio508
    }
    #[doc = "0x0c - Serial GPIO\\_A/B/C/D 1 Interrupt Sensitivity Type 1 Register"]
    #[inline(always)]
    pub const fn gpio50c(&self) -> &Gpio50c {
        &self.gpio50c
    }
    #[doc = "0x10 - Serial GPIO\\_A/B/C/D 1 Interrupt Sensitivity Type 2 Register"]
    #[inline(always)]
    pub const fn gpio510(&self) -> &Gpio510 {
        &self.gpio510
    }
    #[doc = "0x14 - Serial GPIO\\_A/B/C/D 1 Interrupt Status Register"]
    #[inline(always)]
    pub const fn gpio514(&self) -> &Gpio514 {
        &self.gpio514
    }
    #[doc = "0x18 - Serial GPIO\\_A/B/C/D 1 Reset Tolerant Register"]
    #[inline(always)]
    pub const fn gpio518(&self) -> &Gpio518 {
        &self.gpio518
    }
    #[doc = "0x1c - Serial GPIO\\_E/F/G/H 1 Data Value Register"]
    #[inline(always)]
    pub const fn gpio51c(&self) -> &Gpio51c {
        &self.gpio51c
    }
    #[doc = "0x20 - Serial GPIO\\_E/F/G/H 1 Interrupt Enable Register"]
    #[inline(always)]
    pub const fn gpio520(&self) -> &Gpio520 {
        &self.gpio520
    }
    #[doc = "0x24 - Serial GPIO\\_E/F/G/H 1 Interrupt Sensitivity Type 0 Register"]
    #[inline(always)]
    pub const fn gpio524(&self) -> &Gpio524 {
        &self.gpio524
    }
    #[doc = "0x28 - Serial GPIO\\_E/F/G/H 1 Interrupt Sensitivity Type 1 Register"]
    #[inline(always)]
    pub const fn gpio528(&self) -> &Gpio528 {
        &self.gpio528
    }
    #[doc = "0x2c - Serial GPIO\\_E/F/G/H 1 Interrupt Sensitivity Type 2 Register"]
    #[inline(always)]
    pub const fn gpio52c(&self) -> &Gpio52c {
        &self.gpio52c
    }
    #[doc = "0x30 - Serial GPIO\\_E/F/G/H 1 Interrupt Status Register"]
    #[inline(always)]
    pub const fn gpio530(&self) -> &Gpio530 {
        &self.gpio530
    }
    #[doc = "0x34 - Serial GPIO\\_E/F/G/H 1 Reset Tolerant Register"]
    #[inline(always)]
    pub const fn gpio534(&self) -> &Gpio534 {
        &self.gpio534
    }
    #[doc = "0x38 - Serial GPIO\\_I/J/K/L 1 Data Value Register"]
    #[inline(always)]
    pub const fn gpio538(&self) -> &Gpio538 {
        &self.gpio538
    }
    #[doc = "0x3c - Serial GPIO\\_I/J/K/L 1 Interrupt Enable Register"]
    #[inline(always)]
    pub const fn gpio53c(&self) -> &Gpio53c {
        &self.gpio53c
    }
    #[doc = "0x40 - Serial GPIO\\_I/J/K/L 1 Interrupt Sensitivity Type 0 Register"]
    #[inline(always)]
    pub const fn gpio540(&self) -> &Gpio540 {
        &self.gpio540
    }
    #[doc = "0x44 - Serial GPIO\\_I/J/K/L 1 Interrupt Sensitivity Type 1 Register"]
    #[inline(always)]
    pub const fn gpio544(&self) -> &Gpio544 {
        &self.gpio544
    }
    #[doc = "0x48 - Serial GPIO\\_I/J/K/L 1 Interrupt Sensitivity Type 2 Register"]
    #[inline(always)]
    pub const fn gpio548(&self) -> &Gpio548 {
        &self.gpio548
    }
    #[doc = "0x4c - Serial GPIO\\_I/J/K/L 1 Interrupt Status Register"]
    #[inline(always)]
    pub const fn gpio54c(&self) -> &Gpio54c {
        &self.gpio54c
    }
    #[doc = "0x50 - Serial GPIO\\_I/J/K/L 1 Reset Tolerant Register"]
    #[inline(always)]
    pub const fn gpio550(&self) -> &Gpio550 {
        &self.gpio550
    }
    #[doc = "0x54 - Serial GPIO 1 Configuration Register"]
    #[inline(always)]
    pub const fn gpio554(&self) -> &Gpio554 {
        &self.gpio554
    }
    #[doc = "0x58 - Serial GPIO\\_A/B/C/D Input Mask Register"]
    #[inline(always)]
    pub const fn gpio558(&self) -> &Gpio558 {
        &self.gpio558
    }
    #[doc = "0x5c - Serial GPIO\\_E/F/G/H Input Mask Register"]
    #[inline(always)]
    pub const fn gpio55c(&self) -> &Gpio55c {
        &self.gpio55c
    }
    #[doc = "0x60 - Serial GPIO\\_I/J/K/L Input Mask Register"]
    #[inline(always)]
    pub const fn gpio560(&self) -> &Gpio560 {
        &self.gpio560
    }
    #[doc = "0x64 - Serial GPIO\\_M/N/O/P Input Mask Register"]
    #[inline(always)]
    pub const fn gpio564(&self) -> &Gpio564 {
        &self.gpio564
    }
    #[doc = "0x70 - Serial GPIO\\_A/B/C/D 1 Data Read Register"]
    #[inline(always)]
    pub const fn gpio570(&self) -> &Gpio570 {
        &self.gpio570
    }
    #[doc = "0x74 - Serial GPIO\\_E/F/G/H 1 Data Read Register"]
    #[inline(always)]
    pub const fn gpio574(&self) -> &Gpio574 {
        &self.gpio574
    }
    #[doc = "0x78 - Serial GPIO\\_I/J/K/L 1 Data Read Register"]
    #[inline(always)]
    pub const fn gpio578(&self) -> &Gpio578 {
        &self.gpio578
    }
    #[doc = "0x90 - Serial GPIO\\_M/N/O/P 1 Data Value Register"]
    #[inline(always)]
    pub const fn gpio590(&self) -> &Gpio590 {
        &self.gpio590
    }
    #[doc = "0x94 - Serial GPIO\\_M/N/O/P 1 Interrupt Enable Register"]
    #[inline(always)]
    pub const fn gpio594(&self) -> &Gpio594 {
        &self.gpio594
    }
    #[doc = "0x98 - Serial GPIO\\_M/N/O/P 1 Interrupt Sensitivity Type 0 Register"]
    #[inline(always)]
    pub const fn gpio598(&self) -> &Gpio598 {
        &self.gpio598
    }
    #[doc = "0x9c - Serial GPIO\\_M/N/O/P 1 Interrupt Sensitivity Type 1 Register"]
    #[inline(always)]
    pub const fn gpio59c(&self) -> &Gpio59c {
        &self.gpio59c
    }
    #[doc = "0xa0 - Serial GPIO\\_M/N/O/P 1 Interrupt Sensitivity Type 2 Register"]
    #[inline(always)]
    pub const fn gpio5a0(&self) -> &Gpio5a0 {
        &self.gpio5a0
    }
    #[doc = "0xa4 - Serial GPIO\\_M/N/O/P 1 Interrupt Status Register"]
    #[inline(always)]
    pub const fn gpio5a4(&self) -> &Gpio5a4 {
        &self.gpio5a4
    }
    #[doc = "0xa8 - Serial GPIO\\_M/N/O/P 1 Reset Tolerant Register"]
    #[inline(always)]
    pub const fn gpio5a8(&self) -> &Gpio5a8 {
        &self.gpio5a8
    }
}
#[doc = "GPIO500 (rw) register accessor: Serial GPIO\\_A/B/C/D 1 Data Value Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio500::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio500::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio500`] module"]
#[doc(alias = "GPIO500")]
pub type Gpio500 = crate::Reg<gpio500::Gpio500Spec>;
#[doc = "Serial GPIO\\_A/B/C/D 1 Data Value Register"]
pub mod gpio500;
#[doc = "GPIO504 (rw) register accessor: Serial GPIO\\_A/B/C/D 1 Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio504::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio504::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio504`] module"]
#[doc(alias = "GPIO504")]
pub type Gpio504 = crate::Reg<gpio504::Gpio504Spec>;
#[doc = "Serial GPIO\\_A/B/C/D 1 Interrupt Enable Register"]
pub mod gpio504;
#[doc = "GPIO508 (rw) register accessor: Serial GPIO\\_A/B/C/D 1 Interrupt Sensitivity Type 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio508::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio508::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio508`] module"]
#[doc(alias = "GPIO508")]
pub type Gpio508 = crate::Reg<gpio508::Gpio508Spec>;
#[doc = "Serial GPIO\\_A/B/C/D 1 Interrupt Sensitivity Type 0 Register"]
pub mod gpio508;
#[doc = "GPIO50C (rw) register accessor: Serial GPIO\\_A/B/C/D 1 Interrupt Sensitivity Type 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio50c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio50c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio50c`] module"]
#[doc(alias = "GPIO50C")]
pub type Gpio50c = crate::Reg<gpio50c::Gpio50cSpec>;
#[doc = "Serial GPIO\\_A/B/C/D 1 Interrupt Sensitivity Type 1 Register"]
pub mod gpio50c;
#[doc = "GPIO510 (rw) register accessor: Serial GPIO\\_A/B/C/D 1 Interrupt Sensitivity Type 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio510::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio510::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio510`] module"]
#[doc(alias = "GPIO510")]
pub type Gpio510 = crate::Reg<gpio510::Gpio510Spec>;
#[doc = "Serial GPIO\\_A/B/C/D 1 Interrupt Sensitivity Type 2 Register"]
pub mod gpio510;
#[doc = "GPIO514 (rw) register accessor: Serial GPIO\\_A/B/C/D 1 Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio514::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio514::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio514`] module"]
#[doc(alias = "GPIO514")]
pub type Gpio514 = crate::Reg<gpio514::Gpio514Spec>;
#[doc = "Serial GPIO\\_A/B/C/D 1 Interrupt Status Register"]
pub mod gpio514;
#[doc = "GPIO518 (rw) register accessor: Serial GPIO\\_A/B/C/D 1 Reset Tolerant Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio518::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio518::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio518`] module"]
#[doc(alias = "GPIO518")]
pub type Gpio518 = crate::Reg<gpio518::Gpio518Spec>;
#[doc = "Serial GPIO\\_A/B/C/D 1 Reset Tolerant Register"]
pub mod gpio518;
#[doc = "GPIO51C (rw) register accessor: Serial GPIO\\_E/F/G/H 1 Data Value Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio51c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio51c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio51c`] module"]
#[doc(alias = "GPIO51C")]
pub type Gpio51c = crate::Reg<gpio51c::Gpio51cSpec>;
#[doc = "Serial GPIO\\_E/F/G/H 1 Data Value Register"]
pub mod gpio51c;
#[doc = "GPIO520 (rw) register accessor: Serial GPIO\\_E/F/G/H 1 Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio520::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio520::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio520`] module"]
#[doc(alias = "GPIO520")]
pub type Gpio520 = crate::Reg<gpio520::Gpio520Spec>;
#[doc = "Serial GPIO\\_E/F/G/H 1 Interrupt Enable Register"]
pub mod gpio520;
#[doc = "GPIO524 (rw) register accessor: Serial GPIO\\_E/F/G/H 1 Interrupt Sensitivity Type 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio524::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio524::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio524`] module"]
#[doc(alias = "GPIO524")]
pub type Gpio524 = crate::Reg<gpio524::Gpio524Spec>;
#[doc = "Serial GPIO\\_E/F/G/H 1 Interrupt Sensitivity Type 0 Register"]
pub mod gpio524;
#[doc = "GPIO528 (rw) register accessor: Serial GPIO\\_E/F/G/H 1 Interrupt Sensitivity Type 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio528::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio528::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio528`] module"]
#[doc(alias = "GPIO528")]
pub type Gpio528 = crate::Reg<gpio528::Gpio528Spec>;
#[doc = "Serial GPIO\\_E/F/G/H 1 Interrupt Sensitivity Type 1 Register"]
pub mod gpio528;
#[doc = "GPIO52C (rw) register accessor: Serial GPIO\\_E/F/G/H 1 Interrupt Sensitivity Type 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio52c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio52c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio52c`] module"]
#[doc(alias = "GPIO52C")]
pub type Gpio52c = crate::Reg<gpio52c::Gpio52cSpec>;
#[doc = "Serial GPIO\\_E/F/G/H 1 Interrupt Sensitivity Type 2 Register"]
pub mod gpio52c;
#[doc = "GPIO530 (rw) register accessor: Serial GPIO\\_E/F/G/H 1 Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio530::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio530::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio530`] module"]
#[doc(alias = "GPIO530")]
pub type Gpio530 = crate::Reg<gpio530::Gpio530Spec>;
#[doc = "Serial GPIO\\_E/F/G/H 1 Interrupt Status Register"]
pub mod gpio530;
#[doc = "GPIO534 (rw) register accessor: Serial GPIO\\_E/F/G/H 1 Reset Tolerant Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio534::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio534::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio534`] module"]
#[doc(alias = "GPIO534")]
pub type Gpio534 = crate::Reg<gpio534::Gpio534Spec>;
#[doc = "Serial GPIO\\_E/F/G/H 1 Reset Tolerant Register"]
pub mod gpio534;
#[doc = "GPIO538 (rw) register accessor: Serial GPIO\\_I/J/K/L 1 Data Value Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio538::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio538::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio538`] module"]
#[doc(alias = "GPIO538")]
pub type Gpio538 = crate::Reg<gpio538::Gpio538Spec>;
#[doc = "Serial GPIO\\_I/J/K/L 1 Data Value Register"]
pub mod gpio538;
#[doc = "GPIO53C (rw) register accessor: Serial GPIO\\_I/J/K/L 1 Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio53c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio53c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio53c`] module"]
#[doc(alias = "GPIO53C")]
pub type Gpio53c = crate::Reg<gpio53c::Gpio53cSpec>;
#[doc = "Serial GPIO\\_I/J/K/L 1 Interrupt Enable Register"]
pub mod gpio53c;
#[doc = "GPIO540 (rw) register accessor: Serial GPIO\\_I/J/K/L 1 Interrupt Sensitivity Type 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio540::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio540::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio540`] module"]
#[doc(alias = "GPIO540")]
pub type Gpio540 = crate::Reg<gpio540::Gpio540Spec>;
#[doc = "Serial GPIO\\_I/J/K/L 1 Interrupt Sensitivity Type 0 Register"]
pub mod gpio540;
#[doc = "GPIO544 (rw) register accessor: Serial GPIO\\_I/J/K/L 1 Interrupt Sensitivity Type 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio544::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio544::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio544`] module"]
#[doc(alias = "GPIO544")]
pub type Gpio544 = crate::Reg<gpio544::Gpio544Spec>;
#[doc = "Serial GPIO\\_I/J/K/L 1 Interrupt Sensitivity Type 1 Register"]
pub mod gpio544;
#[doc = "GPIO548 (rw) register accessor: Serial GPIO\\_I/J/K/L 1 Interrupt Sensitivity Type 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio548::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio548::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio548`] module"]
#[doc(alias = "GPIO548")]
pub type Gpio548 = crate::Reg<gpio548::Gpio548Spec>;
#[doc = "Serial GPIO\\_I/J/K/L 1 Interrupt Sensitivity Type 2 Register"]
pub mod gpio548;
#[doc = "GPIO54C (rw) register accessor: Serial GPIO\\_I/J/K/L 1 Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio54c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio54c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio54c`] module"]
#[doc(alias = "GPIO54C")]
pub type Gpio54c = crate::Reg<gpio54c::Gpio54cSpec>;
#[doc = "Serial GPIO\\_I/J/K/L 1 Interrupt Status Register"]
pub mod gpio54c;
#[doc = "GPIO550 (rw) register accessor: Serial GPIO\\_I/J/K/L 1 Reset Tolerant Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio550::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio550::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio550`] module"]
#[doc(alias = "GPIO550")]
pub type Gpio550 = crate::Reg<gpio550::Gpio550Spec>;
#[doc = "Serial GPIO\\_I/J/K/L 1 Reset Tolerant Register"]
pub mod gpio550;
#[doc = "GPIO554 (rw) register accessor: Serial GPIO 1 Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio554::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio554::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio554`] module"]
#[doc(alias = "GPIO554")]
pub type Gpio554 = crate::Reg<gpio554::Gpio554Spec>;
#[doc = "Serial GPIO 1 Configuration Register"]
pub mod gpio554;
#[doc = "GPIO558 (rw) register accessor: Serial GPIO\\_A/B/C/D Input Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio558::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio558::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio558`] module"]
#[doc(alias = "GPIO558")]
pub type Gpio558 = crate::Reg<gpio558::Gpio558Spec>;
#[doc = "Serial GPIO\\_A/B/C/D Input Mask Register"]
pub mod gpio558;
#[doc = "GPIO55C (rw) register accessor: Serial GPIO\\_E/F/G/H Input Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio55c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio55c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio55c`] module"]
#[doc(alias = "GPIO55C")]
pub type Gpio55c = crate::Reg<gpio55c::Gpio55cSpec>;
#[doc = "Serial GPIO\\_E/F/G/H Input Mask Register"]
pub mod gpio55c;
#[doc = "GPIO560 (rw) register accessor: Serial GPIO\\_I/J/K/L Input Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio560::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio560::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio560`] module"]
#[doc(alias = "GPIO560")]
pub type Gpio560 = crate::Reg<gpio560::Gpio560Spec>;
#[doc = "Serial GPIO\\_I/J/K/L Input Mask Register"]
pub mod gpio560;
#[doc = "GPIO564 (rw) register accessor: Serial GPIO\\_M/N/O/P Input Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio564::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio564::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio564`] module"]
#[doc(alias = "GPIO564")]
pub type Gpio564 = crate::Reg<gpio564::Gpio564Spec>;
#[doc = "Serial GPIO\\_M/N/O/P Input Mask Register"]
pub mod gpio564;
#[doc = "GPIO570 (rw) register accessor: Serial GPIO\\_A/B/C/D 1 Data Read Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio570::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio570::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio570`] module"]
#[doc(alias = "GPIO570")]
pub type Gpio570 = crate::Reg<gpio570::Gpio570Spec>;
#[doc = "Serial GPIO\\_A/B/C/D 1 Data Read Register"]
pub mod gpio570;
#[doc = "GPIO574 (rw) register accessor: Serial GPIO\\_E/F/G/H 1 Data Read Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio574::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio574::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio574`] module"]
#[doc(alias = "GPIO574")]
pub type Gpio574 = crate::Reg<gpio574::Gpio574Spec>;
#[doc = "Serial GPIO\\_E/F/G/H 1 Data Read Register"]
pub mod gpio574;
#[doc = "GPIO578 (rw) register accessor: Serial GPIO\\_I/J/K/L 1 Data Read Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio578::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio578::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio578`] module"]
#[doc(alias = "GPIO578")]
pub type Gpio578 = crate::Reg<gpio578::Gpio578Spec>;
#[doc = "Serial GPIO\\_I/J/K/L 1 Data Read Register"]
pub mod gpio578;
#[doc = "GPIO590 (rw) register accessor: Serial GPIO\\_M/N/O/P 1 Data Value Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio590::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio590::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio590`] module"]
#[doc(alias = "GPIO590")]
pub type Gpio590 = crate::Reg<gpio590::Gpio590Spec>;
#[doc = "Serial GPIO\\_M/N/O/P 1 Data Value Register"]
pub mod gpio590;
#[doc = "GPIO594 (rw) register accessor: Serial GPIO\\_M/N/O/P 1 Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio594::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio594::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio594`] module"]
#[doc(alias = "GPIO594")]
pub type Gpio594 = crate::Reg<gpio594::Gpio594Spec>;
#[doc = "Serial GPIO\\_M/N/O/P 1 Interrupt Enable Register"]
pub mod gpio594;
#[doc = "GPIO598 (rw) register accessor: Serial GPIO\\_M/N/O/P 1 Interrupt Sensitivity Type 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio598::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio598::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio598`] module"]
#[doc(alias = "GPIO598")]
pub type Gpio598 = crate::Reg<gpio598::Gpio598Spec>;
#[doc = "Serial GPIO\\_M/N/O/P 1 Interrupt Sensitivity Type 0 Register"]
pub mod gpio598;
#[doc = "GPIO59C (rw) register accessor: Serial GPIO\\_M/N/O/P 1 Interrupt Sensitivity Type 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio59c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio59c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio59c`] module"]
#[doc(alias = "GPIO59C")]
pub type Gpio59c = crate::Reg<gpio59c::Gpio59cSpec>;
#[doc = "Serial GPIO\\_M/N/O/P 1 Interrupt Sensitivity Type 1 Register"]
pub mod gpio59c;
#[doc = "GPIO5A0 (rw) register accessor: Serial GPIO\\_M/N/O/P 1 Interrupt Sensitivity Type 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio5a0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio5a0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio5a0`] module"]
#[doc(alias = "GPIO5A0")]
pub type Gpio5a0 = crate::Reg<gpio5a0::Gpio5a0Spec>;
#[doc = "Serial GPIO\\_M/N/O/P 1 Interrupt Sensitivity Type 2 Register"]
pub mod gpio5a0;
#[doc = "GPIO5A4 (rw) register accessor: Serial GPIO\\_M/N/O/P 1 Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio5a4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio5a4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio5a4`] module"]
#[doc(alias = "GPIO5A4")]
pub type Gpio5a4 = crate::Reg<gpio5a4::Gpio5a4Spec>;
#[doc = "Serial GPIO\\_M/N/O/P 1 Interrupt Status Register"]
pub mod gpio5a4;
#[doc = "GPIO5A8 (rw) register accessor: Serial GPIO\\_M/N/O/P 1 Reset Tolerant Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio5a8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio5a8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio5a8`] module"]
#[doc(alias = "GPIO5A8")]
pub type Gpio5a8 = crate::Reg<gpio5a8::Gpio5a8Spec>;
#[doc = "Serial GPIO\\_M/N/O/P 1 Reset Tolerant Register"]
pub mod gpio5a8;
