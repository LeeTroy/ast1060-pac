#[doc = "Register `SECURE084` reader"]
pub type R = crate::R<Secure084Spec>;
#[doc = "Register `SECURE084` writer"]
pub type W = crate::W<Secure084Spec>;
#[doc = "Field `SecBootEngIntCtrlRegsItIsCopyOfOTPCFG315` reader - 0]"]
pub type SecBootEngIntCtrlRegsItIsCopyOfOtpcfg315R = crate::FieldReader<u16>;
#[doc = "Field `SecBootEngIntCtrlRegsItIsCopyOfOTPCFG315` writer - 0]"]
pub type SecBootEngIntCtrlRegsItIsCopyOfOtpcfg315W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `WrProtOfThisRegSEC84` reader - Write Protection of this register SEC84"]
pub type WrProtOfThisRegSec84R = crate::BitReader;
#[doc = "Field `WrProtOfThisRegSEC84` writer - Write Protection of this register SEC84"]
pub type WrProtOfThisRegSec84W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved0` reader - Reserved(0)"]
pub type Reserved0R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - 0]"]
    #[inline(always)]
    pub fn sec_boot_eng_int_ctrl_regs_it_is_copy_of_otpcfg315(
        &self,
    ) -> SecBootEngIntCtrlRegsItIsCopyOfOtpcfg315R {
        SecBootEngIntCtrlRegsItIsCopyOfOtpcfg315R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - Write Protection of this register SEC84"]
    #[inline(always)]
    pub fn wr_prot_of_this_reg_sec84(&self) -> WrProtOfThisRegSec84R {
        WrProtOfThisRegSec84R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:31 - Reserved(0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new(((self.bits >> 17) & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 0]"]
    #[inline(always)]
    pub fn sec_boot_eng_int_ctrl_regs_it_is_copy_of_otpcfg315(
        &mut self,
    ) -> SecBootEngIntCtrlRegsItIsCopyOfOtpcfg315W<Secure084Spec> {
        SecBootEngIntCtrlRegsItIsCopyOfOtpcfg315W::new(self, 0)
    }
    #[doc = "Bit 16 - Write Protection of this register SEC84"]
    #[inline(always)]
    pub fn wr_prot_of_this_reg_sec84(&mut self) -> WrProtOfThisRegSec84W<Secure084Spec> {
        WrProtOfThisRegSec84W::new(self, 16)
    }
}
#[doc = "Secure Boot Engine Internal Controller Register\n\nYou can [`read`](crate::Reg::read) this register and get [`secure084::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure084::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Secure084Spec;
impl crate::RegisterSpec for Secure084Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`secure084::R`](R) reader structure"]
impl crate::Readable for Secure084Spec {}
#[doc = "`write(|w| ..)` method takes [`secure084::W`](W) writer structure"]
impl crate::Writable for Secure084Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SECURE084 to value 0"]
impl crate::Resettable for Secure084Spec {}
