#[doc = "Register `FMC10C` reader"]
pub type R = crate::R<Fmc10cSpec>;
#[doc = "Register `FMC10C` writer"]
pub type W = crate::W<Fmc10cSpec>;
#[doc = "Field `FullyQualifiedCmd12` reader - Fully qualified Command"]
pub type FullyQualifiedCmd12R = crate::FieldReader;
#[doc = "Field `FullyQualifiedCmd12` writer - Fully qualified Command"]
pub type FullyQualifiedCmd12W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `FullyQualifiedCmd3` reader - Fully qualified Command"]
pub type FullyQualifiedCmd3R = crate::FieldReader;
#[doc = "Field `FullyQualifiedCmd3` writer - Fully qualified Command"]
pub type FullyQualifiedCmd3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `Reserved3` reader - Reserved"]
pub type Reserved3R = crate::FieldReader<u16>;
#[doc = "Field `EnblEntry3` reader - Enable Entry"]
pub type EnblEntry3R = crate::BitReader;
#[doc = "Field `EnblEntry3` writer - Enable Entry"]
pub type EnblEntry3W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Fully qualified Command"]
    #[inline(always)]
    pub fn fully_qualified_cmd12(&self) -> FullyQualifiedCmd12R {
        FullyQualifiedCmd12R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Fully qualified Command"]
    #[inline(always)]
    pub fn fully_qualified_cmd3(&self) -> FullyQualifiedCmd3R {
        FullyQualifiedCmd3R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:30 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 16) & 0x7fff) as u16)
    }
    #[doc = "Bit 31 - Enable Entry"]
    #[inline(always)]
    pub fn enbl_entry3(&self) -> EnblEntry3R {
        EnblEntry3R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Fully qualified Command"]
    #[inline(always)]
    pub fn fully_qualified_cmd12(&mut self) -> FullyQualifiedCmd12W<Fmc10cSpec> {
        FullyQualifiedCmd12W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Fully qualified Command"]
    #[inline(always)]
    pub fn fully_qualified_cmd3(&mut self) -> FullyQualifiedCmd3W<Fmc10cSpec> {
        FullyQualifiedCmd3W::new(self, 8)
    }
    #[doc = "Bit 31 - Enable Entry"]
    #[inline(always)]
    pub fn enbl_entry3(&mut self) -> EnblEntry3W<Fmc10cSpec> {
        EnblEntry3W::new(self, 31)
    }
}
#[doc = "Fully Qualified Command\n\nYou can [`read`](crate::Reg::read) this register and get [`fmc10c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmc10c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fmc10cSpec;
impl crate::RegisterSpec for Fmc10cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fmc10c::R`](R) reader structure"]
impl crate::Readable for Fmc10cSpec {}
#[doc = "`write(|w| ..)` method takes [`fmc10c::W`](W) writer structure"]
impl crate::Writable for Fmc10cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FMC10C to value 0x8000_0005"]
impl crate::Resettable for Fmc10cSpec {
    const RESET_VALUE: u32 = 0x8000_0005;
}
