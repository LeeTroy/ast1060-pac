#[doc = "Register `SCU650` reader"]
pub type R = crate::R<Scu650Spec>;
#[doc = "Register `SCU650` writer"]
pub type W = crate::W<Scu650Spec>;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::FieldReader<u16>;
#[doc = "Field `Reserved1` writer - Reserved"]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `HVI3C1DrivingStrength` reader - HVI3C1 Driving Strength"]
pub type Hvi3c1drivingStrengthR = crate::BitReader;
#[doc = "Field `HVI3C1DrivingStrength` writer - HVI3C1 Driving Strength"]
pub type Hvi3c1drivingStrengthW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HVI3C2DrivingStrength` reader - HVI3C2 Driving Strength"]
pub type Hvi3c2drivingStrengthR = crate::BitReader;
#[doc = "Field `HVI3C2DrivingStrength` writer - HVI3C2 Driving Strength"]
pub type Hvi3c2drivingStrengthW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HVI3C3DrivingStrength` reader - HVI3C3 Driving Strength"]
pub type Hvi3c3drivingStrengthR = crate::BitReader;
#[doc = "Field `HVI3C3DrivingStrength` writer - HVI3C3 Driving Strength"]
pub type Hvi3c3drivingStrengthW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HVI3C4DrivingStrength` reader - HVI3C4 Driving Strength"]
pub type Hvi3c4drivingStrengthR = crate::BitReader;
#[doc = "Field `HVI3C4DrivingStrength` writer - HVI3C4 Driving Strength"]
pub type Hvi3c4drivingStrengthW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - HVI3C1 Driving Strength"]
    #[inline(always)]
    pub fn hvi3c1driving_strength(&self) -> Hvi3c1drivingStrengthR {
        Hvi3c1drivingStrengthR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - HVI3C2 Driving Strength"]
    #[inline(always)]
    pub fn hvi3c2driving_strength(&self) -> Hvi3c2drivingStrengthR {
        Hvi3c2drivingStrengthR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - HVI3C3 Driving Strength"]
    #[inline(always)]
    pub fn hvi3c3driving_strength(&self) -> Hvi3c3drivingStrengthR {
        Hvi3c3drivingStrengthR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - HVI3C4 Driving Strength"]
    #[inline(always)]
    pub fn hvi3c4driving_strength(&self) -> Hvi3c4drivingStrengthR {
        Hvi3c4drivingStrengthR::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<Scu650Spec> {
        Reserved1W::new(self, 0)
    }
    #[doc = "Bit 16 - HVI3C1 Driving Strength"]
    #[inline(always)]
    pub fn hvi3c1driving_strength(&mut self) -> Hvi3c1drivingStrengthW<Scu650Spec> {
        Hvi3c1drivingStrengthW::new(self, 16)
    }
    #[doc = "Bit 17 - HVI3C2 Driving Strength"]
    #[inline(always)]
    pub fn hvi3c2driving_strength(&mut self) -> Hvi3c2drivingStrengthW<Scu650Spec> {
        Hvi3c2drivingStrengthW::new(self, 17)
    }
    #[doc = "Bit 18 - HVI3C3 Driving Strength"]
    #[inline(always)]
    pub fn hvi3c3driving_strength(&mut self) -> Hvi3c3drivingStrengthW<Scu650Spec> {
        Hvi3c3drivingStrengthW::new(self, 18)
    }
    #[doc = "Bit 19 - HVI3C4 Driving Strength"]
    #[inline(always)]
    pub fn hvi3c4driving_strength(&mut self) -> Hvi3c4drivingStrengthW<Scu650Spec> {
        Hvi3c4drivingStrengthW::new(self, 19)
    }
}
#[doc = "IO Driving Strength\n\nYou can [`read`](crate::Reg::read) this register and get [`scu650::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu650::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu650Spec;
impl crate::RegisterSpec for Scu650Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu650::R`](R) reader structure"]
impl crate::Readable for Scu650Spec {}
#[doc = "`write(|w| ..)` method takes [`scu650::W`](W) writer structure"]
impl crate::Writable for Scu650Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU650 to value 0x11"]
impl crate::Resettable for Scu650Spec {
    const RESET_VALUE: u32 = 0x11;
}
