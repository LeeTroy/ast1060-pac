#[doc = "Register `I3CD0D8` reader"]
pub type R = crate::R<I3cd0d8Spec>;
#[doc = "Register `I3CD0D8` writer"]
pub type W = crate::W<I3cd0d8Spec>;
#[doc = "Field `BUSIDLETIME` reader - BUS_IDLE_TIME"]
pub type BusidletimeR = crate::FieldReader<u32>;
#[doc = "Field `BUSIDLETIME` writer - BUS_IDLE_TIME"]
pub type BusidletimeW<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
#[doc = "Field `RSVD` reader - reserved"]
pub type RsvdR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:19 - BUS_IDLE_TIME"]
    #[inline(always)]
    pub fn busidletime(&self) -> BusidletimeR {
        BusidletimeR::new(self.bits & 0x000f_ffff)
    }
    #[doc = "Bits 20:31 - reserved"]
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:19 - BUS_IDLE_TIME"]
    #[inline(always)]
    pub fn busidletime(&mut self) -> BusidletimeW<I3cd0d8Spec> {
        BusidletimeW::new(self, 0)
    }
}
#[doc = "Bus Idle Timing Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cd0d8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cd0d8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3cd0d8Spec;
impl crate::RegisterSpec for I3cd0d8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3cd0d8::R`](R) reader structure"]
impl crate::Readable for I3cd0d8Spec {}
#[doc = "`write(|w| ..)` method takes [`i3cd0d8::W`](W) writer structure"]
impl crate::Writable for I3cd0d8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CD0D8 to value 0"]
impl crate::Resettable for I3cd0d8Spec {}
