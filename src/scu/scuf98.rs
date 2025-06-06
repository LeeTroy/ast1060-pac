#[doc = "Register `SCUF98` reader"]
pub type R = crate::R<Scuf98Spec>;
#[doc = "Register `SCUF98` writer"]
pub type W = crate::W<Scuf98Spec>;
#[doc = "Field `EnblARMRSTNAsRstSrcOfHlinkSCU210` reader - Enable hlinkARMRSTN as reset source of hlinkSCU210"]
pub type EnblArmrstnasRstSrcOfHlinkScu210R = crate::BitReader;
#[doc = "Field `EnblARMRSTNAsRstSrcOfHlinkSCU210` writer - Enable hlinkARMRSTN as reset source of hlinkSCU210"]
pub type EnblArmrstnasRstSrcOfHlinkScu210W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblARMRSTNAsRstSrcOfHlinkSCU214` reader - Enable hlinkARMRSTN as reset source of hlinkSCU214"]
pub type EnblArmrstnasRstSrcOfHlinkScu214R = crate::BitReader;
#[doc = "Field `EnblARMRSTNAsRstSrcOfHlinkSCU214` writer - Enable hlinkARMRSTN as reset source of hlinkSCU214"]
pub type EnblArmrstnasRstSrcOfHlinkScu214W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable hlinkARMRSTN as reset source of hlinkSCU210"]
    #[inline(always)]
    pub fn enbl_armrstnas_rst_src_of_hlink_scu210(&self) -> EnblArmrstnasRstSrcOfHlinkScu210R {
        EnblArmrstnasRstSrcOfHlinkScu210R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable hlinkARMRSTN as reset source of hlinkSCU214"]
    #[inline(always)]
    pub fn enbl_armrstnas_rst_src_of_hlink_scu214(&self) -> EnblArmrstnasRstSrcOfHlinkScu214R {
        EnblArmrstnasRstSrcOfHlinkScu214R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable hlinkARMRSTN as reset source of hlinkSCU210"]
    #[inline(always)]
    pub fn enbl_armrstnas_rst_src_of_hlink_scu210(
        &mut self,
    ) -> EnblArmrstnasRstSrcOfHlinkScu210W<Scuf98Spec> {
        EnblArmrstnasRstSrcOfHlinkScu210W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable hlinkARMRSTN as reset source of hlinkSCU214"]
    #[inline(always)]
    pub fn enbl_armrstnas_rst_src_of_hlink_scu214(
        &mut self,
    ) -> EnblArmrstnasRstSrcOfHlinkScu214W<Scuf98Spec> {
        EnblArmrstnasRstSrcOfHlinkScu214W::new(self, 1)
    }
}
#[doc = "Reset Source Control Register \\#7\n\nYou can [`read`](crate::Reg::read) this register and get [`scuf98::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scuf98::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scuf98Spec;
impl crate::RegisterSpec for Scuf98Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scuf98::R`](R) reader structure"]
impl crate::Readable for Scuf98Spec {}
#[doc = "`write(|w| ..)` method takes [`scuf98::W`](W) writer structure"]
impl crate::Writable for Scuf98Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCUF98 to value 0"]
impl crate::Resettable for Scuf98Spec {}
