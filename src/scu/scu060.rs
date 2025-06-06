#[doc = "Register `SCU060` reader"]
pub type R = crate::R<Scu060Spec>;
#[doc = "Register `SCU060` writer"]
pub type W = crate::W<Scu060Spec>;
#[doc = "Field `EnblRstARMThisBitIsAlways1` reader - Enable reset ARM. This bit is always 1."]
pub type EnblRstArmthisBitIsAlways1R = crate::BitReader;
#[doc = "Field `EnblRstSRAMCtrl` reader - Enable reset SRAM controller"]
pub type EnblRstSramctrlR = crate::BitReader;
#[doc = "Field `EnblRstSRAMCtrl` writer - Enable reset SRAM controller"]
pub type EnblRstSramctrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblRstAHBBridgesThisBitMustBeSameAsSCU702` reader - Enable reset AHB bridges. This bit must be same as SCU70\\[2\\]."]
pub type EnblRstAhbbridgesThisBitMustBeSameAsScu702R = crate::BitReader;
#[doc = "Field `EnblRstAHBBridgesThisBitMustBeSameAsSCU702` writer - Enable reset AHB bridges. This bit must be same as SCU70\\[2\\]."]
pub type EnblRstAhbbridgesThisBitMustBeSameAsScu702W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - Reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `EnblRstSOCCtrl` reader - Enable reset SOC controller"]
pub type EnblRstSocctrlR = crate::BitReader;
#[doc = "Field `EnblRstSOCCtrl` writer - Enable reset SOC controller"]
pub type EnblRstSocctrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - Reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `Reserved2` reader - Reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `Reserved2` writer - Reserved"]
pub type Reserved2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `EnblRstHACEng` reader - Enable reset HAC engine"]
pub type EnblRstHacengR = crate::BitReader;
#[doc = "Field `EnblRstHACEng` writer - Enable reset HAC engine"]
pub type EnblRstHacengW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable reset ARM. This bit is always 1."]
    #[inline(always)]
    pub fn enbl_rst_armthis_bit_is_always1(&self) -> EnblRstArmthisBitIsAlways1R {
        EnblRstArmthisBitIsAlways1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable reset SRAM controller"]
    #[inline(always)]
    pub fn enbl_rst_sramctrl(&self) -> EnblRstSramctrlR {
        EnblRstSramctrlR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable reset AHB bridges. This bit must be same as SCU70\\[2\\]."]
    #[inline(always)]
    pub fn enbl_rst_ahbbridges_this_bit_must_be_same_as_scu702(
        &self,
    ) -> EnblRstAhbbridgesThisBitMustBeSameAsScu702R {
        EnblRstAhbbridgesThisBitMustBeSameAsScu702R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable reset SOC controller"]
    #[inline(always)]
    pub fn enbl_rst_socctrl(&self) -> EnblRstSocctrlR {
        EnblRstSocctrlR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:14 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 15 - Enable reset HAC engine"]
    #[inline(always)]
    pub fn enbl_rst_haceng(&self) -> EnblRstHacengR {
        EnblRstHacengR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Enable reset SRAM controller"]
    #[inline(always)]
    pub fn enbl_rst_sramctrl(&mut self) -> EnblRstSramctrlW<Scu060Spec> {
        EnblRstSramctrlW::new(self, 1)
    }
    #[doc = "Bit 2 - Enable reset AHB bridges. This bit must be same as SCU70\\[2\\]."]
    #[inline(always)]
    pub fn enbl_rst_ahbbridges_this_bit_must_be_same_as_scu702(
        &mut self,
    ) -> EnblRstAhbbridgesThisBitMustBeSameAsScu702W<Scu060Spec> {
        EnblRstAhbbridgesThisBitMustBeSameAsScu702W::new(self, 2)
    }
    #[doc = "Bit 4 - Enable reset SOC controller"]
    #[inline(always)]
    pub fn enbl_rst_socctrl(&mut self) -> EnblRstSocctrlW<Scu060Spec> {
        EnblRstSocctrlW::new(self, 4)
    }
    #[doc = "Bit 7 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&mut self) -> Reserved2W<Scu060Spec> {
        Reserved2W::new(self, 7)
    }
    #[doc = "Bit 15 - Enable reset HAC engine"]
    #[inline(always)]
    pub fn enbl_rst_haceng(&mut self) -> EnblRstHacengW<Scu060Spec> {
        EnblRstHacengW::new(self, 15)
    }
}
#[doc = "EXTRST\\# Reset Selection\n\nYou can [`read`](crate::Reg::read) this register and get [`scu060::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu060::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu060Spec;
impl crate::RegisterSpec for Scu060Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu060::R`](R) reader structure"]
impl crate::Readable for Scu060Spec {}
#[doc = "`write(|w| ..)` method takes [`scu060::W`](W) writer structure"]
impl crate::Writable for Scu060Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU060 to value 0x070f_1ff1"]
impl crate::Resettable for Scu060Spec {
    const RESET_VALUE: u32 = 0x070f_1ff1;
}
