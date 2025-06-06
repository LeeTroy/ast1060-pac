#[doc = "Register `SPI17C` reader"]
pub type R = crate::R<Spi17cSpec>;
#[doc = "Register `SPI17C` writer"]
pub type W = crate::W<Spi17cSpec>;
#[doc = "Field `Cmd011` reader - Command 0"]
pub type Cmd011R = crate::FieldReader;
#[doc = "Field `Cmd011` writer - Command 0"]
pub type Cmd011W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `Cmd111` reader - Command 1"]
pub type Cmd111R = crate::FieldReader;
#[doc = "Field `Cmd111` writer - Command 1"]
pub type Cmd111W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `Cmd211` reader - Command 2"]
pub type Cmd211R = crate::FieldReader;
#[doc = "Field `Cmd211` writer - Command 2"]
pub type Cmd211W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "3B/4B Command\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum _3b4bcmd11 {
    #[doc = "0: Command \\#2/\\#1/\\#0 are for 3B mode."]
    Command210AreFor3bMode = 0,
    #[doc = "1: Command \\#2/\\#1/\\#0 are for 4B mode."]
    Command210AreFor4bMode = 1,
}
impl From<_3b4bcmd11> for bool {
    #[inline(always)]
    fn from(variant: _3b4bcmd11) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `3B4BCmd11` reader - 3B/4B Command"]
pub type _3b4bcmd11R = crate::BitReader<_3b4bcmd11>;
impl _3b4bcmd11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> _3b4bcmd11 {
        match self.bits {
            false => _3b4bcmd11::Command210AreFor3bMode,
            true => _3b4bcmd11::Command210AreFor4bMode,
        }
    }
    #[doc = "Command \\#2/\\#1/\\#0 are for 3B mode."]
    #[inline(always)]
    pub fn is_command_210_are_for_3b_mode(&self) -> bool {
        *self == _3b4bcmd11::Command210AreFor3bMode
    }
    #[doc = "Command \\#2/\\#1/\\#0 are for 4B mode."]
    #[inline(always)]
    pub fn is_command_210_are_for_4b_mode(&self) -> bool {
        *self == _3b4bcmd11::Command210AreFor4bMode
    }
}
#[doc = "Field `3B4BCmd11` writer - 3B/4B Command"]
pub type _3b4bcmd11W<'a, REG> = crate::BitWriter<'a, REG, _3b4bcmd11>;
impl<'a, REG> _3b4bcmd11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Command \\#2/\\#1/\\#0 are for 3B mode."]
    #[inline(always)]
    pub fn command_210_are_for_3b_mode(self) -> &'a mut crate::W<REG> {
        self.variant(_3b4bcmd11::Command210AreFor3bMode)
    }
    #[doc = "Command \\#2/\\#1/\\#0 are for 4B mode."]
    #[inline(always)]
    pub fn command_210_are_for_4b_mode(self) -> &'a mut crate::W<REG> {
        self.variant(_3b4bcmd11::Command210AreFor4bMode)
    }
}
#[doc = "Field `Reserved11` reader - Reserved"]
pub type Reserved11R = crate::BitReader;
#[doc = "Field `EnblCmd0ForRead11` reader - Enable Command 0 for read"]
pub type EnblCmd0forRead11R = crate::BitReader;
#[doc = "Field `EnblCmd0ForRead11` writer - Enable Command 0 for read"]
pub type EnblCmd0forRead11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblCmd0ForWr11` reader - Enable Command 0 for write"]
pub type EnblCmd0forWr11R = crate::BitReader;
#[doc = "Field `EnblCmd0ForWr11` writer - Enable Command 0 for write"]
pub type EnblCmd0forWr11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblCmd1ForRead11` reader - Enable Command 1 for read"]
pub type EnblCmd1forRead11R = crate::BitReader;
#[doc = "Field `EnblCmd1ForRead11` writer - Enable Command 1 for read"]
pub type EnblCmd1forRead11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblCmd1ForWr11` reader - Enable Command 1 for write"]
pub type EnblCmd1forWr11R = crate::BitReader;
#[doc = "Field `EnblCmd1ForWr11` writer - Enable Command 1 for write"]
pub type EnblCmd1forWr11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblCmd2ForRead11` reader - Enable Command 2 for read"]
pub type EnblCmd2forRead11R = crate::BitReader;
#[doc = "Field `EnblCmd2ForRead11` writer - Enable Command 2 for read"]
pub type EnblCmd2forRead11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblCmd2ForWr11` reader - Enable Command 2 for write"]
pub type EnblCmd2forWr11R = crate::BitReader;
#[doc = "Field `EnblCmd2ForWr11` writer - Enable Command 2 for write"]
pub type EnblCmd2forWr11W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Command 0"]
    #[inline(always)]
    pub fn cmd011(&self) -> Cmd011R {
        Cmd011R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Command 1"]
    #[inline(always)]
    pub fn cmd111(&self) -> Cmd111R {
        Cmd111R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Command 2"]
    #[inline(always)]
    pub fn cmd211(&self) -> Cmd211R {
        Cmd211R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - 3B/4B Command"]
    #[inline(always)]
    pub fn _3b4bcmd11(&self) -> _3b4bcmd11R {
        _3b4bcmd11R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Reserved"]
    #[inline(always)]
    pub fn reserved11(&self) -> Reserved11R {
        Reserved11R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable Command 0 for read"]
    #[inline(always)]
    pub fn enbl_cmd0for_read11(&self) -> EnblCmd0forRead11R {
        EnblCmd0forRead11R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enable Command 0 for write"]
    #[inline(always)]
    pub fn enbl_cmd0for_wr11(&self) -> EnblCmd0forWr11R {
        EnblCmd0forWr11R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Enable Command 1 for read"]
    #[inline(always)]
    pub fn enbl_cmd1for_read11(&self) -> EnblCmd1forRead11R {
        EnblCmd1forRead11R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Enable Command 1 for write"]
    #[inline(always)]
    pub fn enbl_cmd1for_wr11(&self) -> EnblCmd1forWr11R {
        EnblCmd1forWr11R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Enable Command 2 for read"]
    #[inline(always)]
    pub fn enbl_cmd2for_read11(&self) -> EnblCmd2forRead11R {
        EnblCmd2forRead11R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable Command 2 for write"]
    #[inline(always)]
    pub fn enbl_cmd2for_wr11(&self) -> EnblCmd2forWr11R {
        EnblCmd2forWr11R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Command 0"]
    #[inline(always)]
    pub fn cmd011(&mut self) -> Cmd011W<Spi17cSpec> {
        Cmd011W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Command 1"]
    #[inline(always)]
    pub fn cmd111(&mut self) -> Cmd111W<Spi17cSpec> {
        Cmd111W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Command 2"]
    #[inline(always)]
    pub fn cmd211(&mut self) -> Cmd211W<Spi17cSpec> {
        Cmd211W::new(self, 16)
    }
    #[doc = "Bit 24 - 3B/4B Command"]
    #[inline(always)]
    pub fn _3b4bcmd11(&mut self) -> _3b4bcmd11W<Spi17cSpec> {
        _3b4bcmd11W::new(self, 24)
    }
    #[doc = "Bit 26 - Enable Command 0 for read"]
    #[inline(always)]
    pub fn enbl_cmd0for_read11(&mut self) -> EnblCmd0forRead11W<Spi17cSpec> {
        EnblCmd0forRead11W::new(self, 26)
    }
    #[doc = "Bit 27 - Enable Command 0 for write"]
    #[inline(always)]
    pub fn enbl_cmd0for_wr11(&mut self) -> EnblCmd0forWr11W<Spi17cSpec> {
        EnblCmd0forWr11W::new(self, 27)
    }
    #[doc = "Bit 28 - Enable Command 1 for read"]
    #[inline(always)]
    pub fn enbl_cmd1for_read11(&mut self) -> EnblCmd1forRead11W<Spi17cSpec> {
        EnblCmd1forRead11W::new(self, 28)
    }
    #[doc = "Bit 29 - Enable Command 1 for write"]
    #[inline(always)]
    pub fn enbl_cmd1for_wr11(&mut self) -> EnblCmd1forWr11W<Spi17cSpec> {
        EnblCmd1forWr11W::new(self, 29)
    }
    #[doc = "Bit 30 - Enable Command 2 for read"]
    #[inline(always)]
    pub fn enbl_cmd2for_read11(&mut self) -> EnblCmd2forRead11W<Spi17cSpec> {
        EnblCmd2forRead11W::new(self, 30)
    }
    #[doc = "Bit 31 - Enable Command 2 for write"]
    #[inline(always)]
    pub fn enbl_cmd2for_wr11(&mut self) -> EnblCmd2forWr11W<Spi17cSpec> {
        EnblCmd2forWr11W::new(self, 31)
    }
}
#[doc = "Address Qualified Command\n\nYou can [`read`](crate::Reg::read) this register and get [`spi17c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi17c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spi17cSpec;
impl crate::RegisterSpec for Spi17cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi17c::R`](R) reader structure"]
impl crate::Readable for Spi17cSpec {}
#[doc = "`write(|w| ..)` method takes [`spi17c::W`](W) writer structure"]
impl crate::Writable for Spi17cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPI17C to value 0"]
impl crate::Resettable for Spi17cSpec {}
