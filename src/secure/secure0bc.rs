#[doc = "Register `SECURE0BC` reader"]
pub type R = crate::R<Secure0bcSpec>;
#[doc = "Register `SECURE0BC` writer"]
pub type W = crate::W<Secure0bcSpec>;
#[doc = "Field `SecBootRSAEngTriggerReg` reader - Secure Boot RSA Engine Trigger Register"]
pub type SecBootRsaengTriggerRegR = crate::BitReader;
#[doc = "Field `SecBootRSAEngTriggerReg` writer - Secure Boot RSA Engine Trigger Register"]
pub type SecBootRsaengTriggerRegW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved0` reader - Reserved(0)"]
pub type Reserved0R = crate::FieldReader<u32>;
#[doc = "Field `SecBootECCEngTriggerReg` reader - Secure Boot ECC Engine Trigger Register"]
pub type SecBootEccengTriggerRegR = crate::BitReader;
#[doc = "Field `SecBootECCEngTriggerReg` writer - Secure Boot ECC Engine Trigger Register"]
pub type SecBootEccengTriggerRegW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved01` reader - Reserved(0)"]
pub type Reserved01R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - Secure Boot RSA Engine Trigger Register"]
    #[inline(always)]
    pub fn sec_boot_rsaeng_trigger_reg(&self) -> SecBootRsaengTriggerRegR {
        SecBootRsaengTriggerRegR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31 - Reserved(0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits >> 1) & 0x7fff_ffff)
    }
    #[doc = "Bit 1 - Secure Boot ECC Engine Trigger Register"]
    #[inline(always)]
    pub fn sec_boot_ecceng_trigger_reg(&self) -> SecBootEccengTriggerRegR {
        SecBootEccengTriggerRegR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:31 - Reserved(0)"]
    #[inline(always)]
    pub fn reserved01(&self) -> Reserved01R {
        Reserved01R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - Secure Boot RSA Engine Trigger Register"]
    #[inline(always)]
    pub fn sec_boot_rsaeng_trigger_reg(&mut self) -> SecBootRsaengTriggerRegW<Secure0bcSpec> {
        SecBootRsaengTriggerRegW::new(self, 0)
    }
    #[doc = "Bit 1 - Secure Boot ECC Engine Trigger Register"]
    #[inline(always)]
    pub fn sec_boot_ecceng_trigger_reg(&mut self) -> SecBootEccengTriggerRegW<Secure0bcSpec> {
        SecBootEccengTriggerRegW::new(self, 1)
    }
}
#[doc = "Secure Boot Engine Trigger Register\n\nYou can [`read`](crate::Reg::read) this register and get [`secure0bc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure0bc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Secure0bcSpec;
impl crate::RegisterSpec for Secure0bcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`secure0bc::R`](R) reader structure"]
impl crate::Readable for Secure0bcSpec {}
#[doc = "`write(|w| ..)` method takes [`secure0bc::W`](W) writer structure"]
impl crate::Writable for Secure0bcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SECURE0BC to value 0"]
impl crate::Resettable for Secure0bcSpec {}
