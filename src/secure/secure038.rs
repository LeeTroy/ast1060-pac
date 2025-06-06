#[doc = "Register `SECURE038` reader"]
pub type R = crate::R<Secure038Spec>;
#[doc = "Register `SECURE038` writer"]
pub type W = crate::W<Secure038Spec>;
#[doc = "Field `OTPQRRDataReadBack` reader - OTP QRR data read back"]
pub type OtpqrrdataReadBackR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - OTP QRR data read back"]
    #[inline(always)]
    pub fn otpqrrdata_read_back(&self) -> OtpqrrdataReadBackR {
        OtpqrrdataReadBackR::new(self.bits)
    }
}
impl W {}
#[doc = "OTP QRR data read back\n\nYou can [`read`](crate::Reg::read) this register and get [`secure038::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure038::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Secure038Spec;
impl crate::RegisterSpec for Secure038Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`secure038::R`](R) reader structure"]
impl crate::Readable for Secure038Spec {}
#[doc = "`write(|w| ..)` method takes [`secure038::W`](W) writer structure"]
impl crate::Writable for Secure038Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SECURE038 to value 0"]
impl crate::Resettable for Secure038Spec {}
