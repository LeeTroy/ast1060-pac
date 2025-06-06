#[doc = "Register `GPIO164` reader"]
pub type R = crate::R<Gpio164Spec>;
#[doc = "Register `GPIO164` writer"]
pub type W = crate::W<Gpio164Spec>;
#[doc = "Field `PortGPIU70DebounceSettingReg2` reader - Port GPIU\\[7:0\\] debounce setting register #2"]
pub type PortGpiu70debounceSettingReg2R = crate::FieldReader;
#[doc = "Field `PortGPIU70DebounceSettingReg2` writer - Port GPIU\\[7:0\\] debounce setting register #2"]
pub type PortGpiu70debounceSettingReg2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Port GPIU\\[7:0\\] debounce setting register #2"]
    #[inline(always)]
    pub fn port_gpiu70debounce_setting_reg2(&self) -> PortGpiu70debounceSettingReg2R {
        PortGpiu70debounceSettingReg2R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port GPIU\\[7:0\\] debounce setting register #2"]
    #[inline(always)]
    pub fn port_gpiu70debounce_setting_reg2(
        &mut self,
    ) -> PortGpiu70debounceSettingReg2W<Gpio164Spec> {
        PortGpiu70debounceSettingReg2W::new(self, 0)
    }
}
#[doc = "GPIO\\_U Debounce Setting Register \\#2\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio164::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio164::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio164Spec;
impl crate::RegisterSpec for Gpio164Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio164::R`](R) reader structure"]
impl crate::Readable for Gpio164Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio164::W`](W) writer structure"]
impl crate::Writable for Gpio164Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO164 to value 0"]
impl crate::Resettable for Gpio164Spec {}
