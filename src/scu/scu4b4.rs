#[doc = "Register `SCU4B4` reader"]
pub type R = crate::R<Scu4b4Spec>;
#[doc = "Register `SCU4B4` writer"]
pub type W = crate::W<Scu4b4Spec>;
impl W {}
#[doc = "Multi-function Pin Control \\#14\n\nYou can [`read`](crate::Reg::read) this register and get [`scu4b4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu4b4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu4b4Spec;
impl crate::RegisterSpec for Scu4b4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu4b4::R`](R) reader structure"]
impl crate::Readable for Scu4b4Spec {}
#[doc = "`write(|w| ..)` method takes [`scu4b4::W`](W) writer structure"]
impl crate::Writable for Scu4b4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU4B4 to value 0"]
impl crate::Resettable for Scu4b4Spec {}
