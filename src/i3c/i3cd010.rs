#[doc = "Register `I3CD010` reader"]
pub type R = crate::R<I3cd010Spec>;
#[doc = "Register `I3CD010` writer"]
pub type W = crate::W<I3cd010Spec>;
#[doc = "Field `DL` reader - DL"]
pub type DlR = crate::FieldReader<u16>;
#[doc = "Field `CCCT` reader - CCCT"]
pub type CcctR = crate::FieldReader;
#[doc = "Field `TID` reader - TID"]
pub type TidR = crate::FieldReader;
#[doc = "Field `ERRSTATUS` reader - ERR_STATUS"]
pub type ErrstatusR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:15 - DL"]
    #[inline(always)]
    pub fn dl(&self) -> DlR {
        DlR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - CCCT"]
    #[inline(always)]
    pub fn ccct(&self) -> CcctR {
        CcctR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:27 - TID"]
    #[inline(always)]
    pub fn tid(&self) -> TidR {
        TidR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - ERR_STATUS"]
    #[inline(always)]
    pub fn errstatus(&self) -> ErrstatusR {
        ErrstatusR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {}
#[doc = "Response Queue\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cd010::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cd010::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3cd010Spec;
impl crate::RegisterSpec for I3cd010Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3cd010::R`](R) reader structure"]
impl crate::Readable for I3cd010Spec {}
#[doc = "`write(|w| ..)` method takes [`i3cd010::W`](W) writer structure"]
impl crate::Writable for I3cd010Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CD010 to value 0"]
impl crate::Resettable for I3cd010Spec {}
