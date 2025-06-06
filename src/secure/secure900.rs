#[doc = "Register `SECURE900` reader"]
pub type R = crate::R<Secure900Spec>;
#[doc = "Register `SECURE900` writer"]
pub type W = crate::W<Secure900Spec>;
#[doc = "Field `SecBootFirstVaultKey0Reg` reader - Secure Boot First Vault Key 0 Register"]
pub type SecBootFirstVaultKey0regR = crate::FieldReader<u32>;
#[doc = "Field `SecBootFirstVaultKey0Reg` writer - Secure Boot First Vault Key 0 Register"]
pub type SecBootFirstVaultKey0regW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Secure Boot First Vault Key 0 Register"]
    #[inline(always)]
    pub fn sec_boot_first_vault_key0reg(&self) -> SecBootFirstVaultKey0regR {
        SecBootFirstVaultKey0regR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Secure Boot First Vault Key 0 Register"]
    #[inline(always)]
    pub fn sec_boot_first_vault_key0reg(&mut self) -> SecBootFirstVaultKey0regW<Secure900Spec> {
        SecBootFirstVaultKey0regW::new(self, 0)
    }
}
#[doc = "Secure Boot First Vault Key 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`secure900::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure900::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Secure900Spec;
impl crate::RegisterSpec for Secure900Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`secure900::R`](R) reader structure"]
impl crate::Readable for Secure900Spec {}
#[doc = "`write(|w| ..)` method takes [`secure900::W`](W) writer structure"]
impl crate::Writable for Secure900Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SECURE900 to value 0"]
impl crate::Resettable for Secure900Spec {}
