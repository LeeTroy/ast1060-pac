#[doc = "Register `WDT024` reader"]
pub type R = crate::R<Wdt024Spec>;
#[doc = "Register `WDT024` writer"]
pub type W = crate::W<Wdt024Spec>;
#[doc = "Field `EnblTriggerSwModeRst` reader - Enable trigger software mode reset"]
pub type EnblTriggerSwModeRstR = crate::BitReader;
#[doc = "Field `EnblTriggerSwModeRst` writer - Enable trigger software mode reset"]
pub type EnblTriggerSwModeRstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `WDEventCounter` reader - Watchdog event counter"]
pub type WdeventCounterR = crate::FieldReader;
#[doc = "Field `WDEventCounter` writer - Watchdog event counter"]
pub type WdeventCounterW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - Enable trigger software mode reset"]
    #[inline(always)]
    pub fn enbl_trigger_sw_mode_rst(&self) -> EnblTriggerSwModeRstR {
        EnblTriggerSwModeRstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bits 4:7 - Watchdog event counter"]
    #[inline(always)]
    pub fn wdevent_counter(&self) -> WdeventCounterR {
        WdeventCounterR::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable trigger software mode reset"]
    #[inline(always)]
    pub fn enbl_trigger_sw_mode_rst(&mut self) -> EnblTriggerSwModeRstW<Wdt024Spec> {
        EnblTriggerSwModeRstW::new(self, 0)
    }
    #[doc = "Bits 4:7 - Watchdog event counter"]
    #[inline(always)]
    pub fn wdevent_counter(&mut self) -> WdeventCounterW<Wdt024Spec> {
        WdeventCounterW::new(self, 4)
    }
}
#[doc = "WDTn Software Mode Reset Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wdt024::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdt024::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Wdt024Spec;
impl crate::RegisterSpec for Wdt024Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdt024::R`](R) reader structure"]
impl crate::Readable for Wdt024Spec {}
#[doc = "`write(|w| ..)` method takes [`wdt024::W`](W) writer structure"]
impl crate::Writable for Wdt024Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WDT024 to value 0"]
impl crate::Resettable for Wdt024Spec {}
