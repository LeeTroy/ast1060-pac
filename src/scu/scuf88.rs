#[doc = "Register `SCUF88` reader"]
pub type R = crate::R<Scuf88Spec>;
#[doc = "Register `SCUF88` writer"]
pub type W = crate::W<Scuf88Spec>;
#[doc = "Field `EnblARMRSTNAsRstSrcOfHlinkSCU200` reader - Enable hlinkARMRSTN as reset source of hlinkSCU200"]
pub type EnblArmrstnasRstSrcOfHlinkScu200R = crate::BitReader;
#[doc = "Field `EnblARMRSTNAsRstSrcOfHlinkSCU200` writer - Enable hlinkARMRSTN as reset source of hlinkSCU200"]
pub type EnblArmrstnasRstSrcOfHlinkScu200W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblARMRSTNAsRstSrcOfHlinkSCU204` reader - Enable hlinkARMRSTN as reset source of hlinkSCU204"]
pub type EnblArmrstnasRstSrcOfHlinkScu204R = crate::BitReader;
#[doc = "Field `EnblARMRSTNAsRstSrcOfHlinkSCU204` writer - Enable hlinkARMRSTN as reset source of hlinkSCU204"]
pub type EnblArmrstnasRstSrcOfHlinkScu204W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable hlinkARMRSTN as reset source of hlinkSCU200"]
    #[inline(always)]
    pub fn enbl_armrstnas_rst_src_of_hlink_scu200(&self) -> EnblArmrstnasRstSrcOfHlinkScu200R {
        EnblArmrstnasRstSrcOfHlinkScu200R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable hlinkARMRSTN as reset source of hlinkSCU204"]
    #[inline(always)]
    pub fn enbl_armrstnas_rst_src_of_hlink_scu204(&self) -> EnblArmrstnasRstSrcOfHlinkScu204R {
        EnblArmrstnasRstSrcOfHlinkScu204R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable hlinkARMRSTN as reset source of hlinkSCU200"]
    #[inline(always)]
    pub fn enbl_armrstnas_rst_src_of_hlink_scu200(
        &mut self,
    ) -> EnblArmrstnasRstSrcOfHlinkScu200W<Scuf88Spec> {
        EnblArmrstnasRstSrcOfHlinkScu200W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable hlinkARMRSTN as reset source of hlinkSCU204"]
    #[inline(always)]
    pub fn enbl_armrstnas_rst_src_of_hlink_scu204(
        &mut self,
    ) -> EnblArmrstnasRstSrcOfHlinkScu204W<Scuf88Spec> {
        EnblArmrstnasRstSrcOfHlinkScu204W::new(self, 1)
    }
}
#[doc = "Reset Source Control Register \\#3\n\nYou can [`read`](crate::Reg::read) this register and get [`scuf88::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scuf88::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scuf88Spec;
impl crate::RegisterSpec for Scuf88Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scuf88::R`](R) reader structure"]
impl crate::Readable for Scuf88Spec {}
#[doc = "`write(|w| ..)` method takes [`scuf88::W`](W) writer structure"]
impl crate::Writable for Scuf88Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCUF88 to value 0"]
impl crate::Resettable for Scuf88Spec {}
