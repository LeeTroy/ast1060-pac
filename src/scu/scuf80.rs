#[doc = "Register `SCUF80` reader"]
pub type R = crate::R<Scuf80Spec>;
#[doc = "Register `SCUF80` writer"]
pub type W = crate::W<Scuf80Spec>;
#[doc = "Field `Reserved5` reader - Reserved(0)"]
pub type Reserved5R = crate::FieldReader;
#[doc = "Field `EnblARMRSTNAsRstSrcOfHlinkSCU040` reader - Enable hlinkARMRSTN as reset source of hlinkSCU040"]
pub type EnblArmrstnasRstSrcOfHlinkScu040R = crate::BitReader;
#[doc = "Field `EnblARMRSTNAsRstSrcOfHlinkSCU040` writer - Enable hlinkARMRSTN as reset source of hlinkSCU040"]
pub type EnblArmrstnasRstSrcOfHlinkScu040W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - Reserved(0)"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `Reserved4` reader - Reserved(0)"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `EnblARMRSTNAsRstSrcOfHlinkSCU048` reader - Enable hlinkARMRSTN as reset source of hlinkSCU048"]
pub type EnblArmrstnasRstSrcOfHlinkScu048R = crate::BitReader;
#[doc = "Field `EnblARMRSTNAsRstSrcOfHlinkSCU048` writer - Enable hlinkARMRSTN as reset source of hlinkSCU048"]
pub type EnblArmrstnasRstSrcOfHlinkScu048W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblARMRSTNAsRstSrcOfHlinkSCU060` reader - Enable hlinkARMRSTN as reset source of hlinkSCU060"]
pub type EnblArmrstnasRstSrcOfHlinkScu060R = crate::BitReader;
#[doc = "Field `EnblARMRSTNAsRstSrcOfHlinkSCU060` writer - Enable hlinkARMRSTN as reset source of hlinkSCU060"]
pub type EnblArmrstnasRstSrcOfHlinkScu060W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - Reserved(0)"]
pub type Reserved2R = crate::FieldReader;
#[doc = "Field `EnblARMRSTNAsRstSrcOfHlinkSCU080` reader - Enable hlinkARMRSTN as reset source of hlinkSCU080"]
pub type EnblArmrstnasRstSrcOfHlinkScu080R = crate::BitReader;
#[doc = "Field `EnblARMRSTNAsRstSrcOfHlinkSCU080` writer - Enable hlinkARMRSTN as reset source of hlinkSCU080"]
pub type EnblArmrstnasRstSrcOfHlinkScu080W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - Reserved(0)"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `EnblARMRSTNAsRstSrcOfHlinkSCU088` reader - Enable hlinkARMRSTN as reset source of hlinkSCU088"]
pub type EnblArmrstnasRstSrcOfHlinkScu088R = crate::BitReader;
#[doc = "Field `EnblARMRSTNAsRstSrcOfHlinkSCU088` writer - Enable hlinkARMRSTN as reset source of hlinkSCU088"]
pub type EnblArmrstnasRstSrcOfHlinkScu088W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Reserved(0)"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - Enable hlinkARMRSTN as reset source of hlinkSCU040"]
    #[inline(always)]
    pub fn enbl_armrstnas_rst_src_of_hlink_scu040(&self) -> EnblArmrstnasRstSrcOfHlinkScu040R {
        EnblArmrstnasRstSrcOfHlinkScu040R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:11 - Reserved(0)"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bit 9 - Reserved(0)"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable hlinkARMRSTN as reset source of hlinkSCU048"]
    #[inline(always)]
    pub fn enbl_armrstnas_rst_src_of_hlink_scu048(&self) -> EnblArmrstnasRstSrcOfHlinkScu048R {
        EnblArmrstnasRstSrcOfHlinkScu048R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable hlinkARMRSTN as reset source of hlinkSCU060"]
    #[inline(always)]
    pub fn enbl_armrstnas_rst_src_of_hlink_scu060(&self) -> EnblArmrstnasRstSrcOfHlinkScu060R {
        EnblArmrstnasRstSrcOfHlinkScu060R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:15 - Reserved(0)"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bit 16 - Enable hlinkARMRSTN as reset source of hlinkSCU080"]
    #[inline(always)]
    pub fn enbl_armrstnas_rst_src_of_hlink_scu080(&self) -> EnblArmrstnasRstSrcOfHlinkScu080R {
        EnblArmrstnasRstSrcOfHlinkScu080R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Reserved(0)"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable hlinkARMRSTN as reset source of hlinkSCU088"]
    #[inline(always)]
    pub fn enbl_armrstnas_rst_src_of_hlink_scu088(&self) -> EnblArmrstnasRstSrcOfHlinkScu088R {
        EnblArmrstnasRstSrcOfHlinkScu088R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - Enable hlinkARMRSTN as reset source of hlinkSCU040"]
    #[inline(always)]
    pub fn enbl_armrstnas_rst_src_of_hlink_scu040(
        &mut self,
    ) -> EnblArmrstnasRstSrcOfHlinkScu040W<Scuf80Spec> {
        EnblArmrstnasRstSrcOfHlinkScu040W::new(self, 8)
    }
    #[doc = "Bit 10 - Enable hlinkARMRSTN as reset source of hlinkSCU048"]
    #[inline(always)]
    pub fn enbl_armrstnas_rst_src_of_hlink_scu048(
        &mut self,
    ) -> EnblArmrstnasRstSrcOfHlinkScu048W<Scuf80Spec> {
        EnblArmrstnasRstSrcOfHlinkScu048W::new(self, 10)
    }
    #[doc = "Bit 12 - Enable hlinkARMRSTN as reset source of hlinkSCU060"]
    #[inline(always)]
    pub fn enbl_armrstnas_rst_src_of_hlink_scu060(
        &mut self,
    ) -> EnblArmrstnasRstSrcOfHlinkScu060W<Scuf80Spec> {
        EnblArmrstnasRstSrcOfHlinkScu060W::new(self, 12)
    }
    #[doc = "Bit 16 - Enable hlinkARMRSTN as reset source of hlinkSCU080"]
    #[inline(always)]
    pub fn enbl_armrstnas_rst_src_of_hlink_scu080(
        &mut self,
    ) -> EnblArmrstnasRstSrcOfHlinkScu080W<Scuf80Spec> {
        EnblArmrstnasRstSrcOfHlinkScu080W::new(self, 16)
    }
    #[doc = "Bit 18 - Enable hlinkARMRSTN as reset source of hlinkSCU088"]
    #[inline(always)]
    pub fn enbl_armrstnas_rst_src_of_hlink_scu088(
        &mut self,
    ) -> EnblArmrstnasRstSrcOfHlinkScu088W<Scuf80Spec> {
        EnblArmrstnasRstSrcOfHlinkScu088W::new(self, 18)
    }
}
#[doc = "Reset Source Control Register \\#1\n\nYou can [`read`](crate::Reg::read) this register and get [`scuf80::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scuf80::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scuf80Spec;
impl crate::RegisterSpec for Scuf80Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scuf80::R`](R) reader structure"]
impl crate::Readable for Scuf80Spec {}
#[doc = "`write(|w| ..)` method takes [`scuf80::W`](W) writer structure"]
impl crate::Writable for Scuf80Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCUF80 to value 0"]
impl crate::Resettable for Scuf80Spec {}
