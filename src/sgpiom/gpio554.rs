#[doc = "Register `GPIO554` reader"]
pub type R = crate::R<Gpio554Spec>;
#[doc = "Register `GPIO554` writer"]
pub type W = crate::W<Gpio554Spec>;
#[doc = "Field `EnblOfSerialGPIO` reader - Enable of Serial GPIO"]
pub type EnblOfSerialGpioR = crate::BitReader;
#[doc = "Field `EnblOfSerialGPIO` writer - Enable of Serial GPIO"]
pub type EnblOfSerialGpioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - Reserved"]
pub type Reserved3R = crate::BitReader;
#[doc = "Field `Reserved2` reader - Reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `NumbersOfSerialGPIOPins` reader - Numbers of Serial GPIO pins"]
pub type NumbersOfSerialGpiopinsR = crate::FieldReader;
#[doc = "Field `NumbersOfSerialGPIOPins` writer - Numbers of Serial GPIO pins"]
pub type NumbersOfSerialGpiopinsW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `SerialGPIOClkDivision` reader - Serial GPIO clock division"]
pub type SerialGpioclkDivisionR = crate::FieldReader<u16>;
#[doc = "Field `SerialGPIOClkDivision` writer - Serial GPIO clock division"]
pub type SerialGpioclkDivisionW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - Enable of Serial GPIO"]
    #[inline(always)]
    pub fn enbl_of_serial_gpio(&self) -> EnblOfSerialGpioR {
        EnblOfSerialGpioR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:5 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:10 - Numbers of Serial GPIO pins"]
    #[inline(always)]
    pub fn numbers_of_serial_gpiopins(&self) -> NumbersOfSerialGpiopinsR {
        NumbersOfSerialGpiopinsR::new(((self.bits >> 6) & 0x1f) as u8)
    }
    #[doc = "Bits 16:31 - Serial GPIO clock division"]
    #[inline(always)]
    pub fn serial_gpioclk_division(&self) -> SerialGpioclkDivisionR {
        SerialGpioclkDivisionR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Enable of Serial GPIO"]
    #[inline(always)]
    pub fn enbl_of_serial_gpio(&mut self) -> EnblOfSerialGpioW<Gpio554Spec> {
        EnblOfSerialGpioW::new(self, 0)
    }
    #[doc = "Bits 6:10 - Numbers of Serial GPIO pins"]
    #[inline(always)]
    pub fn numbers_of_serial_gpiopins(&mut self) -> NumbersOfSerialGpiopinsW<Gpio554Spec> {
        NumbersOfSerialGpiopinsW::new(self, 6)
    }
    #[doc = "Bits 16:31 - Serial GPIO clock division"]
    #[inline(always)]
    pub fn serial_gpioclk_division(&mut self) -> SerialGpioclkDivisionW<Gpio554Spec> {
        SerialGpioclkDivisionW::new(self, 16)
    }
}
#[doc = "Serial GPIO 1 Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio554::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio554::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio554Spec;
impl crate::RegisterSpec for Gpio554Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio554::R`](R) reader structure"]
impl crate::Readable for Gpio554Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio554::W`](W) writer structure"]
impl crate::Writable for Gpio554Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO554 to value 0"]
impl crate::Resettable for Gpio554Spec {}
