#[doc = "Register `SECURE8A8` reader"]
pub type R = crate::R<Secure8a8Spec>;
#[doc = "Register `SECURE8A8` writer"]
pub type W = crate::W<Secure8a8Spec>;
#[doc = "Field `SecBootCryptoKeyBuffer2Reg` reader - Secure Boot Crypto Key Buffer 2 Register"]
pub type SecBootCryptoKeyBuffer2regR = crate::FieldReader<u32>;
#[doc = "Field `SecBootCryptoKeyBuffer2Reg` writer - Secure Boot Crypto Key Buffer 2 Register"]
pub type SecBootCryptoKeyBuffer2regW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Secure Boot Crypto Key Buffer 2 Register"]
    #[inline(always)]
    pub fn sec_boot_crypto_key_buffer2reg(&self) -> SecBootCryptoKeyBuffer2regR {
        SecBootCryptoKeyBuffer2regR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Secure Boot Crypto Key Buffer 2 Register"]
    #[inline(always)]
    pub fn sec_boot_crypto_key_buffer2reg(&mut self) -> SecBootCryptoKeyBuffer2regW<Secure8a8Spec> {
        SecBootCryptoKeyBuffer2regW::new(self, 0)
    }
}
#[doc = "Secure Boot Crypto Key Buffer 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`secure8a8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure8a8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Secure8a8Spec;
impl crate::RegisterSpec for Secure8a8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`secure8a8::R`](R) reader structure"]
impl crate::Readable for Secure8a8Spec {}
#[doc = "`write(|w| ..)` method takes [`secure8a8::W`](W) writer structure"]
impl crate::Writable for Secure8a8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SECURE8A8 to value 0"]
impl crate::Resettable for Secure8a8Spec {}
