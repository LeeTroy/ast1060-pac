#[doc = "Register `I3CD02C` reader"]
pub type R = crate::R<I3cd02cSpec>;
#[doc = "Register `I3CD02C` writer"]
pub type W = crate::W<I3cd02cSpec>;
#[doc = "Field `InbandMasterReqReject` reader - In-band Master Request Reject."]
pub type InbandMasterReqRejectR = crate::FieldReader<u32>;
#[doc = "Field `InbandMasterReqReject` writer - In-band Master Request Reject."]
pub type InbandMasterReqRejectW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - In-band Master Request Reject."]
    #[inline(always)]
    pub fn inband_master_req_reject(&self) -> InbandMasterReqRejectR {
        InbandMasterReqRejectR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - In-band Master Request Reject."]
    #[inline(always)]
    pub fn inband_master_req_reject(&mut self) -> InbandMasterReqRejectW<I3cd02cSpec> {
        InbandMasterReqRejectW::new(self, 0)
    }
}
#[doc = "IBI MR Request Rejection Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cd02c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cd02c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3cd02cSpec;
impl crate::RegisterSpec for I3cd02cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3cd02c::R`](R) reader structure"]
impl crate::Readable for I3cd02cSpec {}
#[doc = "`write(|w| ..)` method takes [`i3cd02c::W`](W) writer structure"]
impl crate::Writable for I3cd02cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CD02C to value 0"]
impl crate::Resettable for I3cd02cSpec {}
