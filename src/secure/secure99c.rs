#[doc = "Register `SECURE99C` reader"]
pub type R = crate::R<Secure99cSpec>;
#[doc = "Register `SECURE99C` writer"]
pub type W = crate::W<Secure99cSpec>;
#[doc = "Field `SecBootImageDigestReadBack` reader - Secure Boot Image Digest Read Back"]
pub type SecBootImageDigestReadBackR = crate::FieldReader<u32>;
#[doc = "Field `SecBootImageDigestReadBack` writer - Secure Boot Image Digest Read Back"]
pub type SecBootImageDigestReadBackW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Secure Boot Image Digest Read Back"]
    #[inline(always)]
    pub fn sec_boot_image_digest_read_back(&self) -> SecBootImageDigestReadBackR {
        SecBootImageDigestReadBackR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Secure Boot Image Digest Read Back"]
    #[inline(always)]
    pub fn sec_boot_image_digest_read_back(
        &mut self,
    ) -> SecBootImageDigestReadBackW<Secure99cSpec> {
        SecBootImageDigestReadBackW::new(self, 0)
    }
}
#[doc = "Secure Boot Image Digest Read Back \\#0 - \\#16\n\nYou can [`read`](crate::Reg::read) this register and get [`secure99c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure99c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Secure99cSpec;
impl crate::RegisterSpec for Secure99cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`secure99c::R`](R) reader structure"]
impl crate::Readable for Secure99cSpec {}
#[doc = "`write(|w| ..)` method takes [`secure99c::W`](W) writer structure"]
impl crate::Writable for Secure99cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SECURE99C to value 0"]
impl crate::Resettable for Secure99cSpec {}
