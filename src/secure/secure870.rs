#[doc = "Register `SECURE870` reader"]
pub type R = crate::R<Secure870Spec>;
#[doc = "Register `SECURE870` writer"]
pub type W = crate::W<Secure870Spec>;
#[doc = "Field `SecBootCryptoLowKeyWrTriggerReg` reader - Secure Boot Crypto Low Key Write Trigger Register"]
pub type SecBootCryptoLowKeyWrTriggerRegR = crate::FieldReader<u32>;
#[doc = "Field `SecBootCryptoLowKeyWrTriggerReg` writer - Secure Boot Crypto Low Key Write Trigger Register"]
pub type SecBootCryptoLowKeyWrTriggerRegW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Secure Boot Crypto Low Key Write Trigger Register"]
    #[inline(always)]
    pub fn sec_boot_crypto_low_key_wr_trigger_reg(&self) -> SecBootCryptoLowKeyWrTriggerRegR {
        SecBootCryptoLowKeyWrTriggerRegR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Secure Boot Crypto Low Key Write Trigger Register"]
    #[inline(always)]
    pub fn sec_boot_crypto_low_key_wr_trigger_reg(
        &mut self,
    ) -> SecBootCryptoLowKeyWrTriggerRegW<Secure870Spec> {
        SecBootCryptoLowKeyWrTriggerRegW::new(self, 0)
    }
}
#[doc = "Secure Boot Crypto Low Key Write Trigger Register\n\nYou can [`read`](crate::Reg::read) this register and get [`secure870::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure870::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Secure870Spec;
impl crate::RegisterSpec for Secure870Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`secure870::R`](R) reader structure"]
impl crate::Readable for Secure870Spec {}
#[doc = "`write(|w| ..)` method takes [`secure870::W`](W) writer structure"]
impl crate::Writable for Secure870Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SECURE870 to value 0"]
impl crate::Resettable for Secure870Spec {}
