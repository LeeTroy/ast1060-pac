#[doc = "Register `SCU41C` reader"]
pub type R = crate::R<Scu41cSpec>;
#[doc = "Register `SCU41C` writer"]
pub type W = crate::W<Scu41cSpec>;
#[doc = "Field `Reserved1` reader - Reserved, must keep at value 0x0"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `Reserved1` writer - Reserved, must keep at value 0x0"]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `EnblSGPIOMasterCKFnPin` reader - Enable SGPIO Master CK function pin"]
pub type EnblSgpiomasterCkfnPinR = crate::BitReader;
#[doc = "Field `EnblSGPIOMasterCKFnPin` writer - Enable SGPIO Master CK function pin"]
pub type EnblSgpiomasterCkfnPinW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblSGPIOMasterLDFnPin` reader - Enable SGPIO Master LD function pin"]
pub type EnblSgpiomasterLdfnPinR = crate::BitReader;
#[doc = "Field `EnblSGPIOMasterLDFnPin` writer - Enable SGPIO Master LD function pin"]
pub type EnblSgpiomasterLdfnPinW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblSGPIOMasterDOFnPin` reader - Enable SGPIO Master DO function pin"]
pub type EnblSgpiomasterDofnPinR = crate::BitReader;
#[doc = "Field `EnblSGPIOMasterDOFnPin` writer - Enable SGPIO Master DO function pin"]
pub type EnblSgpiomasterDofnPinW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblSGPIOMasterDIFnPin` reader - Enable SGPIO Master DI function pin"]
pub type EnblSgpiomasterDifnPinR = crate::BitReader;
#[doc = "Field `EnblSGPIOMasterDIFnPin` writer - Enable SGPIO Master DI function pin"]
pub type EnblSgpiomasterDifnPinW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblARMTRSTFnPin` reader - Enable ARM TRST function pin"]
pub type EnblArmtrstfnPinR = crate::BitReader;
#[doc = "Field `EnblARMTRSTFnPin` writer - Enable ARM TRST function pin"]
pub type EnblArmtrstfnPinW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblARMTCKFnPin` reader - Enable ARM TCK function pin"]
pub type EnblArmtckfnPinR = crate::BitReader;
#[doc = "Field `EnblARMTCKFnPin` writer - Enable ARM TCK function pin"]
pub type EnblArmtckfnPinW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblARMTDIFnPin` reader - Enable ARM TDI function pin"]
pub type EnblArmtdifnPinR = crate::BitReader;
#[doc = "Field `EnblARMTDIFnPin` writer - Enable ARM TDI function pin"]
pub type EnblArmtdifnPinW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblARMTDOFnPin` reader - Enable ARM TDO function pin"]
pub type EnblArmtdofnPinR = crate::BitReader;
#[doc = "Field `EnblARMTDOFnPin` writer - Enable ARM TDO function pin"]
pub type EnblArmtdofnPinW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblARMTMSFnPin` reader - Enable ARM TMS function pin"]
pub type EnblArmtmsfnPinR = crate::BitReader;
#[doc = "Field `EnblARMTMSFnPin` writer - Enable ARM TMS function pin"]
pub type EnblArmtmsfnPinW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblSPI2CS0FnPin` reader - Enable SPI2CS0 function pin"]
pub type EnblSpi2cs0fnPinR = crate::BitReader;
#[doc = "Field `EnblSPI2CS0FnPin` writer - Enable SPI2CS0 function pin"]
pub type EnblSpi2cs0fnPinW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblSPI2CS1FnPin` reader - Enable SPI2CS1 function pin"]
pub type EnblSpi2cs1fnPinR = crate::BitReader;
#[doc = "Field `EnblSPI2CS1FnPin` writer - Enable SPI2CS1 function pin"]
pub type EnblSpi2cs1fnPinW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Reserved, must keep at value 0x0"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - Enable SGPIO Master CK function pin"]
    #[inline(always)]
    pub fn enbl_sgpiomaster_ckfn_pin(&self) -> EnblSgpiomasterCkfnPinR {
        EnblSgpiomasterCkfnPinR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable SGPIO Master LD function pin"]
    #[inline(always)]
    pub fn enbl_sgpiomaster_ldfn_pin(&self) -> EnblSgpiomasterLdfnPinR {
        EnblSgpiomasterLdfnPinR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable SGPIO Master DO function pin"]
    #[inline(always)]
    pub fn enbl_sgpiomaster_dofn_pin(&self) -> EnblSgpiomasterDofnPinR {
        EnblSgpiomasterDofnPinR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable SGPIO Master DI function pin"]
    #[inline(always)]
    pub fn enbl_sgpiomaster_difn_pin(&self) -> EnblSgpiomasterDifnPinR {
        EnblSgpiomasterDifnPinR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable ARM TRST function pin"]
    #[inline(always)]
    pub fn enbl_armtrstfn_pin(&self) -> EnblArmtrstfnPinR {
        EnblArmtrstfnPinR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable ARM TCK function pin"]
    #[inline(always)]
    pub fn enbl_armtckfn_pin(&self) -> EnblArmtckfnPinR {
        EnblArmtckfnPinR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enable ARM TDI function pin"]
    #[inline(always)]
    pub fn enbl_armtdifn_pin(&self) -> EnblArmtdifnPinR {
        EnblArmtdifnPinR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Enable ARM TDO function pin"]
    #[inline(always)]
    pub fn enbl_armtdofn_pin(&self) -> EnblArmtdofnPinR {
        EnblArmtdofnPinR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Enable ARM TMS function pin"]
    #[inline(always)]
    pub fn enbl_armtmsfn_pin(&self) -> EnblArmtmsfnPinR {
        EnblArmtmsfnPinR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Enable SPI2CS0 function pin"]
    #[inline(always)]
    pub fn enbl_spi2cs0fn_pin(&self) -> EnblSpi2cs0fnPinR {
        EnblSpi2cs0fnPinR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable SPI2CS1 function pin"]
    #[inline(always)]
    pub fn enbl_spi2cs1fn_pin(&self) -> EnblSpi2cs1fnPinR {
        EnblSpi2cs1fnPinR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Reserved, must keep at value 0x0"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<Scu41cSpec> {
        Reserved1W::new(self, 0)
    }
    #[doc = "Bit 8 - Enable SGPIO Master CK function pin"]
    #[inline(always)]
    pub fn enbl_sgpiomaster_ckfn_pin(&mut self) -> EnblSgpiomasterCkfnPinW<Scu41cSpec> {
        EnblSgpiomasterCkfnPinW::new(self, 8)
    }
    #[doc = "Bit 9 - Enable SGPIO Master LD function pin"]
    #[inline(always)]
    pub fn enbl_sgpiomaster_ldfn_pin(&mut self) -> EnblSgpiomasterLdfnPinW<Scu41cSpec> {
        EnblSgpiomasterLdfnPinW::new(self, 9)
    }
    #[doc = "Bit 10 - Enable SGPIO Master DO function pin"]
    #[inline(always)]
    pub fn enbl_sgpiomaster_dofn_pin(&mut self) -> EnblSgpiomasterDofnPinW<Scu41cSpec> {
        EnblSgpiomasterDofnPinW::new(self, 10)
    }
    #[doc = "Bit 11 - Enable SGPIO Master DI function pin"]
    #[inline(always)]
    pub fn enbl_sgpiomaster_difn_pin(&mut self) -> EnblSgpiomasterDifnPinW<Scu41cSpec> {
        EnblSgpiomasterDifnPinW::new(self, 11)
    }
    #[doc = "Bit 25 - Enable ARM TRST function pin"]
    #[inline(always)]
    pub fn enbl_armtrstfn_pin(&mut self) -> EnblArmtrstfnPinW<Scu41cSpec> {
        EnblArmtrstfnPinW::new(self, 25)
    }
    #[doc = "Bit 26 - Enable ARM TCK function pin"]
    #[inline(always)]
    pub fn enbl_armtckfn_pin(&mut self) -> EnblArmtckfnPinW<Scu41cSpec> {
        EnblArmtckfnPinW::new(self, 26)
    }
    #[doc = "Bit 27 - Enable ARM TDI function pin"]
    #[inline(always)]
    pub fn enbl_armtdifn_pin(&mut self) -> EnblArmtdifnPinW<Scu41cSpec> {
        EnblArmtdifnPinW::new(self, 27)
    }
    #[doc = "Bit 28 - Enable ARM TDO function pin"]
    #[inline(always)]
    pub fn enbl_armtdofn_pin(&mut self) -> EnblArmtdofnPinW<Scu41cSpec> {
        EnblArmtdofnPinW::new(self, 28)
    }
    #[doc = "Bit 29 - Enable ARM TMS function pin"]
    #[inline(always)]
    pub fn enbl_armtmsfn_pin(&mut self) -> EnblArmtmsfnPinW<Scu41cSpec> {
        EnblArmtmsfnPinW::new(self, 29)
    }
    #[doc = "Bit 30 - Enable SPI2CS0 function pin"]
    #[inline(always)]
    pub fn enbl_spi2cs0fn_pin(&mut self) -> EnblSpi2cs0fnPinW<Scu41cSpec> {
        EnblSpi2cs0fnPinW::new(self, 30)
    }
    #[doc = "Bit 31 - Enable SPI2CS1 function pin"]
    #[inline(always)]
    pub fn enbl_spi2cs1fn_pin(&mut self) -> EnblSpi2cs1fnPinW<Scu41cSpec> {
        EnblSpi2cs1fnPinW::new(self, 31)
    }
}
#[doc = "Multi-function Pin Control \\#4\n\nYou can [`read`](crate::Reg::read) this register and get [`scu41c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu41c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu41cSpec;
impl crate::RegisterSpec for Scu41cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu41c::R`](R) reader structure"]
impl crate::Readable for Scu41cSpec {}
#[doc = "`write(|w| ..)` method takes [`scu41c::W`](W) writer structure"]
impl crate::Writable for Scu41cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU41C to value 0x000f_f000"]
impl crate::Resettable for Scu41cSpec {
    const RESET_VALUE: u32 = 0x000f_f000;
}
