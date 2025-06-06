#[doc = "Register `WDT028` reader"]
pub type R = crate::R<Wdt028Spec>;
#[doc = "Register `WDT028` writer"]
pub type W = crate::W<Wdt028Spec>;
#[doc = "Field `EnblRstARMAndRelatedCtrls` reader - Enable reset ARM and related controllers"]
pub type EnblRstArmandRelatedCtrlsR = crate::BitReader;
#[doc = "Field `EnblRstARMAndRelatedCtrls` writer - Enable reset ARM and related controllers"]
pub type EnblRstArmandRelatedCtrlsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblRstSRAMCtrl` reader - Enable reset SRAM controller"]
pub type EnblRstSramctrlR = crate::BitReader;
#[doc = "Field `EnblRstSRAMCtrl` writer - Enable reset SRAM controller"]
pub type EnblRstSramctrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblRstAHBBridges` reader - Enable reset AHB bridges"]
pub type EnblRstAhbbridgesR = crate::BitReader;
#[doc = "Field `EnblRstAHBBridges` writer - Enable reset AHB bridges"]
pub type EnblRstAhbbridgesW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - Reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `Reserved4` writer - Reserved"]
pub type Reserved4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblRstSOCCtrl` reader - Enable reset SOC controller"]
pub type EnblRstSocctrlR = crate::BitReader;
#[doc = "Field `EnblRstSOCCtrl` writer - Enable reset SOC controller"]
pub type EnblRstSocctrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - Reserved"]
pub type Reserved2R = crate::FieldReader;
#[doc = "Field `Reserved2` writer - Reserved"]
pub type Reserved2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `Reserved3` reader - Reserved"]
pub type Reserved3R = crate::FieldReader<u16>;
#[doc = "Field `Reserved3` writer - Reserved"]
pub type Reserved3W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `EnblRstUSBDevCtrl` reader - Enable reset USB Dev controller"]
pub type EnblRstUsbdevCtrlR = crate::BitReader;
#[doc = "Field `EnblRstUSBDevCtrl` writer - Enable reset USB Dev controller"]
pub type EnblRstUsbdevCtrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `Reserved1` writer - Reserved"]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `EnblRstHACEng` reader - Enable reset HAC engine"]
pub type EnblRstHacengR = crate::BitReader;
#[doc = "Field `EnblRstHACEng` writer - Enable reset HAC engine"]
pub type EnblRstHacengW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable reset ARM and related controllers"]
    #[inline(always)]
    pub fn enbl_rst_armand_related_ctrls(&self) -> EnblRstArmandRelatedCtrlsR {
        EnblRstArmandRelatedCtrlsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable reset SRAM controller"]
    #[inline(always)]
    pub fn enbl_rst_sramctrl(&self) -> EnblRstSramctrlR {
        EnblRstSramctrlR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable reset AHB bridges"]
    #[inline(always)]
    pub fn enbl_rst_ahbbridges(&self) -> EnblRstAhbbridgesR {
        EnblRstAhbbridgesR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable reset SOC controller"]
    #[inline(always)]
    pub fn enbl_rst_socctrl(&self) -> EnblRstSocctrlR {
        EnblRstSocctrlR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bits 5:14 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 5) & 0x03ff) as u16)
    }
    #[doc = "Bit 7 - Enable reset USB Dev controller"]
    #[inline(always)]
    pub fn enbl_rst_usbdev_ctrl(&self) -> EnblRstUsbdevCtrlR {
        EnblRstUsbdevCtrlR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:14 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 15 - Enable reset HAC engine"]
    #[inline(always)]
    pub fn enbl_rst_haceng(&self) -> EnblRstHacengR {
        EnblRstHacengR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable reset ARM and related controllers"]
    #[inline(always)]
    pub fn enbl_rst_armand_related_ctrls(&mut self) -> EnblRstArmandRelatedCtrlsW<Wdt028Spec> {
        EnblRstArmandRelatedCtrlsW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable reset SRAM controller"]
    #[inline(always)]
    pub fn enbl_rst_sramctrl(&mut self) -> EnblRstSramctrlW<Wdt028Spec> {
        EnblRstSramctrlW::new(self, 1)
    }
    #[doc = "Bit 2 - Enable reset AHB bridges"]
    #[inline(always)]
    pub fn enbl_rst_ahbbridges(&mut self) -> EnblRstAhbbridgesW<Wdt028Spec> {
        EnblRstAhbbridgesW::new(self, 2)
    }
    #[doc = "Bit 3 - Reserved"]
    #[inline(always)]
    pub fn reserved4(&mut self) -> Reserved4W<Wdt028Spec> {
        Reserved4W::new(self, 3)
    }
    #[doc = "Bit 4 - Enable reset SOC controller"]
    #[inline(always)]
    pub fn enbl_rst_socctrl(&mut self) -> EnblRstSocctrlW<Wdt028Spec> {
        EnblRstSocctrlW::new(self, 4)
    }
    #[doc = "Bits 5:6 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&mut self) -> Reserved2W<Wdt028Spec> {
        Reserved2W::new(self, 5)
    }
    #[doc = "Bits 5:14 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&mut self) -> Reserved3W<Wdt028Spec> {
        Reserved3W::new(self, 5)
    }
    #[doc = "Bit 7 - Enable reset USB Dev controller"]
    #[inline(always)]
    pub fn enbl_rst_usbdev_ctrl(&mut self) -> EnblRstUsbdevCtrlW<Wdt028Spec> {
        EnblRstUsbdevCtrlW::new(self, 7)
    }
    #[doc = "Bits 8:14 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<Wdt028Spec> {
        Reserved1W::new(self, 8)
    }
    #[doc = "Bit 15 - Enable reset HAC engine"]
    #[inline(always)]
    pub fn enbl_rst_haceng(&mut self) -> EnblRstHacengW<Wdt028Spec> {
        EnblRstHacengW::new(self, 15)
    }
}
#[doc = "WDTn Software Mode Reset Mask Register \\#1)\n\nYou can [`read`](crate::Reg::read) this register and get [`wdt028::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdt028::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Wdt028Spec;
impl crate::RegisterSpec for Wdt028Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdt028::R`](R) reader structure"]
impl crate::Readable for Wdt028Spec {}
#[doc = "`write(|w| ..)` method takes [`wdt028::W`](W) writer structure"]
impl crate::Writable for Wdt028Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WDT028 to value 0"]
impl crate::Resettable for Wdt028Spec {}
