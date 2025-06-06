#[doc = "Register `SECURE02C` reader"]
pub type R = crate::R<Secure02cSpec>;
#[doc = "Register `SECURE02C` writer"]
pub type W = crate::W<Secure02cSpec>;
#[doc = "Field `OTPDataCompareReg4` reader - OTP Data Compare Register 4"]
pub type OtpdataCompareReg4R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - OTP Data Compare Register 4"]
    #[inline(always)]
    pub fn otpdata_compare_reg4(&self) -> OtpdataCompareReg4R {
        OtpdataCompareReg4R::new(self.bits)
    }
}
impl W {}
#[doc = "OTP Data Compare Register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`secure02c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure02c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Secure02cSpec;
impl crate::RegisterSpec for Secure02cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`secure02c::R`](R) reader structure"]
impl crate::Readable for Secure02cSpec {}
#[doc = "`write(|w| ..)` method takes [`secure02c::W`](W) writer structure"]
impl crate::Writable for Secure02cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SECURE02C to value 0"]
impl crate::Resettable for Secure02cSpec {}
