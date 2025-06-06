#[doc = "Register `I3CD03C` reader"]
pub type R = crate::R<I3cd03cSpec>;
#[doc = "Register `I3CD03C` writer"]
pub type W = crate::W<I3cd03cSpec>;
#[doc = "Field `TXTHLDSTAT` reader - TX_THLD_STAT"]
pub type TxthldstatR = crate::BitReader;
#[doc = "Field `TXTHLDSTAT` writer - TX_THLD_STAT"]
pub type TxthldstatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXTHLDSTAT` reader - RX_THLD_STAT"]
pub type RxthldstatR = crate::BitReader;
#[doc = "Field `RXTHLDSTAT` writer - RX_THLD_STAT"]
pub type RxthldstatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IBITHLDSTAT` reader - IBI_THLD_STAT"]
pub type IbithldstatR = crate::BitReader;
#[doc = "Field `IBITHLDSTAT` writer - IBI_THLD_STAT"]
pub type IbithldstatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMDQUEUEREADYSTAT` reader - CMD_QUEUE_READY_STAT"]
pub type CmdqueuereadystatR = crate::BitReader;
#[doc = "Field `CMDQUEUEREADYSTAT` writer - CMD_QUEUE_READY_STAT"]
pub type CmdqueuereadystatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESPREADYSTATINTR` reader - RESP_READY_STAT_INTR"]
pub type RespreadystatintrR = crate::BitReader;
#[doc = "Field `RESPREADYSTATINTR` writer - RESP_READY_STAT_INTR"]
pub type RespreadystatintrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRANSFERABORTSTAT` reader - TRANSFER_ABORT_STAT"]
pub type TransferabortstatR = crate::BitReader;
#[doc = "Field `TRANSFERABORTSTAT` writer - TRANSFER_ABORT_STAT"]
pub type TransferabortstatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCCUPDATEDSTAT` reader - CCC_UPDATED_STAT"]
pub type CccupdatedstatR = crate::BitReader;
#[doc = "Field `CCCUPDATEDSTAT` writer - CCC_UPDATED_STAT"]
pub type CccupdatedstatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSVD7` reader - This bit in Interrupt Status register is reserved."]
pub type Rsvd7R = crate::BitReader;
#[doc = "Field `DYNADDRASSGNSTAT` reader - DYN_ADDR_ASSGN_STAT"]
pub type DynaddrassgnstatR = crate::BitReader;
#[doc = "Field `DYNADDRASSGNSTAT` writer - DYN_ADDR_ASSGN_STAT"]
pub type DynaddrassgnstatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRANSFERERRSTAT` reader - TRANSFER_ERR_STAT"]
pub type TransfererrstatR = crate::BitReader;
#[doc = "Field `TRANSFERERRSTAT` writer - TRANSFER_ERR_STAT"]
pub type TransfererrstatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEFSLVSTAT` reader - DEFSLV_STAT"]
pub type DefslvstatR = crate::BitReader;
#[doc = "Field `DEFSLVSTAT` writer - DEFSLV_STAT"]
pub type DefslvstatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `READREQRECVSTAT` reader - READ_REQ_RECV_STAT"]
pub type ReadreqrecvstatR = crate::BitReader;
#[doc = "Field `READREQRECVSTAT` writer - READ_REQ_RECV_STAT"]
pub type ReadreqrecvstatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IBIUPDATEDSTAT` reader - IBI_UPDATED_STAT"]
pub type IbiupdatedstatR = crate::BitReader;
#[doc = "Field `IBIUPDATEDSTAT` writer - IBI_UPDATED_STAT"]
pub type IbiupdatedstatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUSOWNERUPDATEDSTAT` reader - BUSOWNER_UPDATED_STAT"]
pub type BusownerupdatedstatR = crate::BitReader;
#[doc = "Field `BUSOWNERUPDATEDSTAT` writer - BUSOWNER_UPDATED_STAT"]
pub type BusownerupdatedstatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSVD1` reader - reserved"]
pub type Rsvd1R = crate::BitReader;
#[doc = "Field `BUSRESETDONESTS` reader - BUS_RESET_DONE_STS"]
pub type BusresetdonestsR = crate::BitReader;
#[doc = "Field `BUSRESETDONESTS` writer - BUS_RESET_DONE_STS"]
pub type BusresetdonestsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSVD` reader - reserved"]
pub type RsvdR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bit 0 - TX_THLD_STAT"]
    #[inline(always)]
    pub fn txthldstat(&self) -> TxthldstatR {
        TxthldstatR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RX_THLD_STAT"]
    #[inline(always)]
    pub fn rxthldstat(&self) -> RxthldstatR {
        RxthldstatR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - IBI_THLD_STAT"]
    #[inline(always)]
    pub fn ibithldstat(&self) -> IbithldstatR {
        IbithldstatR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CMD_QUEUE_READY_STAT"]
    #[inline(always)]
    pub fn cmdqueuereadystat(&self) -> CmdqueuereadystatR {
        CmdqueuereadystatR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RESP_READY_STAT_INTR"]
    #[inline(always)]
    pub fn respreadystatintr(&self) -> RespreadystatintrR {
        RespreadystatintrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TRANSFER_ABORT_STAT"]
    #[inline(always)]
    pub fn transferabortstat(&self) -> TransferabortstatR {
        TransferabortstatR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CCC_UPDATED_STAT"]
    #[inline(always)]
    pub fn cccupdatedstat(&self) -> CccupdatedstatR {
        CccupdatedstatR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - This bit in Interrupt Status register is reserved."]
    #[inline(always)]
    pub fn rsvd7(&self) -> Rsvd7R {
        Rsvd7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - DYN_ADDR_ASSGN_STAT"]
    #[inline(always)]
    pub fn dynaddrassgnstat(&self) -> DynaddrassgnstatR {
        DynaddrassgnstatR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - TRANSFER_ERR_STAT"]
    #[inline(always)]
    pub fn transfererrstat(&self) -> TransfererrstatR {
        TransfererrstatR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - DEFSLV_STAT"]
    #[inline(always)]
    pub fn defslvstat(&self) -> DefslvstatR {
        DefslvstatR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - READ_REQ_RECV_STAT"]
    #[inline(always)]
    pub fn readreqrecvstat(&self) -> ReadreqrecvstatR {
        ReadreqrecvstatR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - IBI_UPDATED_STAT"]
    #[inline(always)]
    pub fn ibiupdatedstat(&self) -> IbiupdatedstatR {
        IbiupdatedstatR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - BUSOWNER_UPDATED_STAT"]
    #[inline(always)]
    pub fn busownerupdatedstat(&self) -> BusownerupdatedstatR {
        BusownerupdatedstatR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - reserved"]
    #[inline(always)]
    pub fn rsvd1(&self) -> Rsvd1R {
        Rsvd1R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - BUS_RESET_DONE_STS"]
    #[inline(always)]
    pub fn busresetdonests(&self) -> BusresetdonestsR {
        BusresetdonestsR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31 - reserved"]
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - TX_THLD_STAT"]
    #[inline(always)]
    pub fn txthldstat(&mut self) -> TxthldstatW<I3cd03cSpec> {
        TxthldstatW::new(self, 0)
    }
    #[doc = "Bit 1 - RX_THLD_STAT"]
    #[inline(always)]
    pub fn rxthldstat(&mut self) -> RxthldstatW<I3cd03cSpec> {
        RxthldstatW::new(self, 1)
    }
    #[doc = "Bit 2 - IBI_THLD_STAT"]
    #[inline(always)]
    pub fn ibithldstat(&mut self) -> IbithldstatW<I3cd03cSpec> {
        IbithldstatW::new(self, 2)
    }
    #[doc = "Bit 3 - CMD_QUEUE_READY_STAT"]
    #[inline(always)]
    pub fn cmdqueuereadystat(&mut self) -> CmdqueuereadystatW<I3cd03cSpec> {
        CmdqueuereadystatW::new(self, 3)
    }
    #[doc = "Bit 4 - RESP_READY_STAT_INTR"]
    #[inline(always)]
    pub fn respreadystatintr(&mut self) -> RespreadystatintrW<I3cd03cSpec> {
        RespreadystatintrW::new(self, 4)
    }
    #[doc = "Bit 5 - TRANSFER_ABORT_STAT"]
    #[inline(always)]
    pub fn transferabortstat(&mut self) -> TransferabortstatW<I3cd03cSpec> {
        TransferabortstatW::new(self, 5)
    }
    #[doc = "Bit 6 - CCC_UPDATED_STAT"]
    #[inline(always)]
    pub fn cccupdatedstat(&mut self) -> CccupdatedstatW<I3cd03cSpec> {
        CccupdatedstatW::new(self, 6)
    }
    #[doc = "Bit 8 - DYN_ADDR_ASSGN_STAT"]
    #[inline(always)]
    pub fn dynaddrassgnstat(&mut self) -> DynaddrassgnstatW<I3cd03cSpec> {
        DynaddrassgnstatW::new(self, 8)
    }
    #[doc = "Bit 9 - TRANSFER_ERR_STAT"]
    #[inline(always)]
    pub fn transfererrstat(&mut self) -> TransfererrstatW<I3cd03cSpec> {
        TransfererrstatW::new(self, 9)
    }
    #[doc = "Bit 10 - DEFSLV_STAT"]
    #[inline(always)]
    pub fn defslvstat(&mut self) -> DefslvstatW<I3cd03cSpec> {
        DefslvstatW::new(self, 10)
    }
    #[doc = "Bit 11 - READ_REQ_RECV_STAT"]
    #[inline(always)]
    pub fn readreqrecvstat(&mut self) -> ReadreqrecvstatW<I3cd03cSpec> {
        ReadreqrecvstatW::new(self, 11)
    }
    #[doc = "Bit 12 - IBI_UPDATED_STAT"]
    #[inline(always)]
    pub fn ibiupdatedstat(&mut self) -> IbiupdatedstatW<I3cd03cSpec> {
        IbiupdatedstatW::new(self, 12)
    }
    #[doc = "Bit 13 - BUSOWNER_UPDATED_STAT"]
    #[inline(always)]
    pub fn busownerupdatedstat(&mut self) -> BusownerupdatedstatW<I3cd03cSpec> {
        BusownerupdatedstatW::new(self, 13)
    }
    #[doc = "Bit 15 - BUS_RESET_DONE_STS"]
    #[inline(always)]
    pub fn busresetdonests(&mut self) -> BusresetdonestsW<I3cd03cSpec> {
        BusresetdonestsW::new(self, 15)
    }
}
#[doc = "Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cd03c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cd03c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3cd03cSpec;
impl crate::RegisterSpec for I3cd03cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3cd03c::R`](R) reader structure"]
impl crate::Readable for I3cd03cSpec {}
#[doc = "`write(|w| ..)` method takes [`i3cd03c::W`](W) writer structure"]
impl crate::Writable for I3cd03cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CD03C to value 0"]
impl crate::Resettable for I3cd03cSpec {}
