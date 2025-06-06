#[doc = "Register `SECURE848` reader"]
pub type R = crate::R<Secure848Spec>;
#[doc = "Register `SECURE848` writer"]
pub type W = crate::W<Secure848Spec>;
#[doc = "Field `SecBootDMASizeReg` reader - Secure Boot DMA Size Register"]
pub type SecBootDmasizeRegR = crate::FieldReader<u32>;
#[doc = "Field `SecBootDMASizeReg` writer - Secure Boot DMA Size Register"]
pub type SecBootDmasizeRegW<'a, REG> = crate::FieldWriter<'a, REG, 17, u32>;
#[doc = "Field `Reserved0` reader - Reserved(0)"]
pub type Reserved0R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:16 - Secure Boot DMA Size Register"]
    #[inline(always)]
    pub fn sec_boot_dmasize_reg(&self) -> SecBootDmasizeRegR {
        SecBootDmasizeRegR::new(self.bits & 0x0001_ffff)
    }
    #[doc = "Bits 17:31 - Reserved(0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new(((self.bits >> 17) & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:16 - Secure Boot DMA Size Register"]
    #[inline(always)]
    pub fn sec_boot_dmasize_reg(&mut self) -> SecBootDmasizeRegW<Secure848Spec> {
        SecBootDmasizeRegW::new(self, 0)
    }
}
#[doc = "Secure Boot DMA Size Register\n\nYou can [`read`](crate::Reg::read) this register and get [`secure848::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure848::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Secure848Spec;
impl crate::RegisterSpec for Secure848Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`secure848::R`](R) reader structure"]
impl crate::Readable for Secure848Spec {}
#[doc = "`write(|w| ..)` method takes [`secure848::W`](W) writer structure"]
impl crate::Writable for Secure848Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SECURE848 to value 0"]
impl crate::Resettable for Secure848Spec {}
