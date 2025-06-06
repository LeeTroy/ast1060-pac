#[doc = "Register `SECURE910` reader"]
pub type R = crate::R<Secure910Spec>;
#[doc = "Register `SECURE910` writer"]
pub type W = crate::W<Secure910Spec>;
#[doc = "Field `SecBootFirstVaultKey4Reg` reader - Secure Boot First Vault Key 4 Register"]
pub type SecBootFirstVaultKey4regR = crate::FieldReader<u32>;
#[doc = "Field `SecBootFirstVaultKey4Reg` writer - Secure Boot First Vault Key 4 Register"]
pub type SecBootFirstVaultKey4regW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Secure Boot First Vault Key 4 Register"]
    #[inline(always)]
    pub fn sec_boot_first_vault_key4reg(&self) -> SecBootFirstVaultKey4regR {
        SecBootFirstVaultKey4regR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Secure Boot First Vault Key 4 Register"]
    #[inline(always)]
    pub fn sec_boot_first_vault_key4reg(&mut self) -> SecBootFirstVaultKey4regW<Secure910Spec> {
        SecBootFirstVaultKey4regW::new(self, 0)
    }
}
#[doc = "Secure Boot First Vault Key 4 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`secure910::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure910::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Secure910Spec;
impl crate::RegisterSpec for Secure910Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`secure910::R`](R) reader structure"]
impl crate::Readable for Secure910Spec {}
#[doc = "`write(|w| ..)` method takes [`secure910::W`](W) writer structure"]
impl crate::Writable for Secure910Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SECURE910 to value 0"]
impl crate::Resettable for Secure910Spec {}
