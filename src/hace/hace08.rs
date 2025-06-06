#[doc = "Register `HACE08` reader"]
pub type R = crate::R<Hace08Spec>;
#[doc = "Register `HACE08` writer"]
pub type W = crate::W<Hace08Spec>;
#[doc = "Field `Reserved01` reader - Reserved (0)"]
pub type Reserved01R = crate::FieldReader;
#[doc = "Field `BaseAddrOfCryptoCtxBuf303` reader - Base address of crypto context buffer\\[30:3\\] (8-byte aligned)"]
pub type BaseAddrOfCryptoCtxBuf303R = crate::FieldReader<u32>;
#[doc = "Field `BaseAddrOfCryptoCtxBuf303` writer - Base address of crypto context buffer\\[30:3\\] (8-byte aligned)"]
pub type BaseAddrOfCryptoCtxBuf303W<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
#[doc = "Field `Reserved0` reader - Reserved (0)"]
pub type Reserved0R = crate::BitReader;
impl R {
    #[doc = "Bits 0:2 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved01(&self) -> Reserved01R {
        Reserved01R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:30 - Base address of crypto context buffer\\[30:3\\] (8-byte aligned)"]
    #[inline(always)]
    pub fn base_addr_of_crypto_ctx_buf303(&self) -> BaseAddrOfCryptoCtxBuf303R {
        BaseAddrOfCryptoCtxBuf303R::new((self.bits >> 3) & 0x0fff_ffff)
    }
    #[doc = "Bit 31 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 3:30 - Base address of crypto context buffer\\[30:3\\] (8-byte aligned)"]
    #[inline(always)]
    pub fn base_addr_of_crypto_ctx_buf303(&mut self) -> BaseAddrOfCryptoCtxBuf303W<Hace08Spec> {
        BaseAddrOfCryptoCtxBuf303W::new(self, 3)
    }
}
#[doc = "Crypto Context Buffer Base Address Registerr\n\nYou can [`read`](crate::Reg::read) this register and get [`hace08::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hace08::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hace08Spec;
impl crate::RegisterSpec for Hace08Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hace08::R`](R) reader structure"]
impl crate::Readable for Hace08Spec {}
#[doc = "`write(|w| ..)` method takes [`hace08::W`](W) writer structure"]
impl crate::Writable for Hace08Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HACE08 to value 0"]
impl crate::Resettable for Hace08Spec {}
