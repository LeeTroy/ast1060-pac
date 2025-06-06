#[doc = "Register `GPIO0C0` reader"]
pub type R = crate::R<Gpio0c0Spec>;
#[doc = "Register `GPIO0C0` writer"]
pub type W = crate::W<Gpio0c0Spec>;
#[doc = "Field `DataWrittenToGPIO000GPIODGPIOCGPIOBGPIOA` reader - Data written to GPIO000.(GPIOD/GPIOC/GPIOB/GPIOA)"]
pub type DataWrittenToGpio000gpiodgpiocgpiobgpioaR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Data written to GPIO000.(GPIOD/GPIOC/GPIOB/GPIOA)"]
    #[inline(always)]
    pub fn data_written_to_gpio000gpiodgpiocgpiobgpioa(
        &self,
    ) -> DataWrittenToGpio000gpiodgpiocgpiobgpioaR {
        DataWrittenToGpio000gpiodgpiocgpiobgpioaR::new(self.bits)
    }
}
impl W {}
#[doc = "GPIO\\_A/B/C/D Data Read Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio0c0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio0c0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio0c0Spec;
impl crate::RegisterSpec for Gpio0c0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio0c0::R`](R) reader structure"]
impl crate::Readable for Gpio0c0Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio0c0::W`](W) writer structure"]
impl crate::Writable for Gpio0c0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO0C0 to value 0"]
impl crate::Resettable for Gpio0c0Spec {}
