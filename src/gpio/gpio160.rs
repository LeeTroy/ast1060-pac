#[doc = "Register `GPIO160` reader"]
pub type R = crate::R<Gpio160Spec>;
#[doc = "Register `GPIO160` writer"]
pub type W = crate::W<Gpio160Spec>;
#[doc = "Field `PortGPIU70DebounceSettingReg1` reader - Port GPIU\\[7:0\\] debounce setting register #1"]
pub type PortGpiu70debounceSettingReg1R = crate::FieldReader;
#[doc = "Field `PortGPIU70DebounceSettingReg1` writer - Port GPIU\\[7:0\\] debounce setting register #1"]
pub type PortGpiu70debounceSettingReg1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Port GPIU\\[7:0\\] debounce setting register #1"]
    #[inline(always)]
    pub fn port_gpiu70debounce_setting_reg1(&self) -> PortGpiu70debounceSettingReg1R {
        PortGpiu70debounceSettingReg1R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port GPIU\\[7:0\\] debounce setting register #1"]
    #[inline(always)]
    pub fn port_gpiu70debounce_setting_reg1(
        &mut self,
    ) -> PortGpiu70debounceSettingReg1W<Gpio160Spec> {
        PortGpiu70debounceSettingReg1W::new(self, 0)
    }
}
#[doc = "GPIO\\_U Debounce Setting Register \\#1\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio160::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio160::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio160Spec;
impl crate::RegisterSpec for Gpio160Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio160::R`](R) reader structure"]
impl crate::Readable for Gpio160Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio160::W`](W) writer structure"]
impl crate::Writable for Gpio160Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO160 to value 0"]
impl crate::Resettable for Gpio160Spec {}
