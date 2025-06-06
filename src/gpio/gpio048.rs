#[doc = "Register `GPIO048` reader"]
pub type R = crate::R<Gpio048Spec>;
#[doc = "Register `GPIO048` writer"]
pub type W = crate::W<Gpio048Spec>;
#[doc = "Field `PortGPIOE70DebounceSettingReg1` reader - Port GPIOE\\[7:0\\] debounce setting register #1"]
pub type PortGpioe70debounceSettingReg1R = crate::FieldReader;
#[doc = "Field `PortGPIOE70DebounceSettingReg1` writer - Port GPIOE\\[7:0\\] debounce setting register #1"]
pub type PortGpioe70debounceSettingReg1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortGPIOF70DebounceSettingReg1` reader - Port GPIOF\\[7:0\\] debounce setting register #1"]
pub type PortGpiof70debounceSettingReg1R = crate::FieldReader;
#[doc = "Field `PortGPIOF70DebounceSettingReg1` writer - Port GPIOF\\[7:0\\] debounce setting register #1"]
pub type PortGpiof70debounceSettingReg1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortGPIOG70DebounceSettingReg1` reader - Port GPIOG\\[7:0\\] debounce setting register #1"]
pub type PortGpiog70debounceSettingReg1R = crate::FieldReader;
#[doc = "Field `PortGPIOG70DebounceSettingReg1` writer - Port GPIOG\\[7:0\\] debounce setting register #1"]
pub type PortGpiog70debounceSettingReg1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortGPIOH70DebounceSettingReg1` reader - Port GPIOH\\[7:0\\] debounce setting register #1"]
pub type PortGpioh70debounceSettingReg1R = crate::FieldReader;
#[doc = "Field `PortGPIOH70DebounceSettingReg1` writer - Port GPIOH\\[7:0\\] debounce setting register #1"]
pub type PortGpioh70debounceSettingReg1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Port GPIOE\\[7:0\\] debounce setting register #1"]
    #[inline(always)]
    pub fn port_gpioe70debounce_setting_reg1(&self) -> PortGpioe70debounceSettingReg1R {
        PortGpioe70debounceSettingReg1R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port GPIOF\\[7:0\\] debounce setting register #1"]
    #[inline(always)]
    pub fn port_gpiof70debounce_setting_reg1(&self) -> PortGpiof70debounceSettingReg1R {
        PortGpiof70debounceSettingReg1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Port GPIOG\\[7:0\\] debounce setting register #1"]
    #[inline(always)]
    pub fn port_gpiog70debounce_setting_reg1(&self) -> PortGpiog70debounceSettingReg1R {
        PortGpiog70debounceSettingReg1R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Port GPIOH\\[7:0\\] debounce setting register #1"]
    #[inline(always)]
    pub fn port_gpioh70debounce_setting_reg1(&self) -> PortGpioh70debounceSettingReg1R {
        PortGpioh70debounceSettingReg1R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port GPIOE\\[7:0\\] debounce setting register #1"]
    #[inline(always)]
    pub fn port_gpioe70debounce_setting_reg1(
        &mut self,
    ) -> PortGpioe70debounceSettingReg1W<Gpio048Spec> {
        PortGpioe70debounceSettingReg1W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Port GPIOF\\[7:0\\] debounce setting register #1"]
    #[inline(always)]
    pub fn port_gpiof70debounce_setting_reg1(
        &mut self,
    ) -> PortGpiof70debounceSettingReg1W<Gpio048Spec> {
        PortGpiof70debounceSettingReg1W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Port GPIOG\\[7:0\\] debounce setting register #1"]
    #[inline(always)]
    pub fn port_gpiog70debounce_setting_reg1(
        &mut self,
    ) -> PortGpiog70debounceSettingReg1W<Gpio048Spec> {
        PortGpiog70debounceSettingReg1W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Port GPIOH\\[7:0\\] debounce setting register #1"]
    #[inline(always)]
    pub fn port_gpioh70debounce_setting_reg1(
        &mut self,
    ) -> PortGpioh70debounceSettingReg1W<Gpio048Spec> {
        PortGpioh70debounceSettingReg1W::new(self, 24)
    }
}
#[doc = "GPIO\\_E/F/G/H Debounce Setting Register \\#1\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio048::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio048::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio048Spec;
impl crate::RegisterSpec for Gpio048Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio048::R`](R) reader structure"]
impl crate::Readable for Gpio048Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio048::W`](W) writer structure"]
impl crate::Writable for Gpio048Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO048 to value 0"]
impl crate::Resettable for Gpio048Spec {}
