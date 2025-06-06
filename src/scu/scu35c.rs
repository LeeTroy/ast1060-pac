#[doc = "Register `SCU35C` reader"]
pub type R = crate::R<Scu35cSpec>;
#[doc = "Register `SCU35C` writer"]
pub type W = crate::W<Scu35cSpec>;
#[doc = "Field `MACRGMIITXCLK10MClkOutputDelay` reader - MAC RGMII_TXCLK 10M clock output delay"]
pub type Macrgmiitxclk10mclkOutputDelayR = crate::FieldReader;
#[doc = "Field `MACRGMIITXCLK10MClkOutputDelay` writer - MAC RGMII_TXCLK 10M clock output delay"]
pub type Macrgmiitxclk10mclkOutputDelayW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `Reserved3` reader - Reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `Reserved3` writer - Reserved"]
pub type Reserved3W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `MACRGMIIRXCLK10MClkInputDelay` reader - MAC RGMII_RXCLK 10M clock input delay"]
pub type Macrgmiirxclk10mclkInputDelayR = crate::FieldReader;
#[doc = "Field `MACRGMIIRXCLK10MClkInputDelay` writer - MAC RGMII_RXCLK 10M clock input delay"]
pub type Macrgmiirxclk10mclkInputDelayW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `Reserved2` reader - Reserved"]
pub type Reserved2R = crate::FieldReader;
#[doc = "Field `Reserved2` writer - Reserved"]
pub type Reserved2W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `MACRXCLK10MClkTreeInverePhase` reader - MAC RXCLK 10M clock tree invere phase"]
pub type Macrxclk10mclkTreeInverePhaseR = crate::BitReader;
#[doc = "Field `MACRXCLK10MClkTreeInverePhase` writer - MAC RXCLK 10M clock tree invere phase"]
pub type Macrxclk10mclkTreeInverePhaseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `Reserved1` writer - Reserved"]
pub type Reserved1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - MAC RGMII_TXCLK 10M clock output delay"]
    #[inline(always)]
    pub fn macrgmiitxclk10mclk_output_delay(&self) -> Macrgmiitxclk10mclkOutputDelayR {
        Macrgmiitxclk10mclkOutputDelayR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:11 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 6) & 0x3f) as u8)
    }
    #[doc = "Bits 12:17 - MAC RGMII_RXCLK 10M clock input delay"]
    #[inline(always)]
    pub fn macrgmiirxclk10mclk_input_delay(&self) -> Macrgmiirxclk10mclkInputDelayR {
        Macrgmiirxclk10mclkInputDelayR::new(((self.bits >> 12) & 0x3f) as u8)
    }
    #[doc = "Bits 18:23 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 18) & 0x3f) as u8)
    }
    #[doc = "Bit 24 - MAC RXCLK 10M clock tree invere phase"]
    #[inline(always)]
    pub fn macrxclk10mclk_tree_invere_phase(&self) -> Macrxclk10mclkTreeInverePhaseR {
        Macrxclk10mclkTreeInverePhaseR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - MAC RGMII_TXCLK 10M clock output delay"]
    #[inline(always)]
    pub fn macrgmiitxclk10mclk_output_delay(
        &mut self,
    ) -> Macrgmiitxclk10mclkOutputDelayW<Scu35cSpec> {
        Macrgmiitxclk10mclkOutputDelayW::new(self, 0)
    }
    #[doc = "Bits 6:11 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&mut self) -> Reserved3W<Scu35cSpec> {
        Reserved3W::new(self, 6)
    }
    #[doc = "Bits 12:17 - MAC RGMII_RXCLK 10M clock input delay"]
    #[inline(always)]
    pub fn macrgmiirxclk10mclk_input_delay(
        &mut self,
    ) -> Macrgmiirxclk10mclkInputDelayW<Scu35cSpec> {
        Macrgmiirxclk10mclkInputDelayW::new(self, 12)
    }
    #[doc = "Bits 18:23 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&mut self) -> Reserved2W<Scu35cSpec> {
        Reserved2W::new(self, 18)
    }
    #[doc = "Bit 24 - MAC RXCLK 10M clock tree invere phase"]
    #[inline(always)]
    pub fn macrxclk10mclk_tree_invere_phase(
        &mut self,
    ) -> Macrxclk10mclkTreeInverePhaseW<Scu35cSpec> {
        Macrxclk10mclkTreeInverePhaseW::new(self, 24)
    }
    #[doc = "Bit 25 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<Scu35cSpec> {
        Reserved1W::new(self, 25)
    }
}
#[doc = "MAC Interface Clock Delay 10M Setting\n\nYou can [`read`](crate::Reg::read) this register and get [`scu35c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu35c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu35cSpec;
impl crate::RegisterSpec for Scu35cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu35c::R`](R) reader structure"]
impl crate::Readable for Scu35cSpec {}
#[doc = "`write(|w| ..)` method takes [`scu35c::W`](W) writer structure"]
impl crate::Writable for Scu35cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU35C to value 0"]
impl crate::Resettable for Scu35cSpec {}
