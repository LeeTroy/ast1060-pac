#[doc = "Register `I2CM30` reader"]
pub type R = crate::R<I2cm30Spec>;
#[doc = "Register `I2CM30` writer"]
pub type W = crate::W<I2cm30Spec>;
#[doc = "Field `SDRAMDMABufferBaseAddr` reader - SDRAM DMA Buffer Base Address"]
pub type SdramdmabufferBaseAddrR = crate::FieldReader<u32>;
#[doc = "Field `SDRAMDMABufferBaseAddr` writer - SDRAM DMA Buffer Base Address"]
pub type SdramdmabufferBaseAddrW<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
#[doc = "Field `Reserved0x1` reader - Reserved (0x1)"]
pub type Reserved0x1R = crate::BitReader;
impl R {
    #[doc = "Bits 0:30 - SDRAM DMA Buffer Base Address"]
    #[inline(always)]
    pub fn sdramdmabuffer_base_addr(&self) -> SdramdmabufferBaseAddrR {
        SdramdmabufferBaseAddrR::new(self.bits & 0x7fff_ffff)
    }
    #[doc = "Bit 31 - Reserved (0x1)"]
    #[inline(always)]
    pub fn reserved0x1(&self) -> Reserved0x1R {
        Reserved0x1R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:30 - SDRAM DMA Buffer Base Address"]
    #[inline(always)]
    pub fn sdramdmabuffer_base_addr(&mut self) -> SdramdmabufferBaseAddrW<I2cm30Spec> {
        SdramdmabufferBaseAddrW::new(self, 0)
    }
}
#[doc = "\newver{Master DMA Mode Tx Buffer Base Address\n\nYou can [`read`](crate::Reg::read) this register and get [`i2cm30::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2cm30::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2cm30Spec;
impl crate::RegisterSpec for I2cm30Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2cm30::R`](R) reader structure"]
impl crate::Readable for I2cm30Spec {}
#[doc = "`write(|w| ..)` method takes [`i2cm30::W`](W) writer structure"]
impl crate::Writable for I2cm30Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2CM30 to value 0"]
impl crate::Resettable for I2cm30Spec {}
