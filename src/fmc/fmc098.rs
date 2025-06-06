#[doc = "Register `FMC098` reader"]
pub type R = crate::R<Fmc098Spec>;
#[doc = "Register `FMC098` writer"]
pub type W = crate::W<Fmc098Spec>;
#[doc = "Field `SPICLKHCLK2DelayCycleForDataInputLatchPoint1` reader - SPICLK = HCLK/2, delay cycle for data input latch point"]
pub type Spiclkhclk2delayCycleForDataInputLatchPoint1R = crate::FieldReader;
#[doc = "Field `SPICLKHCLK2DelayCycleForDataInputLatchPoint1` writer - SPICLK = HCLK/2, delay cycle for data input latch point"]
pub type Spiclkhclk2delayCycleForDataInputLatchPoint1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SPICLKHCLK3DelayCycleForDataInputLatchPoint1` reader - SPICLK = HCLK/3, delay cycle for data input latch point"]
pub type Spiclkhclk3delayCycleForDataInputLatchPoint1R = crate::FieldReader;
#[doc = "Field `SPICLKHCLK3DelayCycleForDataInputLatchPoint1` writer - SPICLK = HCLK/3, delay cycle for data input latch point"]
pub type Spiclkhclk3delayCycleForDataInputLatchPoint1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SPICLKHCLK4DelayCycleForDataInputLatchPoint1` reader - SPICLK = HCLK/4, delay cycle for data input latch point"]
pub type Spiclkhclk4delayCycleForDataInputLatchPoint1R = crate::FieldReader;
#[doc = "Field `SPICLKHCLK4DelayCycleForDataInputLatchPoint1` writer - SPICLK = HCLK/4, delay cycle for data input latch point"]
pub type Spiclkhclk4delayCycleForDataInputLatchPoint1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SPICLKHCLK5DelayCycleForDataInputLatchPoint1` reader - SPICLK = HCLK/5, delay cycle for data input latch point"]
pub type Spiclkhclk5delayCycleForDataInputLatchPoint1R = crate::FieldReader;
#[doc = "Field `SPICLKHCLK5DelayCycleForDataInputLatchPoint1` writer - SPICLK = HCLK/5, delay cycle for data input latch point"]
pub type Spiclkhclk5delayCycleForDataInputLatchPoint1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - SPICLK = HCLK/2, delay cycle for data input latch point"]
    #[inline(always)]
    pub fn spiclkhclk2delay_cycle_for_data_input_latch_point1(
        &self,
    ) -> Spiclkhclk2delayCycleForDataInputLatchPoint1R {
        Spiclkhclk2delayCycleForDataInputLatchPoint1R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - SPICLK = HCLK/3, delay cycle for data input latch point"]
    #[inline(always)]
    pub fn spiclkhclk3delay_cycle_for_data_input_latch_point1(
        &self,
    ) -> Spiclkhclk3delayCycleForDataInputLatchPoint1R {
        Spiclkhclk3delayCycleForDataInputLatchPoint1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - SPICLK = HCLK/4, delay cycle for data input latch point"]
    #[inline(always)]
    pub fn spiclkhclk4delay_cycle_for_data_input_latch_point1(
        &self,
    ) -> Spiclkhclk4delayCycleForDataInputLatchPoint1R {
        Spiclkhclk4delayCycleForDataInputLatchPoint1R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - SPICLK = HCLK/5, delay cycle for data input latch point"]
    #[inline(always)]
    pub fn spiclkhclk5delay_cycle_for_data_input_latch_point1(
        &self,
    ) -> Spiclkhclk5delayCycleForDataInputLatchPoint1R {
        Spiclkhclk5delayCycleForDataInputLatchPoint1R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - SPICLK = HCLK/2, delay cycle for data input latch point"]
    #[inline(always)]
    pub fn spiclkhclk2delay_cycle_for_data_input_latch_point1(
        &mut self,
    ) -> Spiclkhclk2delayCycleForDataInputLatchPoint1W<Fmc098Spec> {
        Spiclkhclk2delayCycleForDataInputLatchPoint1W::new(self, 0)
    }
    #[doc = "Bits 8:15 - SPICLK = HCLK/3, delay cycle for data input latch point"]
    #[inline(always)]
    pub fn spiclkhclk3delay_cycle_for_data_input_latch_point1(
        &mut self,
    ) -> Spiclkhclk3delayCycleForDataInputLatchPoint1W<Fmc098Spec> {
        Spiclkhclk3delayCycleForDataInputLatchPoint1W::new(self, 8)
    }
    #[doc = "Bits 16:23 - SPICLK = HCLK/4, delay cycle for data input latch point"]
    #[inline(always)]
    pub fn spiclkhclk4delay_cycle_for_data_input_latch_point1(
        &mut self,
    ) -> Spiclkhclk4delayCycleForDataInputLatchPoint1W<Fmc098Spec> {
        Spiclkhclk4delayCycleForDataInputLatchPoint1W::new(self, 16)
    }
    #[doc = "Bits 24:31 - SPICLK = HCLK/5, delay cycle for data input latch point"]
    #[inline(always)]
    pub fn spiclkhclk5delay_cycle_for_data_input_latch_point1(
        &mut self,
    ) -> Spiclkhclk5delayCycleForDataInputLatchPoint1W<Fmc098Spec> {
        Spiclkhclk5delayCycleForDataInputLatchPoint1W::new(self, 24)
    }
}
#[doc = "CE1 SPI Flash Read Timing Compensation\n\nYou can [`read`](crate::Reg::read) this register and get [`fmc098::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmc098::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fmc098Spec;
impl crate::RegisterSpec for Fmc098Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fmc098::R`](R) reader structure"]
impl crate::Readable for Fmc098Spec {}
#[doc = "`write(|w| ..)` method takes [`fmc098::W`](W) writer structure"]
impl crate::Writable for Fmc098Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FMC098 to value 0"]
impl crate::Resettable for Fmc098Spec {}
