#[doc = "Register `SECURE088` reader"]
pub type R = crate::R<Secure088Spec>;
#[doc = "Register `SECURE088` writer"]
pub type W = crate::W<Secure088Spec>;
#[doc = "Software ECC Control for User Region Register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwEccctrlForUserRegionReg {
    #[doc = "0: normal mode"]
    NormalMode = 0,
    #[doc = "1: disable ECC for user region"]
    DisableEccForUserRegion = 1,
}
impl From<SwEccctrlForUserRegionReg> for bool {
    #[inline(always)]
    fn from(variant: SwEccctrlForUserRegionReg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SwECCCtrlForUserRegionReg` reader - Software ECC Control for User Region Register"]
pub type SwEccctrlForUserRegionRegR = crate::BitReader<SwEccctrlForUserRegionReg>;
impl SwEccctrlForUserRegionRegR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SwEccctrlForUserRegionReg {
        match self.bits {
            false => SwEccctrlForUserRegionReg::NormalMode,
            true => SwEccctrlForUserRegionReg::DisableEccForUserRegion,
        }
    }
    #[doc = "normal mode"]
    #[inline(always)]
    pub fn is_normal_mode(&self) -> bool {
        *self == SwEccctrlForUserRegionReg::NormalMode
    }
    #[doc = "disable ECC for user region"]
    #[inline(always)]
    pub fn is_disable_ecc_for_user_region(&self) -> bool {
        *self == SwEccctrlForUserRegionReg::DisableEccForUserRegion
    }
}
#[doc = "Field `SwECCCtrlForUserRegionReg` writer - Software ECC Control for User Region Register"]
pub type SwEccctrlForUserRegionRegW<'a, REG> = crate::BitWriter<'a, REG, SwEccctrlForUserRegionReg>;
impl<'a, REG> SwEccctrlForUserRegionRegW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "normal mode"]
    #[inline(always)]
    pub fn normal_mode(self) -> &'a mut crate::W<REG> {
        self.variant(SwEccctrlForUserRegionReg::NormalMode)
    }
    #[doc = "disable ECC for user region"]
    #[inline(always)]
    pub fn disable_ecc_for_user_region(self) -> &'a mut crate::W<REG> {
        self.variant(SwEccctrlForUserRegionReg::DisableEccForUserRegion)
    }
}
#[doc = "Software ECC Control for Secure Region Register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwEccctrlForSecRegionReg {
    #[doc = "0: normal mode"]
    NormalMode = 0,
    #[doc = "1: disable ECC for secure region"]
    DisableEccForSecureRegion = 1,
}
impl From<SwEccctrlForSecRegionReg> for bool {
    #[inline(always)]
    fn from(variant: SwEccctrlForSecRegionReg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SwECCCtrlForSecRegionReg` reader - Software ECC Control for Secure Region Register"]
pub type SwEccctrlForSecRegionRegR = crate::BitReader<SwEccctrlForSecRegionReg>;
impl SwEccctrlForSecRegionRegR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SwEccctrlForSecRegionReg {
        match self.bits {
            false => SwEccctrlForSecRegionReg::NormalMode,
            true => SwEccctrlForSecRegionReg::DisableEccForSecureRegion,
        }
    }
    #[doc = "normal mode"]
    #[inline(always)]
    pub fn is_normal_mode(&self) -> bool {
        *self == SwEccctrlForSecRegionReg::NormalMode
    }
    #[doc = "disable ECC for secure region"]
    #[inline(always)]
    pub fn is_disable_ecc_for_secure_region(&self) -> bool {
        *self == SwEccctrlForSecRegionReg::DisableEccForSecureRegion
    }
}
#[doc = "Field `SwECCCtrlForSecRegionReg` writer - Software ECC Control for Secure Region Register"]
pub type SwEccctrlForSecRegionRegW<'a, REG> = crate::BitWriter<'a, REG, SwEccctrlForSecRegionReg>;
impl<'a, REG> SwEccctrlForSecRegionRegW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "normal mode"]
    #[inline(always)]
    pub fn normal_mode(self) -> &'a mut crate::W<REG> {
        self.variant(SwEccctrlForSecRegionReg::NormalMode)
    }
    #[doc = "disable ECC for secure region"]
    #[inline(always)]
    pub fn disable_ecc_for_secure_region(self) -> &'a mut crate::W<REG> {
        self.variant(SwEccctrlForSecRegionReg::DisableEccForSecureRegion)
    }
}
#[doc = "Field `WrProtOfThisRegSEC88` reader - Write Protection of this register SEC88"]
pub type WrProtOfThisRegSec88R = crate::BitReader;
#[doc = "Field `WrProtOfThisRegSEC88` writer - Write Protection of this register SEC88"]
pub type WrProtOfThisRegSec88W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved0` reader - Reserved(0)"]
pub type Reserved0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - Software ECC Control for User Region Register"]
    #[inline(always)]
    pub fn sw_eccctrl_for_user_region_reg(&self) -> SwEccctrlForUserRegionRegR {
        SwEccctrlForUserRegionRegR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Software ECC Control for Secure Region Register"]
    #[inline(always)]
    pub fn sw_eccctrl_for_sec_region_reg(&self) -> SwEccctrlForSecRegionRegR {
        SwEccctrlForSecRegionRegR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Write Protection of this register SEC88"]
    #[inline(always)]
    pub fn wr_prot_of_this_reg_sec88(&self) -> WrProtOfThisRegSec88R {
        WrProtOfThisRegSec88R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:31 - Reserved(0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits >> 3) & 0x1fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - Software ECC Control for User Region Register"]
    #[inline(always)]
    pub fn sw_eccctrl_for_user_region_reg(&mut self) -> SwEccctrlForUserRegionRegW<Secure088Spec> {
        SwEccctrlForUserRegionRegW::new(self, 0)
    }
    #[doc = "Bit 1 - Software ECC Control for Secure Region Register"]
    #[inline(always)]
    pub fn sw_eccctrl_for_sec_region_reg(&mut self) -> SwEccctrlForSecRegionRegW<Secure088Spec> {
        SwEccctrlForSecRegionRegW::new(self, 1)
    }
    #[doc = "Bit 2 - Write Protection of this register SEC88"]
    #[inline(always)]
    pub fn wr_prot_of_this_reg_sec88(&mut self) -> WrProtOfThisRegSec88W<Secure088Spec> {
        WrProtOfThisRegSec88W::new(self, 2)
    }
}
#[doc = "Software ECC Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`secure088::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure088::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Secure088Spec;
impl crate::RegisterSpec for Secure088Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`secure088::R`](R) reader structure"]
impl crate::Readable for Secure088Spec {}
#[doc = "`write(|w| ..)` method takes [`secure088::W`](W) writer structure"]
impl crate::Writable for Secure088Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SECURE088 to value 0"]
impl crate::Resettable for Secure088Spec {}
