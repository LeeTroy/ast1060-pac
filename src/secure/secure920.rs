#[doc = "Register `SECURE920` reader"]
pub type R = crate::R<Secure920Spec>;
#[doc = "Register `SECURE920` writer"]
pub type W = crate::W<Secure920Spec>;
#[doc = "Field `SecBootSecondVaultKey0Reg` reader - Secure Boot Second Vault Key 0 Register"]
pub type SecBootSecondVaultKey0regR = crate::FieldReader<u32>;
#[doc = "Field `SecBootSecondVaultKey0Reg` writer - Secure Boot Second Vault Key 0 Register"]
pub type SecBootSecondVaultKey0regW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Secure Boot Second Vault Key 0 Register"]
    #[inline(always)]
    pub fn sec_boot_second_vault_key0reg(&self) -> SecBootSecondVaultKey0regR {
        SecBootSecondVaultKey0regR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Secure Boot Second Vault Key 0 Register"]
    #[inline(always)]
    pub fn sec_boot_second_vault_key0reg(&mut self) -> SecBootSecondVaultKey0regW<Secure920Spec> {
        SecBootSecondVaultKey0regW::new(self, 0)
    }
}
#[doc = "Secure Boot Second Vault Key 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`secure920::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure920::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Secure920Spec;
impl crate::RegisterSpec for Secure920Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`secure920::R`](R) reader structure"]
impl crate::Readable for Secure920Spec {}
#[doc = "`write(|w| ..)` method takes [`secure920::W`](W) writer structure"]
impl crate::Writable for Secure920Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SECURE920 to value 0"]
impl crate::Resettable for Secure920Spec {}
