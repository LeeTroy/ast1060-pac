#[doc = "Register `WDT044` reader"]
pub type R = crate::R<Wdt044Spec>;
#[doc = "Register `WDT044` writer"]
pub type W = crate::W<Wdt044Spec>;
#[doc = "Field `EnblWrProtOfWDT20n` reader - Enable Write Protection of WDT20\\[n\\]"]
pub type EnblWrProtOfWdt20nR = crate::FieldReader<u32>;
#[doc = "Field `EnblWrProtOfWDT20n` writer - Enable Write Protection of WDT20\\[n\\]"]
pub type EnblWrProtOfWdt20nW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Enable Write Protection of WDT20\\[n\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_wdt20n(&self) -> EnblWrProtOfWdt20nR {
        EnblWrProtOfWdt20nR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Enable Write Protection of WDT20\\[n\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_wdt20n(&mut self) -> EnblWrProtOfWdt20nW<Wdt044Spec> {
        EnblWrProtOfWdt20nW::new(self, 0)
    }
}
#[doc = "WDTn Reset Mask Write Protection Register \\#2\n\nYou can [`read`](crate::Reg::read) this register and get [`wdt044::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdt044::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Wdt044Spec;
impl crate::RegisterSpec for Wdt044Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdt044::R`](R) reader structure"]
impl crate::Readable for Wdt044Spec {}
#[doc = "`write(|w| ..)` method takes [`wdt044::W`](W) writer structure"]
impl crate::Writable for Wdt044Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WDT044 to value 0"]
impl crate::Resettable for Wdt044Spec {}
