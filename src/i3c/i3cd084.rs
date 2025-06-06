#[doc = "Register `I3CD084` reader"]
pub type R = crate::R<I3cd084Spec>;
#[doc = "Register `I3CD084` writer"]
pub type W = crate::W<I3cd084Spec>;
#[doc = "Field `MXDSMAXWRSPEED` reader - MXDS_MAX_WR_SPEED"]
pub type MxdsmaxwrspeedR = crate::FieldReader;
#[doc = "Field `MXDSMAXWRSPEED` writer - MXDS_MAX_WR_SPEED"]
pub type MxdsmaxwrspeedW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RSVD73` reader - These bits in MXDS Maximum Data Speed Register are reserved."]
pub type Rsvd73R = crate::FieldReader;
#[doc = "Field `MXDSMAXRDSPEED` reader - MXDS_MAX_RD_SPEED"]
pub type MxdsmaxrdspeedR = crate::FieldReader;
#[doc = "Field `MXDSMAXRDSPEED` writer - MXDS_MAX_RD_SPEED"]
pub type MxdsmaxrdspeedW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RSVD1` reader - reserved"]
pub type Rsvd1R = crate::FieldReader;
#[doc = "Field `TounaroundTimeInNS` reader - Tounaround Time in NS"]
pub type TounaroundTimeInNsR = crate::FieldReader;
#[doc = "Field `TounaroundTimeInNS` writer - Tounaround Time in NS"]
pub type TounaroundTimeInNsW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RSVD` reader - reserved"]
pub type RsvdR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:2 - MXDS_MAX_WR_SPEED"]
    #[inline(always)]
    pub fn mxdsmaxwrspeed(&self) -> MxdsmaxwrspeedR {
        MxdsmaxwrspeedR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:7 - These bits in MXDS Maximum Data Speed Register are reserved."]
    #[inline(always)]
    pub fn rsvd73(&self) -> Rsvd73R {
        Rsvd73R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bits 8:10 - MXDS_MAX_RD_SPEED"]
    #[inline(always)]
    pub fn mxdsmaxrdspeed(&self) -> MxdsmaxrdspeedR {
        MxdsmaxrdspeedR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:15 - reserved"]
    #[inline(always)]
    pub fn rsvd1(&self) -> Rsvd1R {
        Rsvd1R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 16:18 - Tounaround Time in NS"]
    #[inline(always)]
    pub fn tounaround_time_in_ns(&self) -> TounaroundTimeInNsR {
        TounaroundTimeInNsR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 19:31 - reserved"]
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 19) & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:2 - MXDS_MAX_WR_SPEED"]
    #[inline(always)]
    pub fn mxdsmaxwrspeed(&mut self) -> MxdsmaxwrspeedW<I3cd084Spec> {
        MxdsmaxwrspeedW::new(self, 0)
    }
    #[doc = "Bits 8:10 - MXDS_MAX_RD_SPEED"]
    #[inline(always)]
    pub fn mxdsmaxrdspeed(&mut self) -> MxdsmaxrdspeedW<I3cd084Spec> {
        MxdsmaxrdspeedW::new(self, 8)
    }
    #[doc = "Bits 16:18 - Tounaround Time in NS"]
    #[inline(always)]
    pub fn tounaround_time_in_ns(&mut self) -> TounaroundTimeInNsW<I3cd084Spec> {
        TounaroundTimeInNsW::new(self, 16)
    }
}
#[doc = "MXDS Maximum Data Speed Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cd084::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cd084::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3cd084Spec;
impl crate::RegisterSpec for I3cd084Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3cd084::R`](R) reader structure"]
impl crate::Readable for I3cd084Spec {}
#[doc = "`write(|w| ..)` method takes [`i3cd084::W`](W) writer structure"]
impl crate::Writable for I3cd084Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CD084 to value 0"]
impl crate::Resettable for I3cd084Spec {}
