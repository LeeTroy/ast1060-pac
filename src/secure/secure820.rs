#[doc = "Register `SECURE820` reader"]
pub type R = crate::R<Secure820Spec>;
#[doc = "Register `SECURE820` writer"]
pub type W = crate::W<Secure820Spec>;
#[doc = "Secure Crypto Engine Enable Register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SecCryptoEngEnblReg {
    #[doc = "0: Secure crypto engine disable"]
    SecureCryptoEngineDisable = 0,
    #[doc = "1: Secure crypto engine enable. HACE won't work when this mode."]
    SecureCryptoEngineEnableHaceWontWorkWhenThisMode = 1,
}
impl From<SecCryptoEngEnblReg> for bool {
    #[inline(always)]
    fn from(variant: SecCryptoEngEnblReg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SecCryptoEngEnblReg` reader - Secure Crypto Engine Enable Register"]
pub type SecCryptoEngEnblRegR = crate::BitReader<SecCryptoEngEnblReg>;
impl SecCryptoEngEnblRegR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SecCryptoEngEnblReg {
        match self.bits {
            false => SecCryptoEngEnblReg::SecureCryptoEngineDisable,
            true => SecCryptoEngEnblReg::SecureCryptoEngineEnableHaceWontWorkWhenThisMode,
        }
    }
    #[doc = "Secure crypto engine disable"]
    #[inline(always)]
    pub fn is_secure_crypto_engine_disable(&self) -> bool {
        *self == SecCryptoEngEnblReg::SecureCryptoEngineDisable
    }
    #[doc = "Secure crypto engine enable. HACE won't work when this mode."]
    #[inline(always)]
    pub fn is_secure_crypto_engine_enable_hace_wont_work_when_this_mode(&self) -> bool {
        *self == SecCryptoEngEnblReg::SecureCryptoEngineEnableHaceWontWorkWhenThisMode
    }
}
#[doc = "Field `SecCryptoEngEnblReg` writer - Secure Crypto Engine Enable Register"]
pub type SecCryptoEngEnblRegW<'a, REG> = crate::BitWriter<'a, REG, SecCryptoEngEnblReg>;
impl<'a, REG> SecCryptoEngEnblRegW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Secure crypto engine disable"]
    #[inline(always)]
    pub fn secure_crypto_engine_disable(self) -> &'a mut crate::W<REG> {
        self.variant(SecCryptoEngEnblReg::SecureCryptoEngineDisable)
    }
    #[doc = "Secure crypto engine enable. HACE won't work when this mode."]
    #[inline(always)]
    pub fn secure_crypto_engine_enable_hace_wont_work_when_this_mode(
        self,
    ) -> &'a mut crate::W<REG> {
        self.variant(SecCryptoEngEnblReg::SecureCryptoEngineEnableHaceWontWorkWhenThisMode)
    }
}
#[doc = "Field `Reserved0` reader - Reserved(0)"]
pub type Reserved0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - Secure Crypto Engine Enable Register"]
    #[inline(always)]
    pub fn sec_crypto_eng_enbl_reg(&self) -> SecCryptoEngEnblRegR {
        SecCryptoEngEnblRegR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31 - Reserved(0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - Secure Crypto Engine Enable Register"]
    #[inline(always)]
    pub fn sec_crypto_eng_enbl_reg(&mut self) -> SecCryptoEngEnblRegW<Secure820Spec> {
        SecCryptoEngEnblRegW::new(self, 0)
    }
}
#[doc = "Secure Crypto Engine Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`secure820::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure820::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Secure820Spec;
impl crate::RegisterSpec for Secure820Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`secure820::R`](R) reader structure"]
impl crate::Readable for Secure820Spec {}
#[doc = "`write(|w| ..)` method takes [`secure820::W`](W) writer structure"]
impl crate::Writable for Secure820Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SECURE820 to value 0"]
impl crate::Resettable for Secure820Spec {}
