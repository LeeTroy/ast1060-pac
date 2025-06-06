#[doc = "Register `SCU530` reader"]
pub type R = crate::R<Scu530Spec>;
#[doc = "Register `SCU530` writer"]
pub type W = crate::W<Scu530Spec>;
#[doc = "Field `RandomNumberGeneratorDisableCtrl` reader - Random number generator disable control"]
pub type RandomNumberGeneratorDisableCtrlR = crate::BitReader;
#[doc = "Field `RandomNumberGeneratorDisableCtrl` writer - Random number generator disable control"]
pub type RandomNumberGeneratorDisableCtrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RandomNumberGeneratorModeSel` reader - Random number generator mode selection"]
pub type RandomNumberGeneratorModeSelR = crate::FieldReader;
#[doc = "Field `RandomNumberGeneratorModeSel` writer - Random number generator mode selection"]
pub type RandomNumberGeneratorModeSelW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bit 0 - Random number generator disable control"]
    #[inline(always)]
    pub fn random_number_generator_disable_ctrl(&self) -> RandomNumberGeneratorDisableCtrlR {
        RandomNumberGeneratorDisableCtrlR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:5 - Random number generator mode selection"]
    #[inline(always)]
    pub fn random_number_generator_mode_sel(&self) -> RandomNumberGeneratorModeSelR {
        RandomNumberGeneratorModeSelR::new(((self.bits >> 1) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Random number generator disable control"]
    #[inline(always)]
    pub fn random_number_generator_disable_ctrl(
        &mut self,
    ) -> RandomNumberGeneratorDisableCtrlW<Scu530Spec> {
        RandomNumberGeneratorDisableCtrlW::new(self, 0)
    }
    #[doc = "Bits 1:5 - Random number generator mode selection"]
    #[inline(always)]
    pub fn random_number_generator_mode_sel(
        &mut self,
    ) -> RandomNumberGeneratorModeSelW<Scu530Spec> {
        RandomNumberGeneratorModeSelW::new(self, 1)
    }
}
#[doc = "Random Number Generator 2 Control\n\nYou can [`read`](crate::Reg::read) this register and get [`scu530::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu530::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu530Spec;
impl crate::RegisterSpec for Scu530Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu530::R`](R) reader structure"]
impl crate::Readable for Scu530Spec {}
#[doc = "`write(|w| ..)` method takes [`scu530::W`](W) writer structure"]
impl crate::Writable for Scu530Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU530 to value 0x0e"]
impl crate::Resettable for Scu530Spec {
    const RESET_VALUE: u32 = 0x0e;
}
