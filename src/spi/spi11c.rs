#[doc = "Register `SPI11C` reader"]
pub type R = crate::R<Spi11cSpec>;
#[doc = "Register `SPI11C` writer"]
pub type W = crate::W<Spi11cSpec>;
#[doc = "Field `FullyQualifiedCmd16` reader - Fully qualified Command"]
pub type FullyQualifiedCmd16R = crate::FieldReader;
#[doc = "Field `FullyQualifiedCmd16` writer - Fully qualified Command"]
pub type FullyQualifiedCmd16W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `FullyQualifiedCmd7` reader - Fully qualified Command"]
pub type FullyQualifiedCmd7R = crate::FieldReader;
#[doc = "Field `FullyQualifiedCmd7` writer - Fully qualified Command"]
pub type FullyQualifiedCmd7W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `Reserved7` reader - Reserved"]
pub type Reserved7R = crate::FieldReader<u16>;
#[doc = "Field `EnblEntry7` reader - Enable Entry"]
pub type EnblEntry7R = crate::BitReader;
#[doc = "Field `EnblEntry7` writer - Enable Entry"]
pub type EnblEntry7W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Fully qualified Command"]
    #[inline(always)]
    pub fn fully_qualified_cmd16(&self) -> FullyQualifiedCmd16R {
        FullyQualifiedCmd16R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Fully qualified Command"]
    #[inline(always)]
    pub fn fully_qualified_cmd7(&self) -> FullyQualifiedCmd7R {
        FullyQualifiedCmd7R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:30 - Reserved"]
    #[inline(always)]
    pub fn reserved7(&self) -> Reserved7R {
        Reserved7R::new(((self.bits >> 16) & 0x7fff) as u16)
    }
    #[doc = "Bit 31 - Enable Entry"]
    #[inline(always)]
    pub fn enbl_entry7(&self) -> EnblEntry7R {
        EnblEntry7R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Fully qualified Command"]
    #[inline(always)]
    pub fn fully_qualified_cmd16(&mut self) -> FullyQualifiedCmd16W<Spi11cSpec> {
        FullyQualifiedCmd16W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Fully qualified Command"]
    #[inline(always)]
    pub fn fully_qualified_cmd7(&mut self) -> FullyQualifiedCmd7W<Spi11cSpec> {
        FullyQualifiedCmd7W::new(self, 8)
    }
    #[doc = "Bit 31 - Enable Entry"]
    #[inline(always)]
    pub fn enbl_entry7(&mut self) -> EnblEntry7W<Spi11cSpec> {
        EnblEntry7W::new(self, 31)
    }
}
#[doc = "Fully Qualified Command\n\nYou can [`read`](crate::Reg::read) this register and get [`spi11c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi11c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spi11cSpec;
impl crate::RegisterSpec for Spi11cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi11c::R`](R) reader structure"]
impl crate::Readable for Spi11cSpec {}
#[doc = "`write(|w| ..)` method takes [`spi11c::W`](W) writer structure"]
impl crate::Writable for Spi11cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPI11C to value 0"]
impl crate::Resettable for Spi11cSpec {}
