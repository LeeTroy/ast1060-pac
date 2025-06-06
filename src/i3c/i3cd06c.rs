#[doc = "Register `I3CD06C` reader"]
pub type R = crate::R<I3cd06cSpec>;
#[doc = "Register `I3CD06C` writer"]
pub type W = crate::W<I3cd06cSpec>;
#[doc = "Field `PVENDORREGSTARTADDR` reader - P_VENDOR_REG_START_ADDR"]
pub type PvendorregstartaddrR = crate::FieldReader<u16>;
#[doc = "Field `RSVD` reader - reserved"]
pub type RsvdR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - P_VENDOR_REG_START_ADDR"]
    #[inline(always)]
    pub fn pvendorregstartaddr(&self) -> PvendorregstartaddrR {
        PvendorregstartaddrR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - reserved"]
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {}
#[doc = "Pointer for Vendor specific Registers.\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cd06c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cd06c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3cd06cSpec;
impl crate::RegisterSpec for I3cd06cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3cd06c::R`](R) reader structure"]
impl crate::Readable for I3cd06cSpec {}
#[doc = "`write(|w| ..)` method takes [`i3cd06c::W`](W) writer structure"]
impl crate::Writable for I3cd06cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CD06C to value 0"]
impl crate::Resettable for I3cd06cSpec {}
