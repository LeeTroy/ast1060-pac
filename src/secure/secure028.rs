#[doc = "Register `SECURE028` reader"]
pub type R = crate::R<Secure028Spec>;
#[doc = "Register `SECURE028` writer"]
pub type W = crate::W<Secure028Spec>;
#[doc = "Field `OTPDataCompareReg3` reader - OTP Data Compare Register 3"]
pub type OtpdataCompareReg3R = crate::FieldReader<u32>;
#[doc = "Field `OTPDataCompareReg3` writer - OTP Data Compare Register 3"]
pub type OtpdataCompareReg3W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - OTP Data Compare Register 3"]
    #[inline(always)]
    pub fn otpdata_compare_reg3(&self) -> OtpdataCompareReg3R {
        OtpdataCompareReg3R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - OTP Data Compare Register 3"]
    #[inline(always)]
    pub fn otpdata_compare_reg3(&mut self) -> OtpdataCompareReg3W<Secure028Spec> {
        OtpdataCompareReg3W::new(self, 0)
    }
}
#[doc = "OTP Data Compare Register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`secure028::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure028::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Secure028Spec;
impl crate::RegisterSpec for Secure028Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`secure028::R`](R) reader structure"]
impl crate::Readable for Secure028Spec {}
#[doc = "`write(|w| ..)` method takes [`secure028::W`](W) writer structure"]
impl crate::Writable for Secure028Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SECURE028 to value 0"]
impl crate::Resettable for Secure028Spec {}
