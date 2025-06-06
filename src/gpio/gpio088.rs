#[doc = "Register `GPIO088` reader"]
pub type R = crate::R<Gpio088Spec>;
#[doc = "Register `GPIO088` writer"]
pub type W = crate::W<Gpio088Spec>;
#[doc = "Field `PortGPIU70DataReg` reader - Port GPIU\\[7:0\\] data register"]
pub type PortGpiu70dataRegR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Port GPIU\\[7:0\\] data register"]
    #[inline(always)]
    pub fn port_gpiu70data_reg(&self) -> PortGpiu70dataRegR {
        PortGpiu70dataRegR::new((self.bits & 0xff) as u8)
    }
}
impl W {}
#[doc = "GPIO\\_U Data Value Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio088::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio088::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio088Spec;
impl crate::RegisterSpec for Gpio088Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio088::R`](R) reader structure"]
impl crate::Readable for Gpio088Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio088::W`](W) writer structure"]
impl crate::Writable for Gpio088Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO088 to value 0"]
impl crate::Resettable for Gpio088Spec {}
