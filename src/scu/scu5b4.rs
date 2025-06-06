#[doc = "Register `SCU5B4` reader"]
pub type R = crate::R<Scu5b4Spec>;
#[doc = "Register `SCU5B4` writer"]
pub type W = crate::W<Scu5b4Spec>;
#[doc = "Field `ChipUniqueIDBit631` reader - 0] read back"]
pub type ChipUniqueIdbit631R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - 0] read back"]
    #[inline(always)]
    pub fn chip_unique_idbit631(&self) -> ChipUniqueIdbit631R {
        ChipUniqueIdbit631R::new(self.bits)
    }
}
impl W {}
#[doc = "Chip Unique ID 1\n\nYou can [`read`](crate::Reg::read) this register and get [`scu5b4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu5b4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu5b4Spec;
impl crate::RegisterSpec for Scu5b4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu5b4::R`](R) reader structure"]
impl crate::Readable for Scu5b4Spec {}
#[doc = "`write(|w| ..)` method takes [`scu5b4::W`](W) writer structure"]
impl crate::Writable for Scu5b4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU5B4 to value 0"]
impl crate::Resettable for Scu5b4Spec {}
