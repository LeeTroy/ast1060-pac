#[doc = "Register `I3CD0D4` reader"]
pub type R = crate::R<I3cd0d4Spec>;
#[doc = "Register `I3CD0D4` writer"]
pub type W = crate::W<I3cd0d4Spec>;
#[doc = "Field `I3CMSTFREE` reader - I3C_MST_FREE"]
pub type I3cmstfreeR = crate::FieldReader<u16>;
#[doc = "Field `I3CMSTFREE` writer - I3C_MST_FREE"]
pub type I3cmstfreeW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `I3CIBIFREE` reader - I3C_IBI_FREE"]
pub type I3cibifreeR = crate::FieldReader<u16>;
#[doc = "Field `I3CIBIFREE` writer - I3C_IBI_FREE"]
pub type I3cibifreeW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - I3C_MST_FREE"]
    #[inline(always)]
    pub fn i3cmstfree(&self) -> I3cmstfreeR {
        I3cmstfreeR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - I3C_IBI_FREE"]
    #[inline(always)]
    pub fn i3cibifree(&self) -> I3cibifreeR {
        I3cibifreeR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - I3C_MST_FREE"]
    #[inline(always)]
    pub fn i3cmstfree(&mut self) -> I3cmstfreeW<I3cd0d4Spec> {
        I3cmstfreeW::new(self, 0)
    }
    #[doc = "Bits 16:31 - I3C_IBI_FREE"]
    #[inline(always)]
    pub fn i3cibifree(&mut self) -> I3cibifreeW<I3cd0d4Spec> {
        I3cibifreeW::new(self, 16)
    }
}
#[doc = "Bus Free Timing Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cd0d4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cd0d4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3cd0d4Spec;
impl crate::RegisterSpec for I3cd0d4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3cd0d4::R`](R) reader structure"]
impl crate::Readable for I3cd0d4Spec {}
#[doc = "`write(|w| ..)` method takes [`i3cd0d4::W`](W) writer structure"]
impl crate::Writable for I3cd0d4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CD0D4 to value 0"]
impl crate::Resettable for I3cd0d4Spec {}
