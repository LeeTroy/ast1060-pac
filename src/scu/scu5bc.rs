#[doc = "Register `SCU5BC` reader"]
pub type R = crate::R<Scu5bcSpec>;
#[doc = "Register `SCU5BC` writer"]
pub type W = crate::W<Scu5bcSpec>;
#[doc = "Field `Reserved1` reader - 0] read back"]
pub type Reserved1R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - 0] read back"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(self.bits)
    }
}
impl W {}
#[doc = "Reserved Read Only ID 1\n\nYou can [`read`](crate::Reg::read) this register and get [`scu5bc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu5bc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu5bcSpec;
impl crate::RegisterSpec for Scu5bcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu5bc::R`](R) reader structure"]
impl crate::Readable for Scu5bcSpec {}
#[doc = "`write(|w| ..)` method takes [`scu5bc::W`](W) writer structure"]
impl crate::Writable for Scu5bcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU5BC to value 0"]
impl crate::Resettable for Scu5bcSpec {}
