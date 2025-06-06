#[doc = "Register `SCU6D0` reader"]
pub type R = crate::R<Scu6d0Spec>;
#[doc = "Register `SCU6D0` writer"]
pub type W = crate::W<Scu6d0Spec>;
impl W {}
#[doc = "Multi-function Pin Control \\#29\n\nYou can [`read`](crate::Reg::read) this register and get [`scu6d0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu6d0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu6d0Spec;
impl crate::RegisterSpec for Scu6d0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu6d0::R`](R) reader structure"]
impl crate::Readable for Scu6d0Spec {}
#[doc = "`write(|w| ..)` method takes [`scu6d0::W`](W) writer structure"]
impl crate::Writable for Scu6d0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU6D0 to value 0"]
impl crate::Resettable for Scu6d0Spec {}
