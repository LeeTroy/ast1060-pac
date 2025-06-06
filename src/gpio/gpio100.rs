#[doc = "Register `GPIO100` reader"]
pub type R = crate::R<Gpio100Spec>;
#[doc = "Register `GPIO100` writer"]
pub type W = crate::W<Gpio100Spec>;
#[doc = "Field `PortGPIOM70DebounceSettingReg1` reader - Port GPIOM\\[7:0\\] debounce setting register #1"]
pub type PortGpiom70debounceSettingReg1R = crate::FieldReader;
#[doc = "Field `PortGPIOM70DebounceSettingReg1` writer - Port GPIOM\\[7:0\\] debounce setting register #1"]
pub type PortGpiom70debounceSettingReg1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortGPION70DebounceSettingReg1` reader - Port GPION\\[7:0\\] debounce setting register #1"]
pub type PortGpion70debounceSettingReg1R = crate::FieldReader;
#[doc = "Field `PortGPION70DebounceSettingReg1` writer - Port GPION\\[7:0\\] debounce setting register #1"]
pub type PortGpion70debounceSettingReg1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortGPIOO70DebounceSettingReg1` reader - Port GPIOO\\[7:0\\] debounce setting register #1"]
pub type PortGpioo70debounceSettingReg1R = crate::FieldReader;
#[doc = "Field `PortGPIOO70DebounceSettingReg1` writer - Port GPIOO\\[7:0\\] debounce setting register #1"]
pub type PortGpioo70debounceSettingReg1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortGPIOP70DebounceSettingReg1` reader - Port GPIOP\\[7:0\\] debounce setting register #1"]
pub type PortGpiop70debounceSettingReg1R = crate::FieldReader;
#[doc = "Field `PortGPIOP70DebounceSettingReg1` writer - Port GPIOP\\[7:0\\] debounce setting register #1"]
pub type PortGpiop70debounceSettingReg1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Port GPIOM\\[7:0\\] debounce setting register #1"]
    #[inline(always)]
    pub fn port_gpiom70debounce_setting_reg1(&self) -> PortGpiom70debounceSettingReg1R {
        PortGpiom70debounceSettingReg1R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port GPION\\[7:0\\] debounce setting register #1"]
    #[inline(always)]
    pub fn port_gpion70debounce_setting_reg1(&self) -> PortGpion70debounceSettingReg1R {
        PortGpion70debounceSettingReg1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Port GPIOO\\[7:0\\] debounce setting register #1"]
    #[inline(always)]
    pub fn port_gpioo70debounce_setting_reg1(&self) -> PortGpioo70debounceSettingReg1R {
        PortGpioo70debounceSettingReg1R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Port GPIOP\\[7:0\\] debounce setting register #1"]
    #[inline(always)]
    pub fn port_gpiop70debounce_setting_reg1(&self) -> PortGpiop70debounceSettingReg1R {
        PortGpiop70debounceSettingReg1R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port GPIOM\\[7:0\\] debounce setting register #1"]
    #[inline(always)]
    pub fn port_gpiom70debounce_setting_reg1(
        &mut self,
    ) -> PortGpiom70debounceSettingReg1W<Gpio100Spec> {
        PortGpiom70debounceSettingReg1W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Port GPION\\[7:0\\] debounce setting register #1"]
    #[inline(always)]
    pub fn port_gpion70debounce_setting_reg1(
        &mut self,
    ) -> PortGpion70debounceSettingReg1W<Gpio100Spec> {
        PortGpion70debounceSettingReg1W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Port GPIOO\\[7:0\\] debounce setting register #1"]
    #[inline(always)]
    pub fn port_gpioo70debounce_setting_reg1(
        &mut self,
    ) -> PortGpioo70debounceSettingReg1W<Gpio100Spec> {
        PortGpioo70debounceSettingReg1W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Port GPIOP\\[7:0\\] debounce setting register #1"]
    #[inline(always)]
    pub fn port_gpiop70debounce_setting_reg1(
        &mut self,
    ) -> PortGpiop70debounceSettingReg1W<Gpio100Spec> {
        PortGpiop70debounceSettingReg1W::new(self, 24)
    }
}
#[doc = "GPIO\\_M/N/O/P Debounce Setting Register \\#1\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio100::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio100::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio100Spec;
impl crate::RegisterSpec for Gpio100Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio100::R`](R) reader structure"]
impl crate::Readable for Gpio100Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio100::W`](W) writer structure"]
impl crate::Writable for Gpio100Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO100 to value 0"]
impl crate::Resettable for Gpio100Spec {}
