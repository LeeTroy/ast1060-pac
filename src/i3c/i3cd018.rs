#[doc = "Register `I3CD018` reader"]
pub type R = crate::R<I3cd018Spec>;
#[doc = "Register `I3CD018` writer"]
pub type W = crate::W<I3cd018Spec>;
#[doc = "Field `InBandINTData` reader - In-Band Interrupt Data"]
pub type InBandIntdataR = crate::FieldReader<u32>;
#[doc = "Field `InBandINTDataLen` reader - In-Band Interrupt data length"]
pub type InBandIntdataLenR = crate::FieldReader;
#[doc = "Field `IBIIdentifier` reader - IBI Identifier"]
pub type IbiidentifierR = crate::FieldReader;
#[doc = "Field `RSVD1` reader - reserved"]
pub type Rsvd1R = crate::FieldReader;
#[doc = "Field `LASTSTATUS` reader - LAST_STATUS"]
pub type LaststatusR = crate::BitReader;
#[doc = "Field `RSVD` reader - reserved"]
pub type RsvdR = crate::FieldReader;
#[doc = "Field `ERROR` reader - ERROR"]
pub type ErrorR = crate::BitReader;
#[doc = "Field `IBIStatus` reader - IBI Status"]
pub type IbistatusR = crate::BitReader;
impl R {
    #[doc = "Bits 0:31 - In-Band Interrupt Data"]
    #[inline(always)]
    pub fn in_band_intdata(&self) -> InBandIntdataR {
        InBandIntdataR::new(self.bits)
    }
    #[doc = "Bits 0:7 - In-Band Interrupt data length"]
    #[inline(always)]
    pub fn in_band_intdata_len(&self) -> InBandIntdataLenR {
        InBandIntdataLenR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - IBI Identifier"]
    #[inline(always)]
    pub fn ibiidentifier(&self) -> IbiidentifierR {
        IbiidentifierR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - reserved"]
    #[inline(always)]
    pub fn rsvd1(&self) -> Rsvd1R {
        Rsvd1R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - LAST_STATUS"]
    #[inline(always)]
    pub fn laststatus(&self) -> LaststatusR {
        LaststatusR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:29 - reserved"]
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 25) & 0x1f) as u8)
    }
    #[doc = "Bit 30 - ERROR"]
    #[inline(always)]
    pub fn error(&self) -> ErrorR {
        ErrorR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - IBI Status"]
    #[inline(always)]
    pub fn ibistatus(&self) -> IbistatusR {
        IbistatusR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {}
#[doc = "In-Band Interrupt Queue Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cd018::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cd018::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3cd018Spec;
impl crate::RegisterSpec for I3cd018Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3cd018::R`](R) reader structure"]
impl crate::Readable for I3cd018Spec {}
#[doc = "`write(|w| ..)` method takes [`i3cd018::W`](W) writer structure"]
impl crate::Writable for I3cd018Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CD018 to value 0"]
impl crate::Resettable for I3cd018Spec {}
