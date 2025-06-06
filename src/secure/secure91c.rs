#[doc = "Register `SECURE91C` reader"]
pub type R = crate::R<Secure91cSpec>;
#[doc = "Register `SECURE91C` writer"]
pub type W = crate::W<Secure91cSpec>;
#[doc = "Field `SecBootFirstVaultKey7Reg` reader - Secure Boot First Vault Key 7 Register"]
pub type SecBootFirstVaultKey7regR = crate::FieldReader<u32>;
#[doc = "Field `SecBootFirstVaultKey7Reg` writer - Secure Boot First Vault Key 7 Register"]
pub type SecBootFirstVaultKey7regW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Secure Boot First Vault Key 7 Register"]
    #[inline(always)]
    pub fn sec_boot_first_vault_key7reg(&self) -> SecBootFirstVaultKey7regR {
        SecBootFirstVaultKey7regR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Secure Boot First Vault Key 7 Register"]
    #[inline(always)]
    pub fn sec_boot_first_vault_key7reg(&mut self) -> SecBootFirstVaultKey7regW<Secure91cSpec> {
        SecBootFirstVaultKey7regW::new(self, 0)
    }
}
#[doc = "Secure Boot First Vault Key 7 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`secure91c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure91c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Secure91cSpec;
impl crate::RegisterSpec for Secure91cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`secure91c::R`](R) reader structure"]
impl crate::Readable for Secure91cSpec {}
#[doc = "`write(|w| ..)` method takes [`secure91c::W`](W) writer structure"]
impl crate::Writable for Secure91cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SECURE91C to value 0"]
impl crate::Resettable for Secure91cSpec {}
