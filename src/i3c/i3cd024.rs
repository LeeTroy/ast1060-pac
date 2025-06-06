#[doc = "Register `I3CD024` reader"]
pub type R = crate::R<I3cd024Spec>;
#[doc = "Register `I3CD024` writer"]
pub type W = crate::W<I3cd024Spec>;
#[doc = "Field `NotifyRejectedHotJoinCtrl` reader - Notify Rejected Hot-Join Control."]
pub type NotifyRejectedHotJoinCtrlR = crate::BitReader;
#[doc = "Field `NotifyRejectedHotJoinCtrl` writer - Notify Rejected Hot-Join Control."]
pub type NotifyRejectedHotJoinCtrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NotifyRejectedMasterReqCtrl` reader - Notify Rejected Master Request Control."]
pub type NotifyRejectedMasterReqCtrlR = crate::BitReader;
#[doc = "Field `NotifyRejectedMasterReqCtrl` writer - Notify Rejected Master Request Control."]
pub type NotifyRejectedMasterReqCtrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSVD2` reader - These bits in IBI Queue Control register are reserved."]
pub type Rsvd2R = crate::BitReader;
#[doc = "Field `NotifyRejectedSlaveINTReqCtrl` reader - Notify Rejected Slave Interrupt Request Control"]
pub type NotifyRejectedSlaveIntreqCtrlR = crate::BitReader;
#[doc = "Field `NotifyRejectedSlaveINTReqCtrl` writer - Notify Rejected Slave Interrupt Request Control"]
pub type NotifyRejectedSlaveIntreqCtrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSVD` reader - reserved"]
pub type RsvdR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - Notify Rejected Hot-Join Control."]
    #[inline(always)]
    pub fn notify_rejected_hot_join_ctrl(&self) -> NotifyRejectedHotJoinCtrlR {
        NotifyRejectedHotJoinCtrlR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Notify Rejected Master Request Control."]
    #[inline(always)]
    pub fn notify_rejected_master_req_ctrl(&self) -> NotifyRejectedMasterReqCtrlR {
        NotifyRejectedMasterReqCtrlR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - These bits in IBI Queue Control register are reserved."]
    #[inline(always)]
    pub fn rsvd2(&self) -> Rsvd2R {
        Rsvd2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Notify Rejected Slave Interrupt Request Control"]
    #[inline(always)]
    pub fn notify_rejected_slave_intreq_ctrl(&self) -> NotifyRejectedSlaveIntreqCtrlR {
        NotifyRejectedSlaveIntreqCtrlR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:31 - reserved"]
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - Notify Rejected Hot-Join Control."]
    #[inline(always)]
    pub fn notify_rejected_hot_join_ctrl(&mut self) -> NotifyRejectedHotJoinCtrlW<I3cd024Spec> {
        NotifyRejectedHotJoinCtrlW::new(self, 0)
    }
    #[doc = "Bit 1 - Notify Rejected Master Request Control."]
    #[inline(always)]
    pub fn notify_rejected_master_req_ctrl(&mut self) -> NotifyRejectedMasterReqCtrlW<I3cd024Spec> {
        NotifyRejectedMasterReqCtrlW::new(self, 1)
    }
    #[doc = "Bit 3 - Notify Rejected Slave Interrupt Request Control"]
    #[inline(always)]
    pub fn notify_rejected_slave_intreq_ctrl(
        &mut self,
    ) -> NotifyRejectedSlaveIntreqCtrlW<I3cd024Spec> {
        NotifyRejectedSlaveIntreqCtrlW::new(self, 3)
    }
}
#[doc = "IBI Queue Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cd024::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cd024::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3cd024Spec;
impl crate::RegisterSpec for I3cd024Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3cd024::R`](R) reader structure"]
impl crate::Readable for I3cd024Spec {}
#[doc = "`write(|w| ..)` method takes [`i3cd024::W`](W) writer structure"]
impl crate::Writable for I3cd024Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CD024 to value 0"]
impl crate::Resettable for I3cd024Spec {}
