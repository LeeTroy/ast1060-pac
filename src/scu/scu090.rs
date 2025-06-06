#[doc = "Register `SCU090` reader"]
pub type R = crate::R<Scu090Spec>;
#[doc = "Register `SCU090` writer"]
pub type W = crate::W<Scu090Spec>;
#[doc = "Field `Reserved6` reader - Reserved, must keep at value \"11\""]
pub type Reserved6R = crate::FieldReader;
#[doc = "Field `Reserved6` writer - Reserved, must keep at value \"11\""]
pub type Reserved6W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `REFCLKStopEnbl` reader - REFCLK Stop Enable"]
pub type RefclkstopEnblR = crate::BitReader;
#[doc = "Field `REFCLKStopEnbl` writer - REFCLK Stop Enable"]
pub type RefclkstopEnblW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - Reserved, must keep at value \"111\""]
pub type Reserved5R = crate::FieldReader;
#[doc = "Field `Reserved5` writer - Reserved, must keep at value \"111\""]
pub type Reserved5W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `StopRSACLKECCCLK` reader - Stop RSACLK/ECCCLK"]
pub type StopRsaclkeccclkR = crate::BitReader;
#[doc = "Field `StopRSACLKECCCLK` writer - Stop RSACLK/ECCCLK"]
pub type StopRsaclkeccclkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - Reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `Reserved4` writer - Reserved"]
pub type Reserved4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `StopI3C0CLKForI3C0Ctrl` reader - Stop I3C0CLK (for I3C0 Controller)"]
pub type StopI3c0clkforI3c0ctrlR = crate::BitReader;
#[doc = "Field `StopI3C0CLKForI3C0Ctrl` writer - Stop I3C0CLK (for I3C0 Controller)"]
pub type StopI3c0clkforI3c0ctrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `StopI3C1CLKForI3C1Ctrl` reader - Stop I3C1CLK (for I3C1 Controller)"]
pub type StopI3c1clkforI3c1ctrlR = crate::BitReader;
#[doc = "Field `StopI3C1CLKForI3C1Ctrl` writer - Stop I3C1CLK (for I3C1 Controller)"]
pub type StopI3c1clkforI3c1ctrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `StopI3C2CLKForI3C2Ctrl` reader - Stop I3C2CLK (for I3C2 Controller)"]
pub type StopI3c2clkforI3c2ctrlR = crate::BitReader;
#[doc = "Field `StopI3C2CLKForI3C2Ctrl` writer - Stop I3C2CLK (for I3C2 Controller)"]
pub type StopI3c2clkforI3c2ctrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `StopI3C3CLKForI3C3Ctrl` reader - Stop I3C3CLK (for I3C3 Controller)"]
pub type StopI3c3clkforI3c3ctrlR = crate::BitReader;
#[doc = "Field `StopI3C3CLKForI3C3Ctrl` writer - Stop I3C3CLK (for I3C3 Controller)"]
pub type StopI3c3clkforI3c3ctrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - Reserved, must keep at value \"111111111\""]
pub type Reserved3R = crate::FieldReader<u16>;
#[doc = "Field `Reserved3` writer - Reserved, must keep at value \"111111111\""]
pub type Reserved3W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `Reserved2` reader - Reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `Reserved1` reader - Reserved, must keep at value \"11111111\""]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `Reserved1` writer - Reserved, must keep at value \"11111111\""]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:1 - Reserved, must keep at value \"11\""]
    #[inline(always)]
    pub fn reserved6(&self) -> Reserved6R {
        Reserved6R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - REFCLK Stop Enable"]
    #[inline(always)]
    pub fn refclkstop_enbl(&self) -> RefclkstopEnblR {
        RefclkstopEnblR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:5 - Reserved, must keep at value \"111\""]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bit 6 - Stop RSACLK/ECCCLK"]
    #[inline(always)]
    pub fn stop_rsaclkeccclk(&self) -> StopRsaclkeccclkR {
        StopRsaclkeccclkR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Stop I3C0CLK (for I3C0 Controller)"]
    #[inline(always)]
    pub fn stop_i3c0clkfor_i3c0ctrl(&self) -> StopI3c0clkforI3c0ctrlR {
        StopI3c0clkforI3c0ctrlR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Stop I3C1CLK (for I3C1 Controller)"]
    #[inline(always)]
    pub fn stop_i3c1clkfor_i3c1ctrl(&self) -> StopI3c1clkforI3c1ctrlR {
        StopI3c1clkforI3c1ctrlR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Stop I3C2CLK (for I3C2 Controller)"]
    #[inline(always)]
    pub fn stop_i3c2clkfor_i3c2ctrl(&self) -> StopI3c2clkforI3c2ctrlR {
        StopI3c2clkforI3c2ctrlR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Stop I3C3CLK (for I3C3 Controller)"]
    #[inline(always)]
    pub fn stop_i3c3clkfor_i3c3ctrl(&self) -> StopI3c3clkforI3c3ctrlR {
        StopI3c3clkforI3c3ctrlR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:20 - Reserved, must keep at value \"111111111\""]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 12) & 0x01ff) as u16)
    }
    #[doc = "Bit 21 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 22:29 - Reserved, must keep at value \"11111111\""]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 22) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Reserved, must keep at value \"11\""]
    #[inline(always)]
    pub fn reserved6(&mut self) -> Reserved6W<Scu090Spec> {
        Reserved6W::new(self, 0)
    }
    #[doc = "Bit 2 - REFCLK Stop Enable"]
    #[inline(always)]
    pub fn refclkstop_enbl(&mut self) -> RefclkstopEnblW<Scu090Spec> {
        RefclkstopEnblW::new(self, 2)
    }
    #[doc = "Bits 3:5 - Reserved, must keep at value \"111\""]
    #[inline(always)]
    pub fn reserved5(&mut self) -> Reserved5W<Scu090Spec> {
        Reserved5W::new(self, 3)
    }
    #[doc = "Bit 6 - Stop RSACLK/ECCCLK"]
    #[inline(always)]
    pub fn stop_rsaclkeccclk(&mut self) -> StopRsaclkeccclkW<Scu090Spec> {
        StopRsaclkeccclkW::new(self, 6)
    }
    #[doc = "Bit 7 - Reserved"]
    #[inline(always)]
    pub fn reserved4(&mut self) -> Reserved4W<Scu090Spec> {
        Reserved4W::new(self, 7)
    }
    #[doc = "Bit 8 - Stop I3C0CLK (for I3C0 Controller)"]
    #[inline(always)]
    pub fn stop_i3c0clkfor_i3c0ctrl(&mut self) -> StopI3c0clkforI3c0ctrlW<Scu090Spec> {
        StopI3c0clkforI3c0ctrlW::new(self, 8)
    }
    #[doc = "Bit 9 - Stop I3C1CLK (for I3C1 Controller)"]
    #[inline(always)]
    pub fn stop_i3c1clkfor_i3c1ctrl(&mut self) -> StopI3c1clkforI3c1ctrlW<Scu090Spec> {
        StopI3c1clkforI3c1ctrlW::new(self, 9)
    }
    #[doc = "Bit 10 - Stop I3C2CLK (for I3C2 Controller)"]
    #[inline(always)]
    pub fn stop_i3c2clkfor_i3c2ctrl(&mut self) -> StopI3c2clkforI3c2ctrlW<Scu090Spec> {
        StopI3c2clkforI3c2ctrlW::new(self, 10)
    }
    #[doc = "Bit 11 - Stop I3C3CLK (for I3C3 Controller)"]
    #[inline(always)]
    pub fn stop_i3c3clkfor_i3c3ctrl(&mut self) -> StopI3c3clkforI3c3ctrlW<Scu090Spec> {
        StopI3c3clkforI3c3ctrlW::new(self, 11)
    }
    #[doc = "Bits 12:20 - Reserved, must keep at value \"111111111\""]
    #[inline(always)]
    pub fn reserved3(&mut self) -> Reserved3W<Scu090Spec> {
        Reserved3W::new(self, 12)
    }
    #[doc = "Bits 22:29 - Reserved, must keep at value \"11111111\""]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<Scu090Spec> {
        Reserved1W::new(self, 22)
    }
}
#[doc = "Clock Stop Control Register Set 2\n\nYou can [`read`](crate::Reg::read) this register and get [`scu090::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu090::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu090Spec;
impl crate::RegisterSpec for Scu090Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu090::R`](R) reader structure"]
impl crate::Readable for Scu090Spec {}
#[doc = "`write(|w| ..)` method takes [`scu090::W`](W) writer structure"]
impl crate::Writable for Scu090Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU090 to value 0xfff0_fff0"]
impl crate::Resettable for Scu090Spec {
    const RESET_VALUE: u32 = 0xfff0_fff0;
}
