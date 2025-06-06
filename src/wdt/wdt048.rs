#[doc = "Register `WDT048` reader"]
pub type R = crate::R<Wdt048Spec>;
#[doc = "Register `WDT048` writer"]
pub type W = crate::W<Wdt048Spec>;
#[doc = "Field `EnblWrProtOfWDT28n` reader - Enable Write Protection of WDT28\\[n\\]"]
pub type EnblWrProtOfWdt28nR = crate::FieldReader<u32>;
#[doc = "Field `EnblWrProtOfWDT28n` writer - Enable Write Protection of WDT28\\[n\\]"]
pub type EnblWrProtOfWdt28nW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Enable Write Protection of WDT28\\[n\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_wdt28n(&self) -> EnblWrProtOfWdt28nR {
        EnblWrProtOfWdt28nR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Enable Write Protection of WDT28\\[n\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_wdt28n(&mut self) -> EnblWrProtOfWdt28nW<Wdt048Spec> {
        EnblWrProtOfWdt28nW::new(self, 0)
    }
}
#[doc = "WDTn Software Mode Reset Mask Write Protection Register \\#1\n\nYou can [`read`](crate::Reg::read) this register and get [`wdt048::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdt048::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Wdt048Spec;
impl crate::RegisterSpec for Wdt048Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdt048::R`](R) reader structure"]
impl crate::Readable for Wdt048Spec {}
#[doc = "`write(|w| ..)` method takes [`wdt048::W`](W) writer structure"]
impl crate::Writable for Wdt048Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WDT048 to value 0"]
impl crate::Resettable for Wdt048Spec {}
