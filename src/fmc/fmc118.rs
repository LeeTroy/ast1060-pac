#[doc = "Register `FMC118` reader"]
pub type R = crate::R<Fmc118Spec>;
#[doc = "Register `FMC118` writer"]
pub type W = crate::W<Fmc118Spec>;
#[doc = "Field `FullyQualifiedCmd15` reader - Fully qualified Command"]
pub type FullyQualifiedCmd15R = crate::FieldReader;
#[doc = "Field `FullyQualifiedCmd15` writer - Fully qualified Command"]
pub type FullyQualifiedCmd15W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `FullyQualifiedCmd6` reader - Fully qualified Command"]
pub type FullyQualifiedCmd6R = crate::FieldReader;
#[doc = "Field `FullyQualifiedCmd6` writer - Fully qualified Command"]
pub type FullyQualifiedCmd6W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `Reserved6` reader - Reserved"]
pub type Reserved6R = crate::FieldReader<u16>;
#[doc = "Field `EnblEntry6` reader - Enable Entry"]
pub type EnblEntry6R = crate::BitReader;
#[doc = "Field `EnblEntry6` writer - Enable Entry"]
pub type EnblEntry6W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Fully qualified Command"]
    #[inline(always)]
    pub fn fully_qualified_cmd15(&self) -> FullyQualifiedCmd15R {
        FullyQualifiedCmd15R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Fully qualified Command"]
    #[inline(always)]
    pub fn fully_qualified_cmd6(&self) -> FullyQualifiedCmd6R {
        FullyQualifiedCmd6R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:30 - Reserved"]
    #[inline(always)]
    pub fn reserved6(&self) -> Reserved6R {
        Reserved6R::new(((self.bits >> 16) & 0x7fff) as u16)
    }
    #[doc = "Bit 31 - Enable Entry"]
    #[inline(always)]
    pub fn enbl_entry6(&self) -> EnblEntry6R {
        EnblEntry6R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Fully qualified Command"]
    #[inline(always)]
    pub fn fully_qualified_cmd15(&mut self) -> FullyQualifiedCmd15W<Fmc118Spec> {
        FullyQualifiedCmd15W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Fully qualified Command"]
    #[inline(always)]
    pub fn fully_qualified_cmd6(&mut self) -> FullyQualifiedCmd6W<Fmc118Spec> {
        FullyQualifiedCmd6W::new(self, 8)
    }
    #[doc = "Bit 31 - Enable Entry"]
    #[inline(always)]
    pub fn enbl_entry6(&mut self) -> EnblEntry6W<Fmc118Spec> {
        EnblEntry6W::new(self, 31)
    }
}
#[doc = "Fully Qualified Command\n\nYou can [`read`](crate::Reg::read) this register and get [`fmc118::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmc118::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fmc118Spec;
impl crate::RegisterSpec for Fmc118Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fmc118::R`](R) reader structure"]
impl crate::Readable for Fmc118Spec {}
#[doc = "`write(|w| ..)` method takes [`fmc118::W`](W) writer structure"]
impl crate::Writable for Fmc118Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FMC118 to value 0x8000_0004"]
impl crate::Resettable for Fmc118Spec {
    const RESET_VALUE: u32 = 0x8000_0004;
}
