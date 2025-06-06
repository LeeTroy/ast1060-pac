#[doc = "Register `I3CD22C` reader"]
pub type R = crate::R<I3cd22cSpec>;
#[doc = "Register `I3CD22C` writer"]
pub type W = crate::W<I3cd22cSpec>;
#[doc = "Field `DEVDYNAMICADDR` reader - DEV_DYNAMIC_ADDR"]
pub type DevdynamicaddrR = crate::FieldReader;
#[doc = "Field `DYNAMICADDR` reader - DYNAMIC_ADDR"]
pub type DynamicaddrR = crate::FieldReader;
#[doc = "Field `RSVD` reader - reserved"]
pub type RsvdR = crate::FieldReader<u32>;
#[doc = "Field `DCRTYPE` reader - DCR_TYPE"]
pub type DcrtypeR = crate::FieldReader;
#[doc = "Field `BCRTYPE` reader - BCR_TYPE"]
pub type BcrtypeR = crate::FieldReader;
#[doc = "Field `STATICADDR` reader - STATIC_ADDR"]
pub type StaticaddrR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - DEV_DYNAMIC_ADDR"]
    #[inline(always)]
    pub fn devdynamicaddr(&self) -> DevdynamicaddrR {
        DevdynamicaddrR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - DYNAMIC_ADDR"]
    #[inline(always)]
    pub fn dynamicaddr(&self) -> DynamicaddrR {
        DynamicaddrR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - reserved"]
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new((self.bits >> 8) & 0x00ff_ffff)
    }
    #[doc = "Bits 8:15 - DCR_TYPE"]
    #[inline(always)]
    pub fn dcrtype(&self) -> DcrtypeR {
        DcrtypeR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - BCR_TYPE"]
    #[inline(always)]
    pub fn bcrtype(&self) -> BcrtypeR {
        BcrtypeR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - STATIC_ADDR"]
    #[inline(always)]
    pub fn staticaddr(&self) -> StaticaddrR {
        StaticaddrR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {}
#[doc = "Device Characteristic Table Location-4 of Device3 (Master Mode)\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cd22c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cd22c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3cd22cSpec;
impl crate::RegisterSpec for I3cd22cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3cd22c::R`](R) reader structure"]
impl crate::Readable for I3cd22cSpec {}
#[doc = "`write(|w| ..)` method takes [`i3cd22c::W`](W) writer structure"]
impl crate::Writable for I3cd22cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CD22C to value 0"]
impl crate::Resettable for I3cd22cSpec {}
