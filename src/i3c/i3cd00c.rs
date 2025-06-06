#[doc = "Register `I3CD00C` reader"]
pub type R = crate::R<I3cd00cSpec>;
#[doc = "Register `I3CD00C` writer"]
pub type W = crate::W<I3cd00cSpec>;
#[doc = "Field `32BitCmd` reader - 32 bit command"]
pub type _32bitCmdR = crate::FieldReader<u32>;
#[doc = "Field `32BitCmd` writer - 32 bit command"]
pub type _32bitCmdW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 32 bit command"]
    #[inline(always)]
    pub fn _32bit_cmd(&self) -> _32bitCmdR {
        _32bitCmdR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 32 bit command"]
    #[inline(always)]
    pub fn _32bit_cmd(&mut self) -> _32bitCmdW<I3cd00cSpec> {
        _32bitCmdW::new(self, 0)
    }
}
#[doc = "COMMAND\\_QUEUE\\_PORT\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cd00c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cd00c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3cd00cSpec;
impl crate::RegisterSpec for I3cd00cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3cd00c::R`](R) reader structure"]
impl crate::Readable for I3cd00cSpec {}
#[doc = "`write(|w| ..)` method takes [`i3cd00c::W`](W) writer structure"]
impl crate::Writable for I3cd00cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CD00C to value 0"]
impl crate::Resettable for I3cd00cSpec {}
