#[doc = "Register `GPIO0D4` reader"]
pub type R = crate::R<Gpio0d4Spec>;
#[doc = "Register `GPIO0D4` writer"]
pub type W = crate::W<Gpio0d4Spec>;
#[doc = "Field `DataWrittenToGPIO088GPIU` reader - Data written to GPIO088.(GPIU)"]
pub type DataWrittenToGpio088gpiuR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Data written to GPIO088.(GPIU)"]
    #[inline(always)]
    pub fn data_written_to_gpio088gpiu(&self) -> DataWrittenToGpio088gpiuR {
        DataWrittenToGpio088gpiuR::new((self.bits & 0xff) as u8)
    }
}
impl W {}
#[doc = "GPIO\\_U Data Read Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio0d4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio0d4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio0d4Spec;
impl crate::RegisterSpec for Gpio0d4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio0d4::R`](R) reader structure"]
impl crate::Readable for Gpio0d4Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio0d4::W`](W) writer structure"]
impl crate::Writable for Gpio0d4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO0D4 to value 0"]
impl crate::Resettable for Gpio0d4Spec {}
