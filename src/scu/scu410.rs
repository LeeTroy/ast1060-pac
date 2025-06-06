#[doc = "Register `SCU410` reader"]
pub type R = crate::R<Scu410Spec>;
#[doc = "Register `SCU410` writer"]
pub type W = crate::W<Scu410Spec>;
impl W {}
#[doc = "Multi-function Pin Control \\#1\n\nYou can [`read`](crate::Reg::read) this register and get [`scu410::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu410::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu410Spec;
impl crate::RegisterSpec for Scu410Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu410::R`](R) reader structure"]
impl crate::Readable for Scu410Spec {}
#[doc = "`write(|w| ..)` method takes [`scu410::W`](W) writer structure"]
impl crate::Writable for Scu410Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU410 to value 0"]
impl crate::Resettable for Scu410Spec {}
