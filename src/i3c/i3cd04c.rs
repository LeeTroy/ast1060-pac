#[doc = "Register `I3CD04C` reader"]
pub type R = crate::R<I3cd04cSpec>;
#[doc = "Register `I3CD04C` writer"]
pub type W = crate::W<I3cd04cSpec>;
#[doc = "Field `CMDQUEUEEMPTYLOC` reader - CMD_QUEUE_EMPTY_LOC"]
pub type CmdqueueemptylocR = crate::FieldReader;
#[doc = "Field `RESPBUFBLR` reader - RESP_BUF_BLR"]
pub type RespbufblrR = crate::FieldReader;
#[doc = "Field `IBIBUFBLR` reader - IBI_BUF_BLR"]
pub type IbibufblrR = crate::FieldReader;
#[doc = "Field `RSVD` reader - reserved"]
pub type RsvdR = crate::FieldReader<u16>;
#[doc = "Field `IBISTATUSCNT` reader - IBI_STATUS_CNT"]
pub type IbistatuscntR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - CMD_QUEUE_EMPTY_LOC"]
    #[inline(always)]
    pub fn cmdqueueemptyloc(&self) -> CmdqueueemptylocR {
        CmdqueueemptylocR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - RESP_BUF_BLR"]
    #[inline(always)]
    pub fn respbufblr(&self) -> RespbufblrR {
        RespbufblrR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - IBI_BUF_BLR"]
    #[inline(always)]
    pub fn ibibufblr(&self) -> IbibufblrR {
        IbibufblrR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 19:31 - reserved"]
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 19) & 0x1fff) as u16)
    }
    #[doc = "Bits 24:28 - IBI_STATUS_CNT"]
    #[inline(always)]
    pub fn ibistatuscnt(&self) -> IbistatuscntR {
        IbistatuscntR::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {}
#[doc = "QUEUE STATUS LEVEL Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cd04c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cd04c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3cd04cSpec;
impl crate::RegisterSpec for I3cd04cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3cd04c::R`](R) reader structure"]
impl crate::Readable for I3cd04cSpec {}
#[doc = "`write(|w| ..)` method takes [`i3cd04c::W`](W) writer structure"]
impl crate::Writable for I3cd04cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CD04C to value 0"]
impl crate::Resettable for I3cd04cSpec {}
