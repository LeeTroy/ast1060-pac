#[doc = "Register `GPIO0D0` reader"]
pub type R = crate::R<Gpio0d0Spec>;
#[doc = "Register `GPIO0D0` writer"]
pub type W = crate::W<Gpio0d0Spec>;
#[doc = "Field `DataWrittenToGPIO080GPIOTGPIOSGPIORGPIOQ` reader - Data written to GPIO080.(GPIOT/GPIOS/GPIOR/GPIOQ)"]
pub type DataWrittenToGpio080gpiotgpiosgpiorgpioqR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Data written to GPIO080.(GPIOT/GPIOS/GPIOR/GPIOQ)"]
    #[inline(always)]
    pub fn data_written_to_gpio080gpiotgpiosgpiorgpioq(
        &self,
    ) -> DataWrittenToGpio080gpiotgpiosgpiorgpioqR {
        DataWrittenToGpio080gpiotgpiosgpiorgpioqR::new(self.bits)
    }
}
impl W {}
#[doc = "GPIO\\_Q/R/S/T Data Read Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio0d0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio0d0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio0d0Spec;
impl crate::RegisterSpec for Gpio0d0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio0d0::R`](R) reader structure"]
impl crate::Readable for Gpio0d0Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio0d0::W`](W) writer structure"]
impl crate::Writable for Gpio0d0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO0D0 to value 0"]
impl crate::Resettable for Gpio0d0Spec {}
