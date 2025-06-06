#[doc = "Register `SECURE004` reader"]
pub type R = crate::R<Secure004Spec>;
#[doc = "Register `SECURE004` writer"]
pub type W = crate::W<Secure004Spec>;
#[doc = "Field `OTPCmdTriggerReg` reader - OTP Command Trigger Register"]
pub type OtpcmdTriggerRegR = crate::FieldReader<u32>;
#[doc = "Field `OTPCmdTriggerReg` writer - OTP Command Trigger Register"]
pub type OtpcmdTriggerRegW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - OTP Command Trigger Register"]
    #[inline(always)]
    pub fn otpcmd_trigger_reg(&self) -> OtpcmdTriggerRegR {
        OtpcmdTriggerRegR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - OTP Command Trigger Register"]
    #[inline(always)]
    pub fn otpcmd_trigger_reg(&mut self) -> OtpcmdTriggerRegW<Secure004Spec> {
        OtpcmdTriggerRegW::new(self, 0)
    }
}
#[doc = "OTP Command Trigger Register\n\nYou can [`read`](crate::Reg::read) this register and get [`secure004::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure004::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Secure004Spec;
impl crate::RegisterSpec for Secure004Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`secure004::R`](R) reader structure"]
impl crate::Readable for Secure004Spec {}
#[doc = "`write(|w| ..)` method takes [`secure004::W`](W) writer structure"]
impl crate::Writable for Secure004Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SECURE004 to value 0"]
impl crate::Resettable for Secure004Spec {}
