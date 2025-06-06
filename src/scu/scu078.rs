#[doc = "Register `SCU078` reader"]
pub type R = crate::R<Scu078Spec>;
#[doc = "Register `SCU078` writer"]
pub type W = crate::W<Scu078Spec>;
#[doc = "Field `GPIOPRSTNGPIORst` reader - Reset event log -- GPIO_PRST_N GPIO reset"]
pub type GpioprstngpiorstR = crate::BitReader;
#[doc = "Field `GPIOPRSTNGPIORst` writer - Reset event log -- GPIO_PRST_N GPIO reset"]
pub type GpioprstngpiorstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JTAGPRST1NJTAGRst` reader - Reset event log -- JTAG_PRST#1_N JTAG reset"]
pub type Jtagprst1njtagrstR = crate::BitReader;
#[doc = "Field `JTAGPRST1NJTAGRst` writer - Reset event log -- JTAG_PRST#1_N JTAG reset"]
pub type Jtagprst1njtagrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCPRSTNADCRst` reader - Reset event log -- ADC_PRST_N ADC reset"]
pub type AdcprstnadcrstR = crate::BitReader;
#[doc = "Field `ADCPRSTNADCRst` writer - Reset event log -- ADC_PRST_N ADC reset"]
pub type AdcprstnadcrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AESPRSTNHashCryptoEngRst` reader - Reset event log -- AES_PRST_N Hash crypto engine reset"]
pub type AesprstnhashCryptoEngRstR = crate::BitReader;
#[doc = "Field `AESPRSTNHashCryptoEngRst` writer - Reset event log -- AES_PRST_N Hash crypto engine reset"]
pub type AesprstnhashCryptoEngRstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2CPRSTNI2CRst` reader - Reset event log -- I2C_PRST_N I2C reset"]
pub type I2cprstni2crstR = crate::BitReader;
#[doc = "Field `I2CPRSTNI2CRst` writer - Reset event log -- I2C_PRST_N I2C reset"]
pub type I2cprstni2crstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved10` reader - Reserved"]
pub type Reserved10R = crate::FieldReader;
#[doc = "Field `Reserved10` writer - Reserved"]
pub type Reserved10W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RstpltrstnPlatformReset` reader - Reset event log -- rstpltrstn Platform Reset"]
pub type RstpltrstnPlatformResetR = crate::BitReader;
#[doc = "Field `RstpltrstnPlatformReset` writer - Reset event log -- rstpltrstn Platform Reset"]
pub type RstpltrstnPlatformResetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved9` reader - Reserved"]
pub type Reserved9R = crate::FieldReader;
#[doc = "Field `Reserved9` writer - Reserved"]
pub type Reserved9W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `JTAGPRST2NJTAGRst` reader - Reset event log -- JTAG_PRST#2_N JTAG reset"]
pub type Jtagprst2njtagrstR = crate::BitReader;
#[doc = "Field `JTAGPRST2NJTAGRst` writer - Reset event log -- JTAG_PRST#2_N JTAG reset"]
pub type Jtagprst2njtagrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved8` reader - Reserved"]
pub type Reserved8R = crate::BitReader;
#[doc = "Field `Reserved8` writer - Reserved"]
pub type Reserved8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I3CPRSTNI3CDMARst` reader - Reset event log -- I3C_PRST_N I3C DMA reset"]
pub type I3cprstni3cdmarstR = crate::BitReader;
#[doc = "Field `I3CPRSTNI3CDMARst` writer - Reset event log -- I3C_PRST_N I3C DMA reset"]
pub type I3cprstni3cdmarstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I3C0HRSTNI3C1Rst` reader - Reset event log -- I3C0_HRST_N I3C #1 reset"]
pub type I3c0hrstni3c1rstR = crate::BitReader;
#[doc = "Field `I3C0HRSTNI3C1Rst` writer - Reset event log -- I3C0_HRST_N I3C #1 reset"]
pub type I3c0hrstni3c1rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I3C1HRSTNI3C2Rst` reader - Reset event log -- I3C1_HRST_N I3C #2 reset"]
pub type I3c1hrstni3c2rstR = crate::BitReader;
#[doc = "Field `I3C1HRSTNI3C2Rst` writer - Reset event log -- I3C1_HRST_N I3C #2 reset"]
pub type I3c1hrstni3c2rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I3C2HRSTNI3C3Rst` reader - Reset event log -- I3C2_HRST_N I3C #3 reset"]
pub type I3c2hrstni3c3rstR = crate::BitReader;
#[doc = "Field `I3C2HRSTNI3C3Rst` writer - Reset event log -- I3C2_HRST_N I3C #3 reset"]
pub type I3c2hrstni3c3rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I3C3HRSTNI3C4Rst` reader - Reset event log -- I3C3_HRST_N I3C #4 reset"]
pub type I3c3hrstni3c4rstR = crate::BitReader;
#[doc = "Field `I3C3HRSTNI3C4Rst` writer - Reset event log -- I3C3_HRST_N I3C #4 reset"]
pub type I3c3hrstni3c4rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved7` reader - Reserved"]
pub type Reserved7R = crate::BitReader;
#[doc = "Field `Reserved6` reader - Reserved"]
pub type Reserved6R = crate::BitReader;
#[doc = "Field `Reserved5` reader - Reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `Reserved4` reader - Reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `Reserved3` reader - Reserved"]
pub type Reserved3R = crate::BitReader;
#[doc = "Field `Reserved3` writer - Reserved"]
pub type Reserved3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - Reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `SPIHRSTNSPICtrlRst` reader - Reset event log -- SPI_HRST_N SPI Controller reset"]
pub type SpihrstnspictrlRstR = crate::BitReader;
#[doc = "Field `SPIHRSTNSPICtrlRst` writer - Reset event log -- SPI_HRST_N SPI Controller reset"]
pub type SpihrstnspictrlRstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `Reserved1` writer - Reserved"]
pub type Reserved1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Reset event log -- GPIO_PRST_N GPIO reset"]
    #[inline(always)]
    pub fn gpioprstngpiorst(&self) -> GpioprstngpiorstR {
        GpioprstngpiorstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Reset event log -- JTAG_PRST#1_N JTAG reset"]
    #[inline(always)]
    pub fn jtagprst1njtagrst(&self) -> Jtagprst1njtagrstR {
        Jtagprst1njtagrstR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Reset event log -- ADC_PRST_N ADC reset"]
    #[inline(always)]
    pub fn adcprstnadcrst(&self) -> AdcprstnadcrstR {
        AdcprstnadcrstR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Reset event log -- AES_PRST_N Hash crypto engine reset"]
    #[inline(always)]
    pub fn aesprstnhash_crypto_eng_rst(&self) -> AesprstnhashCryptoEngRstR {
        AesprstnhashCryptoEngRstR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Reset event log -- I2C_PRST_N I2C reset"]
    #[inline(always)]
    pub fn i2cprstni2crst(&self) -> I2cprstni2crstR {
        I2cprstni2crstR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - Reserved"]
    #[inline(always)]
    pub fn reserved10(&self) -> Reserved10R {
        Reserved10R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - Reset event log -- rstpltrstn Platform Reset"]
    #[inline(always)]
    pub fn rstpltrstn_platform_reset(&self) -> RstpltrstnPlatformResetR {
        RstpltrstnPlatformResetR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:12 - Reserved"]
    #[inline(always)]
    pub fn reserved9(&self) -> Reserved9R {
        Reserved9R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 13 - Reset event log -- JTAG_PRST#2_N JTAG reset"]
    #[inline(always)]
    pub fn jtagprst2njtagrst(&self) -> Jtagprst2njtagrstR {
        Jtagprst2njtagrstR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Reserved"]
    #[inline(always)]
    pub fn reserved8(&self) -> Reserved8R {
        Reserved8R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Reset event log -- I3C_PRST_N I3C DMA reset"]
    #[inline(always)]
    pub fn i3cprstni3cdmarst(&self) -> I3cprstni3cdmarstR {
        I3cprstni3cdmarstR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Reset event log -- I3C0_HRST_N I3C #1 reset"]
    #[inline(always)]
    pub fn i3c0hrstni3c1rst(&self) -> I3c0hrstni3c1rstR {
        I3c0hrstni3c1rstR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Reset event log -- I3C1_HRST_N I3C #2 reset"]
    #[inline(always)]
    pub fn i3c1hrstni3c2rst(&self) -> I3c1hrstni3c2rstR {
        I3c1hrstni3c2rstR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Reset event log -- I3C2_HRST_N I3C #3 reset"]
    #[inline(always)]
    pub fn i3c2hrstni3c3rst(&self) -> I3c2hrstni3c3rstR {
        I3c2hrstni3c3rstR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Reset event log -- I3C3_HRST_N I3C #4 reset"]
    #[inline(always)]
    pub fn i3c3hrstni3c4rst(&self) -> I3c3hrstni3c4rstR {
        I3c3hrstni3c4rstR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Reserved"]
    #[inline(always)]
    pub fn reserved7(&self) -> Reserved7R {
        Reserved7R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Reserved"]
    #[inline(always)]
    pub fn reserved6(&self) -> Reserved6R {
        Reserved6R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Reset event log -- SPI_HRST_N SPI Controller reset"]
    #[inline(always)]
    pub fn spihrstnspictrl_rst(&self) -> SpihrstnspictrlRstR {
        SpihrstnspictrlRstR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Reset event log -- GPIO_PRST_N GPIO reset"]
    #[inline(always)]
    pub fn gpioprstngpiorst(&mut self) -> GpioprstngpiorstW<Scu078Spec> {
        GpioprstngpiorstW::new(self, 0)
    }
    #[doc = "Bit 1 - Reset event log -- JTAG_PRST#1_N JTAG reset"]
    #[inline(always)]
    pub fn jtagprst1njtagrst(&mut self) -> Jtagprst1njtagrstW<Scu078Spec> {
        Jtagprst1njtagrstW::new(self, 1)
    }
    #[doc = "Bit 2 - Reset event log -- ADC_PRST_N ADC reset"]
    #[inline(always)]
    pub fn adcprstnadcrst(&mut self) -> AdcprstnadcrstW<Scu078Spec> {
        AdcprstnadcrstW::new(self, 2)
    }
    #[doc = "Bit 3 - Reset event log -- AES_PRST_N Hash crypto engine reset"]
    #[inline(always)]
    pub fn aesprstnhash_crypto_eng_rst(&mut self) -> AesprstnhashCryptoEngRstW<Scu078Spec> {
        AesprstnhashCryptoEngRstW::new(self, 3)
    }
    #[doc = "Bit 4 - Reset event log -- I2C_PRST_N I2C reset"]
    #[inline(always)]
    pub fn i2cprstni2crst(&mut self) -> I2cprstni2crstW<Scu078Spec> {
        I2cprstni2crstW::new(self, 4)
    }
    #[doc = "Bits 5:6 - Reserved"]
    #[inline(always)]
    pub fn reserved10(&mut self) -> Reserved10W<Scu078Spec> {
        Reserved10W::new(self, 5)
    }
    #[doc = "Bit 7 - Reset event log -- rstpltrstn Platform Reset"]
    #[inline(always)]
    pub fn rstpltrstn_platform_reset(&mut self) -> RstpltrstnPlatformResetW<Scu078Spec> {
        RstpltrstnPlatformResetW::new(self, 7)
    }
    #[doc = "Bits 8:12 - Reserved"]
    #[inline(always)]
    pub fn reserved9(&mut self) -> Reserved9W<Scu078Spec> {
        Reserved9W::new(self, 8)
    }
    #[doc = "Bit 13 - Reset event log -- JTAG_PRST#2_N JTAG reset"]
    #[inline(always)]
    pub fn jtagprst2njtagrst(&mut self) -> Jtagprst2njtagrstW<Scu078Spec> {
        Jtagprst2njtagrstW::new(self, 13)
    }
    #[doc = "Bit 14 - Reserved"]
    #[inline(always)]
    pub fn reserved8(&mut self) -> Reserved8W<Scu078Spec> {
        Reserved8W::new(self, 14)
    }
    #[doc = "Bit 15 - Reset event log -- I3C_PRST_N I3C DMA reset"]
    #[inline(always)]
    pub fn i3cprstni3cdmarst(&mut self) -> I3cprstni3cdmarstW<Scu078Spec> {
        I3cprstni3cdmarstW::new(self, 15)
    }
    #[doc = "Bit 16 - Reset event log -- I3C0_HRST_N I3C #1 reset"]
    #[inline(always)]
    pub fn i3c0hrstni3c1rst(&mut self) -> I3c0hrstni3c1rstW<Scu078Spec> {
        I3c0hrstni3c1rstW::new(self, 16)
    }
    #[doc = "Bit 17 - Reset event log -- I3C1_HRST_N I3C #2 reset"]
    #[inline(always)]
    pub fn i3c1hrstni3c2rst(&mut self) -> I3c1hrstni3c2rstW<Scu078Spec> {
        I3c1hrstni3c2rstW::new(self, 17)
    }
    #[doc = "Bit 18 - Reset event log -- I3C2_HRST_N I3C #3 reset"]
    #[inline(always)]
    pub fn i3c2hrstni3c3rst(&mut self) -> I3c2hrstni3c3rstW<Scu078Spec> {
        I3c2hrstni3c3rstW::new(self, 18)
    }
    #[doc = "Bit 19 - Reset event log -- I3C3_HRST_N I3C #4 reset"]
    #[inline(always)]
    pub fn i3c3hrstni3c4rst(&mut self) -> I3c3hrstni3c4rstW<Scu078Spec> {
        I3c3hrstni3c4rstW::new(self, 19)
    }
    #[doc = "Bit 24 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&mut self) -> Reserved3W<Scu078Spec> {
        Reserved3W::new(self, 24)
    }
    #[doc = "Bit 26 - Reset event log -- SPI_HRST_N SPI Controller reset"]
    #[inline(always)]
    pub fn spihrstnspictrl_rst(&mut self) -> SpihrstnspictrlRstW<Scu078Spec> {
        SpihrstnspictrlRstW::new(self, 26)
    }
    #[doc = "Bit 27 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<Scu078Spec> {
        Reserved1W::new(self, 27)
    }
}
#[doc = "System Reset Event Log Register Set 2-2\n\nYou can [`read`](crate::Reg::read) this register and get [`scu078::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu078::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu078Spec;
impl crate::RegisterSpec for Scu078Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu078::R`](R) reader structure"]
impl crate::Readable for Scu078Spec {}
#[doc = "`write(|w| ..)` method takes [`scu078::W`](W) writer structure"]
impl crate::Writable for Scu078Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU078 to value 0x07ff_ffff"]
impl crate::Resettable for Scu078Spec {
    const RESET_VALUE: u32 = 0x07ff_ffff;
}
