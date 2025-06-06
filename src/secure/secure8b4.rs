#[doc = "Register `SECURE8B4` reader"]
pub type R = crate::R<Secure8b4Spec>;
#[doc = "Register `SECURE8B4` writer"]
pub type W = crate::W<Secure8b4Spec>;
#[doc = "Field `SecBootCryptoKeyBuffer5Reg` reader - Secure Boot Crypto Key Buffer 5 Register"]
pub type SecBootCryptoKeyBuffer5regR = crate::FieldReader<u32>;
#[doc = "Field `SecBootCryptoKeyBuffer5Reg` writer - Secure Boot Crypto Key Buffer 5 Register"]
pub type SecBootCryptoKeyBuffer5regW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Secure Boot Crypto Key Buffer 5 Register"]
    #[inline(always)]
    pub fn sec_boot_crypto_key_buffer5reg(&self) -> SecBootCryptoKeyBuffer5regR {
        SecBootCryptoKeyBuffer5regR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Secure Boot Crypto Key Buffer 5 Register"]
    #[inline(always)]
    pub fn sec_boot_crypto_key_buffer5reg(&mut self) -> SecBootCryptoKeyBuffer5regW<Secure8b4Spec> {
        SecBootCryptoKeyBuffer5regW::new(self, 0)
    }
}
#[doc = "Secure Boot Crypto Key Buffer 5 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`secure8b4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure8b4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Secure8b4Spec;
impl crate::RegisterSpec for Secure8b4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`secure8b4::R`](R) reader structure"]
impl crate::Readable for Secure8b4Spec {}
#[doc = "`write(|w| ..)` method takes [`secure8b4::W`](W) writer structure"]
impl crate::Writable for Secure8b4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SECURE8B4 to value 0"]
impl crate::Resettable for Secure8b4Spec {}
