#[doc = "Register `SCU638` reader"]
pub type R = crate::R<Scu638Spec>;
#[doc = "Register `SCU638` writer"]
pub type W = crate::W<Scu638Spec>;
#[doc = "Field `DisGPIOY0IntPullDown` reader - Disable GPIOY0 Internal Pull-Down"]
pub type DisGpioy0intPullDownR = crate::BitReader;
#[doc = "Field `DisGPIOY0IntPullDown` writer - Disable GPIOY0 Internal Pull-Down"]
pub type DisGpioy0intPullDownW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DisGPIOY1IntPullDown` reader - Disable GPIOY1 Internal Pull-Down"]
pub type DisGpioy1intPullDownR = crate::BitReader;
#[doc = "Field `DisGPIOY1IntPullDown` writer - Disable GPIOY1 Internal Pull-Down"]
pub type DisGpioy1intPullDownW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DisGPIOY2IntPullDown` reader - Disable GPIOY2 Internal Pull-Down"]
pub type DisGpioy2intPullDownR = crate::BitReader;
#[doc = "Field `DisGPIOY2IntPullDown` writer - Disable GPIOY2 Internal Pull-Down"]
pub type DisGpioy2intPullDownW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DisGPIOY3IntPullDown` reader - Disable GPIOY3 Internal Pull-Down"]
pub type DisGpioy3intPullDownR = crate::BitReader;
#[doc = "Field `DisGPIOY3IntPullDown` writer - Disable GPIOY3 Internal Pull-Down"]
pub type DisGpioy3intPullDownW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DisGPIOY4IntPullDown` reader - Disable GPIOY4 Internal Pull-Down"]
pub type DisGpioy4intPullDownR = crate::BitReader;
#[doc = "Field `DisGPIOY4IntPullDown` writer - Disable GPIOY4 Internal Pull-Down"]
pub type DisGpioy4intPullDownW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DisGPIOY5IntPullDown` reader - Disable GPIOY5 Internal Pull-Down"]
pub type DisGpioy5intPullDownR = crate::BitReader;
#[doc = "Field `DisGPIOY5IntPullDown` writer - Disable GPIOY5 Internal Pull-Down"]
pub type DisGpioy5intPullDownW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DisGPIOY6IntPullDown` reader - Disable GPIOY6 Internal Pull-Down"]
pub type DisGpioy6intPullDownR = crate::BitReader;
#[doc = "Field `DisGPIOY6IntPullDown` writer - Disable GPIOY6 Internal Pull-Down"]
pub type DisGpioy6intPullDownW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - Reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `Reserved2` writer - Reserved"]
pub type Reserved2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DisGPIOZ0IntPullDown` reader - Disable GPIOZ0 Internal Pull-Down"]
pub type DisGpioz0intPullDownR = crate::BitReader;
#[doc = "Field `DisGPIOZ0IntPullDown` writer - Disable GPIOZ0 Internal Pull-Down"]
pub type DisGpioz0intPullDownW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DisGPIOZ1IntPullDown` reader - Disable GPIOZ1 Internal Pull-Down"]
pub type DisGpioz1intPullDownR = crate::BitReader;
#[doc = "Field `DisGPIOZ1IntPullDown` writer - Disable GPIOZ1 Internal Pull-Down"]
pub type DisGpioz1intPullDownW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `Reserved1` writer - Reserved"]
pub type Reserved1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DisGPIOZ3IntPullDown` reader - Disable GPIOZ3 Internal Pull-Down"]
pub type DisGpioz3intPullDownR = crate::BitReader;
#[doc = "Field `DisGPIOZ3IntPullDown` writer - Disable GPIOZ3 Internal Pull-Down"]
pub type DisGpioz3intPullDownW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DisGPIOZ4IntPullDown` reader - Disable GPIOZ4 Internal Pull-Down"]
pub type DisGpioz4intPullDownR = crate::BitReader;
#[doc = "Field `DisGPIOZ4IntPullDown` writer - Disable GPIOZ4 Internal Pull-Down"]
pub type DisGpioz4intPullDownW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DisGPIOZ5IntPullDown` reader - Disable GPIOZ5 Internal Pull-Down"]
pub type DisGpioz5intPullDownR = crate::BitReader;
#[doc = "Field `DisGPIOZ5IntPullDown` writer - Disable GPIOZ5 Internal Pull-Down"]
pub type DisGpioz5intPullDownW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DisGPIOZ6IntPullDown` reader - Disable GPIOZ6 Internal Pull-Down"]
pub type DisGpioz6intPullDownR = crate::BitReader;
#[doc = "Field `DisGPIOZ6IntPullDown` writer - Disable GPIOZ6 Internal Pull-Down"]
pub type DisGpioz6intPullDownW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DisGPIOZ7IntPullDown` reader - Disable GPIOZ7 Internal Pull-Down"]
pub type DisGpioz7intPullDownR = crate::BitReader;
#[doc = "Field `DisGPIOZ7IntPullDown` writer - Disable GPIOZ7 Internal Pull-Down"]
pub type DisGpioz7intPullDownW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Disable GPIOY0 Internal Pull-Down"]
    #[inline(always)]
    pub fn dis_gpioy0int_pull_down(&self) -> DisGpioy0intPullDownR {
        DisGpioy0intPullDownR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Disable GPIOY1 Internal Pull-Down"]
    #[inline(always)]
    pub fn dis_gpioy1int_pull_down(&self) -> DisGpioy1intPullDownR {
        DisGpioy1intPullDownR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Disable GPIOY2 Internal Pull-Down"]
    #[inline(always)]
    pub fn dis_gpioy2int_pull_down(&self) -> DisGpioy2intPullDownR {
        DisGpioy2intPullDownR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Disable GPIOY3 Internal Pull-Down"]
    #[inline(always)]
    pub fn dis_gpioy3int_pull_down(&self) -> DisGpioy3intPullDownR {
        DisGpioy3intPullDownR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Disable GPIOY4 Internal Pull-Down"]
    #[inline(always)]
    pub fn dis_gpioy4int_pull_down(&self) -> DisGpioy4intPullDownR {
        DisGpioy4intPullDownR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Disable GPIOY5 Internal Pull-Down"]
    #[inline(always)]
    pub fn dis_gpioy5int_pull_down(&self) -> DisGpioy5intPullDownR {
        DisGpioy5intPullDownR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Disable GPIOY6 Internal Pull-Down"]
    #[inline(always)]
    pub fn dis_gpioy6int_pull_down(&self) -> DisGpioy6intPullDownR {
        DisGpioy6intPullDownR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Disable GPIOZ0 Internal Pull-Down"]
    #[inline(always)]
    pub fn dis_gpioz0int_pull_down(&self) -> DisGpioz0intPullDownR {
        DisGpioz0intPullDownR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Disable GPIOZ1 Internal Pull-Down"]
    #[inline(always)]
    pub fn dis_gpioz1int_pull_down(&self) -> DisGpioz1intPullDownR {
        DisGpioz1intPullDownR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Disable GPIOZ3 Internal Pull-Down"]
    #[inline(always)]
    pub fn dis_gpioz3int_pull_down(&self) -> DisGpioz3intPullDownR {
        DisGpioz3intPullDownR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Disable GPIOZ4 Internal Pull-Down"]
    #[inline(always)]
    pub fn dis_gpioz4int_pull_down(&self) -> DisGpioz4intPullDownR {
        DisGpioz4intPullDownR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Disable GPIOZ5 Internal Pull-Down"]
    #[inline(always)]
    pub fn dis_gpioz5int_pull_down(&self) -> DisGpioz5intPullDownR {
        DisGpioz5intPullDownR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Disable GPIOZ6 Internal Pull-Down"]
    #[inline(always)]
    pub fn dis_gpioz6int_pull_down(&self) -> DisGpioz6intPullDownR {
        DisGpioz6intPullDownR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Disable GPIOZ7 Internal Pull-Down"]
    #[inline(always)]
    pub fn dis_gpioz7int_pull_down(&self) -> DisGpioz7intPullDownR {
        DisGpioz7intPullDownR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Disable GPIOY0 Internal Pull-Down"]
    #[inline(always)]
    pub fn dis_gpioy0int_pull_down(&mut self) -> DisGpioy0intPullDownW<Scu638Spec> {
        DisGpioy0intPullDownW::new(self, 0)
    }
    #[doc = "Bit 1 - Disable GPIOY1 Internal Pull-Down"]
    #[inline(always)]
    pub fn dis_gpioy1int_pull_down(&mut self) -> DisGpioy1intPullDownW<Scu638Spec> {
        DisGpioy1intPullDownW::new(self, 1)
    }
    #[doc = "Bit 2 - Disable GPIOY2 Internal Pull-Down"]
    #[inline(always)]
    pub fn dis_gpioy2int_pull_down(&mut self) -> DisGpioy2intPullDownW<Scu638Spec> {
        DisGpioy2intPullDownW::new(self, 2)
    }
    #[doc = "Bit 3 - Disable GPIOY3 Internal Pull-Down"]
    #[inline(always)]
    pub fn dis_gpioy3int_pull_down(&mut self) -> DisGpioy3intPullDownW<Scu638Spec> {
        DisGpioy3intPullDownW::new(self, 3)
    }
    #[doc = "Bit 4 - Disable GPIOY4 Internal Pull-Down"]
    #[inline(always)]
    pub fn dis_gpioy4int_pull_down(&mut self) -> DisGpioy4intPullDownW<Scu638Spec> {
        DisGpioy4intPullDownW::new(self, 4)
    }
    #[doc = "Bit 5 - Disable GPIOY5 Internal Pull-Down"]
    #[inline(always)]
    pub fn dis_gpioy5int_pull_down(&mut self) -> DisGpioy5intPullDownW<Scu638Spec> {
        DisGpioy5intPullDownW::new(self, 5)
    }
    #[doc = "Bit 6 - Disable GPIOY6 Internal Pull-Down"]
    #[inline(always)]
    pub fn dis_gpioy6int_pull_down(&mut self) -> DisGpioy6intPullDownW<Scu638Spec> {
        DisGpioy6intPullDownW::new(self, 6)
    }
    #[doc = "Bit 7 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&mut self) -> Reserved2W<Scu638Spec> {
        Reserved2W::new(self, 7)
    }
    #[doc = "Bit 8 - Disable GPIOZ0 Internal Pull-Down"]
    #[inline(always)]
    pub fn dis_gpioz0int_pull_down(&mut self) -> DisGpioz0intPullDownW<Scu638Spec> {
        DisGpioz0intPullDownW::new(self, 8)
    }
    #[doc = "Bit 9 - Disable GPIOZ1 Internal Pull-Down"]
    #[inline(always)]
    pub fn dis_gpioz1int_pull_down(&mut self) -> DisGpioz1intPullDownW<Scu638Spec> {
        DisGpioz1intPullDownW::new(self, 9)
    }
    #[doc = "Bit 10 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<Scu638Spec> {
        Reserved1W::new(self, 10)
    }
    #[doc = "Bit 11 - Disable GPIOZ3 Internal Pull-Down"]
    #[inline(always)]
    pub fn dis_gpioz3int_pull_down(&mut self) -> DisGpioz3intPullDownW<Scu638Spec> {
        DisGpioz3intPullDownW::new(self, 11)
    }
    #[doc = "Bit 12 - Disable GPIOZ4 Internal Pull-Down"]
    #[inline(always)]
    pub fn dis_gpioz4int_pull_down(&mut self) -> DisGpioz4intPullDownW<Scu638Spec> {
        DisGpioz4intPullDownW::new(self, 12)
    }
    #[doc = "Bit 13 - Disable GPIOZ5 Internal Pull-Down"]
    #[inline(always)]
    pub fn dis_gpioz5int_pull_down(&mut self) -> DisGpioz5intPullDownW<Scu638Spec> {
        DisGpioz5intPullDownW::new(self, 13)
    }
    #[doc = "Bit 14 - Disable GPIOZ6 Internal Pull-Down"]
    #[inline(always)]
    pub fn dis_gpioz6int_pull_down(&mut self) -> DisGpioz6intPullDownW<Scu638Spec> {
        DisGpioz6intPullDownW::new(self, 14)
    }
    #[doc = "Bit 15 - Disable GPIOZ7 Internal Pull-Down"]
    #[inline(always)]
    pub fn dis_gpioz7int_pull_down(&mut self) -> DisGpioz7intPullDownW<Scu638Spec> {
        DisGpioz7intPullDownW::new(self, 15)
    }
}
#[doc = "Disable GPIO Internal Pull-Down \\#6\n\nYou can [`read`](crate::Reg::read) this register and get [`scu638::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu638::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu638Spec;
impl crate::RegisterSpec for Scu638Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu638::R`](R) reader structure"]
impl crate::Readable for Scu638Spec {}
#[doc = "`write(|w| ..)` method takes [`scu638::W`](W) writer structure"]
impl crate::Writable for Scu638Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU638 to value 0x000f_0000"]
impl crate::Resettable for Scu638Spec {
    const RESET_VALUE: u32 = 0x000f_0000;
}
