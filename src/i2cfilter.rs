#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x08],
    i2cfilter008: I2cfilter008,
    i2cfilter00c: I2cfilter00c,
}
impl RegisterBlock {
    #[doc = "0x08 - I2CFLT\\_IRQEN"]
    #[inline(always)]
    pub const fn i2cfilter008(&self) -> &I2cfilter008 {
        &self.i2cfilter008
    }
    #[doc = "0x0c - I2CFLT\\_IRQSTA"]
    #[inline(always)]
    pub const fn i2cfilter00c(&self) -> &I2cfilter00c {
        &self.i2cfilter00c
    }
}
#[doc = "I2CFILTER008 (rw) register accessor: I2CFLT\\_IRQEN\n\nYou can [`read`](crate::Reg::read) this register and get [`i2cfilter008::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2cfilter008::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2cfilter008`] module"]
#[doc(alias = "I2CFILTER008")]
pub type I2cfilter008 = crate::Reg<i2cfilter008::I2cfilter008Spec>;
#[doc = "I2CFLT\\_IRQEN"]
pub mod i2cfilter008;
#[doc = "I2CFILTER00C (rw) register accessor: I2CFLT\\_IRQSTA\n\nYou can [`read`](crate::Reg::read) this register and get [`i2cfilter00c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2cfilter00c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2cfilter00c`] module"]
#[doc(alias = "I2CFILTER00C")]
pub type I2cfilter00c = crate::Reg<i2cfilter00c::I2cfilter00cSpec>;
#[doc = "I2CFLT\\_IRQSTA"]
pub mod i2cfilter00c;
