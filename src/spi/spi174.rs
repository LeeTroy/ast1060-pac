#[doc = "Register `SPI174` reader"]
pub type R = crate::R<Spi174Spec>;
#[doc = "Register `SPI174` writer"]
pub type W = crate::W<Spi174Spec>;
#[doc = "Field `Cmd09` reader - Command 0"]
pub type Cmd09R = crate::FieldReader;
#[doc = "Field `Cmd09` writer - Command 0"]
pub type Cmd09W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `Cmd19` reader - Command 1"]
pub type Cmd19R = crate::FieldReader;
#[doc = "Field `Cmd19` writer - Command 1"]
pub type Cmd19W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `Cmd29` reader - Command 2"]
pub type Cmd29R = crate::FieldReader;
#[doc = "Field `Cmd29` writer - Command 2"]
pub type Cmd29W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "3B/4B Command\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum _3b4bcmd9 {
    #[doc = "0: Command \\#2/\\#1/\\#0 are for 3B mode."]
    Command210AreFor3bMode = 0,
    #[doc = "1: Command \\#2/\\#1/\\#0 are for 4B mode."]
    Command210AreFor4bMode = 1,
}
impl From<_3b4bcmd9> for bool {
    #[inline(always)]
    fn from(variant: _3b4bcmd9) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `3B4BCmd9` reader - 3B/4B Command"]
pub type _3b4bcmd9R = crate::BitReader<_3b4bcmd9>;
impl _3b4bcmd9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> _3b4bcmd9 {
        match self.bits {
            false => _3b4bcmd9::Command210AreFor3bMode,
            true => _3b4bcmd9::Command210AreFor4bMode,
        }
    }
    #[doc = "Command \\#2/\\#1/\\#0 are for 3B mode."]
    #[inline(always)]
    pub fn is_command_210_are_for_3b_mode(&self) -> bool {
        *self == _3b4bcmd9::Command210AreFor3bMode
    }
    #[doc = "Command \\#2/\\#1/\\#0 are for 4B mode."]
    #[inline(always)]
    pub fn is_command_210_are_for_4b_mode(&self) -> bool {
        *self == _3b4bcmd9::Command210AreFor4bMode
    }
}
#[doc = "Field `3B4BCmd9` writer - 3B/4B Command"]
pub type _3b4bcmd9W<'a, REG> = crate::BitWriter<'a, REG, _3b4bcmd9>;
impl<'a, REG> _3b4bcmd9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Command \\#2/\\#1/\\#0 are for 3B mode."]
    #[inline(always)]
    pub fn command_210_are_for_3b_mode(self) -> &'a mut crate::W<REG> {
        self.variant(_3b4bcmd9::Command210AreFor3bMode)
    }
    #[doc = "Command \\#2/\\#1/\\#0 are for 4B mode."]
    #[inline(always)]
    pub fn command_210_are_for_4b_mode(self) -> &'a mut crate::W<REG> {
        self.variant(_3b4bcmd9::Command210AreFor4bMode)
    }
}
#[doc = "Field `Reserved9` reader - Reserved"]
pub type Reserved9R = crate::BitReader;
#[doc = "Field `EnblCmd0ForRead9` reader - Enable Command 0 for read"]
pub type EnblCmd0forRead9R = crate::BitReader;
#[doc = "Field `EnblCmd0ForRead9` writer - Enable Command 0 for read"]
pub type EnblCmd0forRead9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblCmd0ForWr9` reader - Enable Command 0 for write"]
pub type EnblCmd0forWr9R = crate::BitReader;
#[doc = "Field `EnblCmd0ForWr9` writer - Enable Command 0 for write"]
pub type EnblCmd0forWr9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblCmd1ForRead9` reader - Enable Command 1 for read"]
pub type EnblCmd1forRead9R = crate::BitReader;
#[doc = "Field `EnblCmd1ForRead9` writer - Enable Command 1 for read"]
pub type EnblCmd1forRead9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblCmd1ForWr9` reader - Enable Command 1 for write"]
pub type EnblCmd1forWr9R = crate::BitReader;
#[doc = "Field `EnblCmd1ForWr9` writer - Enable Command 1 for write"]
pub type EnblCmd1forWr9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblCmd2ForRead9` reader - Enable Command 2 for read"]
pub type EnblCmd2forRead9R = crate::BitReader;
#[doc = "Field `EnblCmd2ForRead9` writer - Enable Command 2 for read"]
pub type EnblCmd2forRead9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblCmd2ForWr9` reader - Enable Command 2 for write"]
pub type EnblCmd2forWr9R = crate::BitReader;
#[doc = "Field `EnblCmd2ForWr9` writer - Enable Command 2 for write"]
pub type EnblCmd2forWr9W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Command 0"]
    #[inline(always)]
    pub fn cmd09(&self) -> Cmd09R {
        Cmd09R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Command 1"]
    #[inline(always)]
    pub fn cmd19(&self) -> Cmd19R {
        Cmd19R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Command 2"]
    #[inline(always)]
    pub fn cmd29(&self) -> Cmd29R {
        Cmd29R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - 3B/4B Command"]
    #[inline(always)]
    pub fn _3b4bcmd9(&self) -> _3b4bcmd9R {
        _3b4bcmd9R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Reserved"]
    #[inline(always)]
    pub fn reserved9(&self) -> Reserved9R {
        Reserved9R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable Command 0 for read"]
    #[inline(always)]
    pub fn enbl_cmd0for_read9(&self) -> EnblCmd0forRead9R {
        EnblCmd0forRead9R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enable Command 0 for write"]
    #[inline(always)]
    pub fn enbl_cmd0for_wr9(&self) -> EnblCmd0forWr9R {
        EnblCmd0forWr9R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Enable Command 1 for read"]
    #[inline(always)]
    pub fn enbl_cmd1for_read9(&self) -> EnblCmd1forRead9R {
        EnblCmd1forRead9R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Enable Command 1 for write"]
    #[inline(always)]
    pub fn enbl_cmd1for_wr9(&self) -> EnblCmd1forWr9R {
        EnblCmd1forWr9R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Enable Command 2 for read"]
    #[inline(always)]
    pub fn enbl_cmd2for_read9(&self) -> EnblCmd2forRead9R {
        EnblCmd2forRead9R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable Command 2 for write"]
    #[inline(always)]
    pub fn enbl_cmd2for_wr9(&self) -> EnblCmd2forWr9R {
        EnblCmd2forWr9R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Command 0"]
    #[inline(always)]
    pub fn cmd09(&mut self) -> Cmd09W<Spi174Spec> {
        Cmd09W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Command 1"]
    #[inline(always)]
    pub fn cmd19(&mut self) -> Cmd19W<Spi174Spec> {
        Cmd19W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Command 2"]
    #[inline(always)]
    pub fn cmd29(&mut self) -> Cmd29W<Spi174Spec> {
        Cmd29W::new(self, 16)
    }
    #[doc = "Bit 24 - 3B/4B Command"]
    #[inline(always)]
    pub fn _3b4bcmd9(&mut self) -> _3b4bcmd9W<Spi174Spec> {
        _3b4bcmd9W::new(self, 24)
    }
    #[doc = "Bit 26 - Enable Command 0 for read"]
    #[inline(always)]
    pub fn enbl_cmd0for_read9(&mut self) -> EnblCmd0forRead9W<Spi174Spec> {
        EnblCmd0forRead9W::new(self, 26)
    }
    #[doc = "Bit 27 - Enable Command 0 for write"]
    #[inline(always)]
    pub fn enbl_cmd0for_wr9(&mut self) -> EnblCmd0forWr9W<Spi174Spec> {
        EnblCmd0forWr9W::new(self, 27)
    }
    #[doc = "Bit 28 - Enable Command 1 for read"]
    #[inline(always)]
    pub fn enbl_cmd1for_read9(&mut self) -> EnblCmd1forRead9W<Spi174Spec> {
        EnblCmd1forRead9W::new(self, 28)
    }
    #[doc = "Bit 29 - Enable Command 1 for write"]
    #[inline(always)]
    pub fn enbl_cmd1for_wr9(&mut self) -> EnblCmd1forWr9W<Spi174Spec> {
        EnblCmd1forWr9W::new(self, 29)
    }
    #[doc = "Bit 30 - Enable Command 2 for read"]
    #[inline(always)]
    pub fn enbl_cmd2for_read9(&mut self) -> EnblCmd2forRead9W<Spi174Spec> {
        EnblCmd2forRead9W::new(self, 30)
    }
    #[doc = "Bit 31 - Enable Command 2 for write"]
    #[inline(always)]
    pub fn enbl_cmd2for_wr9(&mut self) -> EnblCmd2forWr9W<Spi174Spec> {
        EnblCmd2forWr9W::new(self, 31)
    }
}
#[doc = "Address Qualified Command\n\nYou can [`read`](crate::Reg::read) this register and get [`spi174::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi174::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spi174Spec;
impl crate::RegisterSpec for Spi174Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi174::R`](R) reader structure"]
impl crate::Readable for Spi174Spec {}
#[doc = "`write(|w| ..)` method takes [`spi174::W`](W) writer structure"]
impl crate::Writable for Spi174Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPI174 to value 0"]
impl crate::Resettable for Spi174Spec {}
