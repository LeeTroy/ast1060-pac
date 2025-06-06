#[doc = "Register `I3CD0DC` reader"]
pub type R = crate::R<I3cd0dcSpec>;
#[doc = "Register `I3CD0DC` writer"]
pub type W = crate::W<I3cd0dcSpec>;
#[doc = "Field `SCLLOWMSTTIMEOUTCOUNT` reader - SCL_LOW_MST_TIMEOUT_COUNT"]
pub type ScllowmsttimeoutcountR = crate::FieldReader<u32>;
#[doc = "Field `SCLLOWMSTTIMEOUTCOUNT` writer - SCL_LOW_MST_TIMEOUT_COUNT"]
pub type ScllowmsttimeoutcountW<'a, REG> = crate::FieldWriter<'a, REG, 26, u32>;
#[doc = "Field `RSVD` reader - reserved"]
pub type RsvdR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:25 - SCL_LOW_MST_TIMEOUT_COUNT"]
    #[inline(always)]
    pub fn scllowmsttimeoutcount(&self) -> ScllowmsttimeoutcountR {
        ScllowmsttimeoutcountR::new(self.bits & 0x03ff_ffff)
    }
    #[doc = "Bits 26:31 - reserved"]
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:25 - SCL_LOW_MST_TIMEOUT_COUNT"]
    #[inline(always)]
    pub fn scllowmsttimeoutcount(&mut self) -> ScllowmsttimeoutcountW<I3cd0dcSpec> {
        ScllowmsttimeoutcountW::new(self, 0)
    }
}
#[doc = "SCL Low Master Extended Timeout Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cd0dc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cd0dc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3cd0dcSpec;
impl crate::RegisterSpec for I3cd0dcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3cd0dc::R`](R) reader structure"]
impl crate::Readable for I3cd0dcSpec {}
#[doc = "`write(|w| ..)` method takes [`i3cd0dc::W`](W) writer structure"]
impl crate::Writable for I3cd0dcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CD0DC to value 0"]
impl crate::Resettable for I3cd0dcSpec {}
