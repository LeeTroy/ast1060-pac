#[doc = "Register `SPI150` reader"]
pub type R = crate::R<Spi150Spec>;
#[doc = "Register `SPI150` writer"]
pub type W = crate::W<Spi150Spec>;
#[doc = "Field `Cmd0` reader - Command 0"]
pub type Cmd0R = crate::FieldReader;
#[doc = "Field `Cmd0` writer - Command 0"]
pub type Cmd0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `Cmd1` reader - Command 1"]
pub type Cmd1R = crate::FieldReader;
#[doc = "Field `Cmd1` writer - Command 1"]
pub type Cmd1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `Cmd2` reader - Command 2"]
pub type Cmd2R = crate::FieldReader;
#[doc = "Field `Cmd2` writer - Command 2"]
pub type Cmd2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "3B/4B Command\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum _3b4bcmd {
    #[doc = "0: Command \\#2/\\#1/\\#0 are for 3B mode."]
    Command210AreFor3bMode = 0,
    #[doc = "1: Command \\#2/\\#1/\\#0 are for 4B mode."]
    Command210AreFor4bMode = 1,
}
impl From<_3b4bcmd> for bool {
    #[inline(always)]
    fn from(variant: _3b4bcmd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `3B4BCmd` reader - 3B/4B Command"]
pub type _3b4bcmdR = crate::BitReader<_3b4bcmd>;
impl _3b4bcmdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> _3b4bcmd {
        match self.bits {
            false => _3b4bcmd::Command210AreFor3bMode,
            true => _3b4bcmd::Command210AreFor4bMode,
        }
    }
    #[doc = "Command \\#2/\\#1/\\#0 are for 3B mode."]
    #[inline(always)]
    pub fn is_command_210_are_for_3b_mode(&self) -> bool {
        *self == _3b4bcmd::Command210AreFor3bMode
    }
    #[doc = "Command \\#2/\\#1/\\#0 are for 4B mode."]
    #[inline(always)]
    pub fn is_command_210_are_for_4b_mode(&self) -> bool {
        *self == _3b4bcmd::Command210AreFor4bMode
    }
}
#[doc = "Field `3B4BCmd` writer - 3B/4B Command"]
pub type _3b4bcmdW<'a, REG> = crate::BitWriter<'a, REG, _3b4bcmd>;
impl<'a, REG> _3b4bcmdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Command \\#2/\\#1/\\#0 are for 3B mode."]
    #[inline(always)]
    pub fn command_210_are_for_3b_mode(self) -> &'a mut crate::W<REG> {
        self.variant(_3b4bcmd::Command210AreFor3bMode)
    }
    #[doc = "Command \\#2/\\#1/\\#0 are for 4B mode."]
    #[inline(always)]
    pub fn command_210_are_for_4b_mode(self) -> &'a mut crate::W<REG> {
        self.variant(_3b4bcmd::Command210AreFor4bMode)
    }
}
#[doc = "Field `EnblCmd0ForRead` reader - Enable Command 0 for read"]
pub type EnblCmd0forReadR = crate::BitReader;
#[doc = "Field `EnblCmd0ForRead` writer - Enable Command 0 for read"]
pub type EnblCmd0forReadW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblCmd0ForWr` reader - Enable Command 0 for write"]
pub type EnblCmd0forWrR = crate::BitReader;
#[doc = "Field `EnblCmd0ForWr` writer - Enable Command 0 for write"]
pub type EnblCmd0forWrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblCmd1ForRead` reader - Enable Command 1 for read"]
pub type EnblCmd1forReadR = crate::BitReader;
#[doc = "Field `EnblCmd1ForRead` writer - Enable Command 1 for read"]
pub type EnblCmd1forReadW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblCmd1ForWr` reader - Enable Command 1 for write"]
pub type EnblCmd1forWrR = crate::BitReader;
#[doc = "Field `EnblCmd1ForWr` writer - Enable Command 1 for write"]
pub type EnblCmd1forWrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblCmd2ForRead` reader - Enable Command 2 for read"]
pub type EnblCmd2forReadR = crate::BitReader;
#[doc = "Field `EnblCmd2ForRead` writer - Enable Command 2 for read"]
pub type EnblCmd2forReadW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblCmd2ForWr` reader - Enable Command 2 for write"]
pub type EnblCmd2forWrR = crate::BitReader;
#[doc = "Field `EnblCmd2ForWr` writer - Enable Command 2 for write"]
pub type EnblCmd2forWrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Command 0"]
    #[inline(always)]
    pub fn cmd0(&self) -> Cmd0R {
        Cmd0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Command 1"]
    #[inline(always)]
    pub fn cmd1(&self) -> Cmd1R {
        Cmd1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Command 2"]
    #[inline(always)]
    pub fn cmd2(&self) -> Cmd2R {
        Cmd2R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - 3B/4B Command"]
    #[inline(always)]
    pub fn _3b4bcmd(&self) -> _3b4bcmdR {
        _3b4bcmdR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable Command 0 for read"]
    #[inline(always)]
    pub fn enbl_cmd0for_read(&self) -> EnblCmd0forReadR {
        EnblCmd0forReadR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enable Command 0 for write"]
    #[inline(always)]
    pub fn enbl_cmd0for_wr(&self) -> EnblCmd0forWrR {
        EnblCmd0forWrR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Enable Command 1 for read"]
    #[inline(always)]
    pub fn enbl_cmd1for_read(&self) -> EnblCmd1forReadR {
        EnblCmd1forReadR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Enable Command 1 for write"]
    #[inline(always)]
    pub fn enbl_cmd1for_wr(&self) -> EnblCmd1forWrR {
        EnblCmd1forWrR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Enable Command 2 for read"]
    #[inline(always)]
    pub fn enbl_cmd2for_read(&self) -> EnblCmd2forReadR {
        EnblCmd2forReadR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable Command 2 for write"]
    #[inline(always)]
    pub fn enbl_cmd2for_wr(&self) -> EnblCmd2forWrR {
        EnblCmd2forWrR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Command 0"]
    #[inline(always)]
    pub fn cmd0(&mut self) -> Cmd0W<Spi150Spec> {
        Cmd0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Command 1"]
    #[inline(always)]
    pub fn cmd1(&mut self) -> Cmd1W<Spi150Spec> {
        Cmd1W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Command 2"]
    #[inline(always)]
    pub fn cmd2(&mut self) -> Cmd2W<Spi150Spec> {
        Cmd2W::new(self, 16)
    }
    #[doc = "Bit 24 - 3B/4B Command"]
    #[inline(always)]
    pub fn _3b4bcmd(&mut self) -> _3b4bcmdW<Spi150Spec> {
        _3b4bcmdW::new(self, 24)
    }
    #[doc = "Bit 26 - Enable Command 0 for read"]
    #[inline(always)]
    pub fn enbl_cmd0for_read(&mut self) -> EnblCmd0forReadW<Spi150Spec> {
        EnblCmd0forReadW::new(self, 26)
    }
    #[doc = "Bit 27 - Enable Command 0 for write"]
    #[inline(always)]
    pub fn enbl_cmd0for_wr(&mut self) -> EnblCmd0forWrW<Spi150Spec> {
        EnblCmd0forWrW::new(self, 27)
    }
    #[doc = "Bit 28 - Enable Command 1 for read"]
    #[inline(always)]
    pub fn enbl_cmd1for_read(&mut self) -> EnblCmd1forReadW<Spi150Spec> {
        EnblCmd1forReadW::new(self, 28)
    }
    #[doc = "Bit 29 - Enable Command 1 for write"]
    #[inline(always)]
    pub fn enbl_cmd1for_wr(&mut self) -> EnblCmd1forWrW<Spi150Spec> {
        EnblCmd1forWrW::new(self, 29)
    }
    #[doc = "Bit 30 - Enable Command 2 for read"]
    #[inline(always)]
    pub fn enbl_cmd2for_read(&mut self) -> EnblCmd2forReadW<Spi150Spec> {
        EnblCmd2forReadW::new(self, 30)
    }
    #[doc = "Bit 31 - Enable Command 2 for write"]
    #[inline(always)]
    pub fn enbl_cmd2for_wr(&mut self) -> EnblCmd2forWrW<Spi150Spec> {
        EnblCmd2forWrW::new(self, 31)
    }
}
#[doc = "Address Qualified Command\n\nYou can [`read`](crate::Reg::read) this register and get [`spi150::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi150::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spi150Spec;
impl crate::RegisterSpec for Spi150Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi150::R`](R) reader structure"]
impl crate::Readable for Spi150Spec {}
#[doc = "`write(|w| ..)` method takes [`spi150::W`](W) writer structure"]
impl crate::Writable for Spi150Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPI150 to value 0x2800_1202"]
impl crate::Resettable for Spi150Spec {
    const RESET_VALUE: u32 = 0x2800_1202;
}
