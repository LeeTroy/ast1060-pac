#[doc = "Register `SECURE90C` reader"]
pub type R = crate::R<Secure90cSpec>;
#[doc = "Register `SECURE90C` writer"]
pub type W = crate::W<Secure90cSpec>;
#[doc = "Field `SecBootFirstVaultKey3Reg` reader - Secure Boot First Vault Key 3 Register"]
pub type SecBootFirstVaultKey3regR = crate::FieldReader<u32>;
#[doc = "Field `SecBootFirstVaultKey3Reg` writer - Secure Boot First Vault Key 3 Register"]
pub type SecBootFirstVaultKey3regW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Secure Boot First Vault Key 3 Register"]
    #[inline(always)]
    pub fn sec_boot_first_vault_key3reg(&self) -> SecBootFirstVaultKey3regR {
        SecBootFirstVaultKey3regR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Secure Boot First Vault Key 3 Register"]
    #[inline(always)]
    pub fn sec_boot_first_vault_key3reg(&mut self) -> SecBootFirstVaultKey3regW<Secure90cSpec> {
        SecBootFirstVaultKey3regW::new(self, 0)
    }
}
#[doc = "Secure Boot First Vault Key 3 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`secure90c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure90c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Secure90cSpec;
impl crate::RegisterSpec for Secure90cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`secure90c::R`](R) reader structure"]
impl crate::Readable for Secure90cSpec {}
#[doc = "`write(|w| ..)` method takes [`secure90c::W`](W) writer structure"]
impl crate::Writable for Secure90cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SECURE90C to value 0"]
impl crate::Resettable for Secure90cSpec {}
