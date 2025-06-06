#[doc = "Register `SECURE080` reader"]
pub type R = crate::R<Secure080Spec>;
#[doc = "Register `SECURE080` writer"]
pub type W = crate::W<Secure080Spec>;
#[doc = "Field `SecBootEngIntCtrlRegs` reader - Secure Boot Engine Internal Controller Registers"]
pub type SecBootEngIntCtrlRegsR = crate::FieldReader;
#[doc = "Field `SecBootEngIntCtrlRegs` writer - Secure Boot Engine Internal Controller Registers"]
pub type SecBootEngIntCtrlRegsW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `Reserved0` reader - Reserved(0)"]
pub type Reserved0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:3 - Secure Boot Engine Internal Controller Registers"]
    #[inline(always)]
    pub fn sec_boot_eng_int_ctrl_regs(&self) -> SecBootEngIntCtrlRegsR {
        SecBootEngIntCtrlRegsR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:31 - Reserved(0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:3 - Secure Boot Engine Internal Controller Registers"]
    #[inline(always)]
    pub fn sec_boot_eng_int_ctrl_regs(&mut self) -> SecBootEngIntCtrlRegsW<Secure080Spec> {
        SecBootEngIntCtrlRegsW::new(self, 0)
    }
}
#[doc = "Secure Boot Engine Internal Controller Register\n\nYou can [`read`](crate::Reg::read) this register and get [`secure080::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure080::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Secure080Spec;
impl crate::RegisterSpec for Secure080Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`secure080::R`](R) reader structure"]
impl crate::Readable for Secure080Spec {}
#[doc = "`write(|w| ..)` method takes [`secure080::W`](W) writer structure"]
impl crate::Writable for Secure080Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SECURE080 to value 0"]
impl crate::Resettable for Secure080Spec {}
