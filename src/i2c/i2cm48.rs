#[doc = "Register `I2CM48` reader"]
pub type R = crate::R<I2cm48Spec>;
#[doc = "Register `I2CM48` writer"]
pub type W = crate::W<I2cm48Spec>;
#[doc = "Field `DMATxActualLenByte` reader - DMA Tx actual length (Byte)"]
pub type DmatxActualLenByteR = crate::FieldReader<u16>;
#[doc = "Field `DMATxActualLenByte` writer - DMA Tx actual length (Byte)"]
pub type DmatxActualLenByteW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
#[doc = "Field `DMARxActualLenByte` reader - DMA Rx actual length (Byte)"]
pub type DmarxActualLenByteR = crate::FieldReader<u16>;
#[doc = "Field `DMARxActualLenByte` writer - DMA Rx actual length (Byte)"]
pub type DmarxActualLenByteW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 0:12 - DMA Tx actual length (Byte)"]
    #[inline(always)]
    pub fn dmatx_actual_len_byte(&self) -> DmatxActualLenByteR {
        DmatxActualLenByteR::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 16:28 - DMA Rx actual length (Byte)"]
    #[inline(always)]
    pub fn dmarx_actual_len_byte(&self) -> DmarxActualLenByteR {
        DmarxActualLenByteR::new(((self.bits >> 16) & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - DMA Tx actual length (Byte)"]
    #[inline(always)]
    pub fn dmatx_actual_len_byte(&mut self) -> DmatxActualLenByteW<I2cm48Spec> {
        DmatxActualLenByteW::new(self, 0)
    }
    #[doc = "Bits 16:28 - DMA Rx actual length (Byte)"]
    #[inline(always)]
    pub fn dmarx_actual_len_byte(&mut self) -> DmarxActualLenByteW<I2cm48Spec> {
        DmarxActualLenByteW::new(self, 16)
    }
}
#[doc = "Master DMA Length Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2cm48::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2cm48::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2cm48Spec;
impl crate::RegisterSpec for I2cm48Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2cm48::R`](R) reader structure"]
impl crate::Readable for I2cm48Spec {}
#[doc = "`write(|w| ..)` method takes [`i2cm48::W`](W) writer structure"]
impl crate::Writable for I2cm48Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2CM48 to value 0"]
impl crate::Resettable for I2cm48Spec {}
