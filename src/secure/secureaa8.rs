#[doc = "Register `SECUREAA8` reader"]
pub type R = crate::R<Secureaa8Spec>;
#[doc = "Register `SECUREAA8` writer"]
pub type W = crate::W<Secureaa8Spec>;
#[doc = "Field `SecBootECCDefaultPublicKeyReadBack` reader - Secure Boot ECC Default Public Key Read Back"]
pub type SecBootEccdefaultPublicKeyReadBackR = crate::FieldReader<u32>;
#[doc = "Field `SecBootECCDefaultPublicKeyReadBack` writer - Secure Boot ECC Default Public Key Read Back"]
pub type SecBootEccdefaultPublicKeyReadBackW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Secure Boot ECC Default Public Key Read Back"]
    #[inline(always)]
    pub fn sec_boot_eccdefault_public_key_read_back(&self) -> SecBootEccdefaultPublicKeyReadBackR {
        SecBootEccdefaultPublicKeyReadBackR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Secure Boot ECC Default Public Key Read Back"]
    #[inline(always)]
    pub fn sec_boot_eccdefault_public_key_read_back(
        &mut self,
    ) -> SecBootEccdefaultPublicKeyReadBackW<Secureaa8Spec> {
        SecBootEccdefaultPublicKeyReadBackW::new(self, 0)
    }
}
#[doc = "Secure Boot ECC Default Public Key Read Back \\#0 - \\#16\n\nYou can [`read`](crate::Reg::read) this register and get [`secureaa8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secureaa8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Secureaa8Spec;
impl crate::RegisterSpec for Secureaa8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`secureaa8::R`](R) reader structure"]
impl crate::Readable for Secureaa8Spec {}
#[doc = "`write(|w| ..)` method takes [`secureaa8::W`](W) writer structure"]
impl crate::Writable for Secureaa8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SECUREAA8 to value 0"]
impl crate::Resettable for Secureaa8Spec {}
