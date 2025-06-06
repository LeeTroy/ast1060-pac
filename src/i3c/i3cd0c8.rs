#[doc = "Register `I3CD0C8` reader"]
pub type R = crate::R<I3cd0c8Spec>;
#[doc = "Register `I3CD0C8` writer"]
pub type W = crate::W<I3cd0c8Spec>;
#[doc = "Field `I3CEXTLCNT1` reader - I3C_EXT_LCNT_1"]
pub type I3cextlcnt1R = crate::FieldReader;
#[doc = "Field `I3CEXTLCNT1` writer - I3C_EXT_LCNT_1"]
pub type I3cextlcnt1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `I3CEXTLCNT2` reader - I3C_EXT_LCNT_2"]
pub type I3cextlcnt2R = crate::FieldReader;
#[doc = "Field `I3CEXTLCNT2` writer - I3C_EXT_LCNT_2"]
pub type I3cextlcnt2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `I3CEXTLCNT3` reader - I3C_EXT_LCNT_3"]
pub type I3cextlcnt3R = crate::FieldReader;
#[doc = "Field `I3CEXTLCNT3` writer - I3C_EXT_LCNT_3"]
pub type I3cextlcnt3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `I3CEXTLCNT4` reader - I3C_EXT_LCNT_4"]
pub type I3cextlcnt4R = crate::FieldReader;
#[doc = "Field `I3CEXTLCNT4` writer - I3C_EXT_LCNT_4"]
pub type I3cextlcnt4W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - I3C_EXT_LCNT_1"]
    #[inline(always)]
    pub fn i3cextlcnt1(&self) -> I3cextlcnt1R {
        I3cextlcnt1R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - I3C_EXT_LCNT_2"]
    #[inline(always)]
    pub fn i3cextlcnt2(&self) -> I3cextlcnt2R {
        I3cextlcnt2R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - I3C_EXT_LCNT_3"]
    #[inline(always)]
    pub fn i3cextlcnt3(&self) -> I3cextlcnt3R {
        I3cextlcnt3R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - I3C_EXT_LCNT_4"]
    #[inline(always)]
    pub fn i3cextlcnt4(&self) -> I3cextlcnt4R {
        I3cextlcnt4R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - I3C_EXT_LCNT_1"]
    #[inline(always)]
    pub fn i3cextlcnt1(&mut self) -> I3cextlcnt1W<I3cd0c8Spec> {
        I3cextlcnt1W::new(self, 0)
    }
    #[doc = "Bits 8:15 - I3C_EXT_LCNT_2"]
    #[inline(always)]
    pub fn i3cextlcnt2(&mut self) -> I3cextlcnt2W<I3cd0c8Spec> {
        I3cextlcnt2W::new(self, 8)
    }
    #[doc = "Bits 16:23 - I3C_EXT_LCNT_3"]
    #[inline(always)]
    pub fn i3cextlcnt3(&mut self) -> I3cextlcnt3W<I3cd0c8Spec> {
        I3cextlcnt3W::new(self, 16)
    }
    #[doc = "Bits 24:31 - I3C_EXT_LCNT_4"]
    #[inline(always)]
    pub fn i3cextlcnt4(&mut self) -> I3cextlcnt4W<I3cd0c8Spec> {
        I3cextlcnt4W::new(self, 24)
    }
}
#[doc = "SCL Extended Low Count Timing Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cd0c8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cd0c8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3cd0c8Spec;
impl crate::RegisterSpec for I3cd0c8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3cd0c8::R`](R) reader structure"]
impl crate::Readable for I3cd0c8Spec {}
#[doc = "`write(|w| ..)` method takes [`i3cd0c8::W`](W) writer structure"]
impl crate::Writable for I3cd0c8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CD0C8 to value 0"]
impl crate::Resettable for I3cd0c8Spec {}
