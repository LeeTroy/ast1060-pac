#[doc = "Register `SPI0B8` reader"]
pub type R = crate::R<Spi0b8Spec>;
#[doc = "Register `SPI0B8` writer"]
pub type W = crate::W<Spi0b8Spec>;
#[doc = "Field `SegmentsLowerBoundAddrBit27122` reader - Segments Lower bound address bit\\[27:12\\]"]
pub type SegmentsLowerBoundAddrBit27122R = crate::FieldReader<u16>;
#[doc = "Field `SegmentsLowerBoundAddrBit27122` writer - Segments Lower bound address bit\\[27:12\\]"]
pub type SegmentsLowerBoundAddrBit27122W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `SegmentsUpperBoundAddrBit27122` reader - Segments Upper bound address bit\\[27:12\\]"]
pub type SegmentsUpperBoundAddrBit27122R = crate::FieldReader<u16>;
#[doc = "Field `SegmentsUpperBoundAddrBit27122` writer - Segments Upper bound address bit\\[27:12\\]"]
pub type SegmentsUpperBoundAddrBit27122W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Segments Lower bound address bit\\[27:12\\]"]
    #[inline(always)]
    pub fn segments_lower_bound_addr_bit27122(&self) -> SegmentsLowerBoundAddrBit27122R {
        SegmentsLowerBoundAddrBit27122R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Segments Upper bound address bit\\[27:12\\]"]
    #[inline(always)]
    pub fn segments_upper_bound_addr_bit27122(&self) -> SegmentsUpperBoundAddrBit27122R {
        SegmentsUpperBoundAddrBit27122R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Segments Lower bound address bit\\[27:12\\]"]
    #[inline(always)]
    pub fn segments_lower_bound_addr_bit27122(
        &mut self,
    ) -> SegmentsLowerBoundAddrBit27122W<Spi0b8Spec> {
        SegmentsLowerBoundAddrBit27122W::new(self, 0)
    }
    #[doc = "Bits 16:31 - Segments Upper bound address bit\\[27:12\\]"]
    #[inline(always)]
    pub fn segments_upper_bound_addr_bit27122(
        &mut self,
    ) -> SegmentsUpperBoundAddrBit27122W<Spi0b8Spec> {
        SegmentsUpperBoundAddrBit27122W::new(self, 16)
    }
}
#[doc = "Write Address Filter Register \\#3\n\nYou can [`read`](crate::Reg::read) this register and get [`spi0b8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi0b8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spi0b8Spec;
impl crate::RegisterSpec for Spi0b8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi0b8::R`](R) reader structure"]
impl crate::Readable for Spi0b8Spec {}
#[doc = "`write(|w| ..)` method takes [`spi0b8::W`](W) writer structure"]
impl crate::Writable for Spi0b8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPI0B8 to value 0"]
impl crate::Resettable for Spi0b8Spec {}
