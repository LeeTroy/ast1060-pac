#[doc = "Register `SECURE80C` reader"]
pub type R = crate::R<Secure80cSpec>;
#[doc = "Register `SECURE80C` writer"]
pub type W = crate::W<Secure80cSpec>;
#[doc = "Vault Key Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VaultKeySel {
    #[doc = "0: Select first vault key for HACE"]
    SelectFirstVaultKeyForHace = 0,
    #[doc = "1: Select second valut key for HACE"]
    SelectSecondValutKeyForHace = 1,
}
impl From<VaultKeySel> for bool {
    #[inline(always)]
    fn from(variant: VaultKeySel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VaultKeySel` reader - Vault Key Selection"]
pub type VaultKeySelR = crate::BitReader<VaultKeySel>;
impl VaultKeySelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VaultKeySel {
        match self.bits {
            false => VaultKeySel::SelectFirstVaultKeyForHace,
            true => VaultKeySel::SelectSecondValutKeyForHace,
        }
    }
    #[doc = "Select first vault key for HACE"]
    #[inline(always)]
    pub fn is_select_first_vault_key_for_hace(&self) -> bool {
        *self == VaultKeySel::SelectFirstVaultKeyForHace
    }
    #[doc = "Select second valut key for HACE"]
    #[inline(always)]
    pub fn is_select_second_valut_key_for_hace(&self) -> bool {
        *self == VaultKeySel::SelectSecondValutKeyForHace
    }
}
#[doc = "Field `VaultKeySel` writer - Vault Key Selection"]
pub type VaultKeySelW<'a, REG> = crate::BitWriter<'a, REG, VaultKeySel>;
impl<'a, REG> VaultKeySelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Select first vault key for HACE"]
    #[inline(always)]
    pub fn select_first_vault_key_for_hace(self) -> &'a mut crate::W<REG> {
        self.variant(VaultKeySel::SelectFirstVaultKeyForHace)
    }
    #[doc = "Select second valut key for HACE"]
    #[inline(always)]
    pub fn select_second_valut_key_for_hace(self) -> &'a mut crate::W<REG> {
        self.variant(VaultKeySel::SelectSecondValutKeyForHace)
    }
}
#[doc = "Vault Key Write Protection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VaultKeyWrProt {
    #[doc = "0: normal mode"]
    NormalMode = 0,
    #[doc = "1: Disable write capability of registers SEC900 - SEC93C ."]
    DisableWriteCapabilityOfRegistersSec900_Sec93c_ = 1,
}
impl From<VaultKeyWrProt> for bool {
    #[inline(always)]
    fn from(variant: VaultKeyWrProt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VaultKeyWrProt` reader - Vault Key Write Protection"]
pub type VaultKeyWrProtR = crate::BitReader<VaultKeyWrProt>;
impl VaultKeyWrProtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VaultKeyWrProt {
        match self.bits {
            false => VaultKeyWrProt::NormalMode,
            true => VaultKeyWrProt::DisableWriteCapabilityOfRegistersSec900_Sec93c_,
        }
    }
    #[doc = "normal mode"]
    #[inline(always)]
    pub fn is_normal_mode(&self) -> bool {
        *self == VaultKeyWrProt::NormalMode
    }
    #[doc = "Disable write capability of registers SEC900 - SEC93C ."]
    #[inline(always)]
    pub fn is_disable_write_capability_of_registers_sec900__sec93c_(&self) -> bool {
        *self == VaultKeyWrProt::DisableWriteCapabilityOfRegistersSec900_Sec93c_
    }
}
#[doc = "Field `VaultKeyWrProt` writer - Vault Key Write Protection"]
pub type VaultKeyWrProtW<'a, REG> = crate::BitWriter<'a, REG, VaultKeyWrProt>;
impl<'a, REG> VaultKeyWrProtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "normal mode"]
    #[inline(always)]
    pub fn normal_mode(self) -> &'a mut crate::W<REG> {
        self.variant(VaultKeyWrProt::NormalMode)
    }
    #[doc = "Disable write capability of registers SEC900 - SEC93C ."]
    #[inline(always)]
    pub fn disable_write_capability_of_registers_sec900__sec93c_(self) -> &'a mut crate::W<REG> {
        self.variant(VaultKeyWrProt::DisableWriteCapabilityOfRegistersSec900_Sec93c_)
    }
}
#[doc = "Vault Key Selection Protection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VaultKeySelProt {
    #[doc = "0: normal mode"]
    NormalMode = 0,
    #[doc = "1: Disable write capability of register SEC80C\\[0\\]."]
    DisableWriteCapabilityOfRegisterSec80c0 = 1,
}
impl From<VaultKeySelProt> for bool {
    #[inline(always)]
    fn from(variant: VaultKeySelProt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VaultKeySelProt` reader - Vault Key Selection Protection"]
pub type VaultKeySelProtR = crate::BitReader<VaultKeySelProt>;
impl VaultKeySelProtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VaultKeySelProt {
        match self.bits {
            false => VaultKeySelProt::NormalMode,
            true => VaultKeySelProt::DisableWriteCapabilityOfRegisterSec80c0,
        }
    }
    #[doc = "normal mode"]
    #[inline(always)]
    pub fn is_normal_mode(&self) -> bool {
        *self == VaultKeySelProt::NormalMode
    }
    #[doc = "Disable write capability of register SEC80C\\[0\\]."]
    #[inline(always)]
    pub fn is_disable_write_capability_of_register_sec80c0(&self) -> bool {
        *self == VaultKeySelProt::DisableWriteCapabilityOfRegisterSec80c0
    }
}
#[doc = "Field `VaultKeySelProt` writer - Vault Key Selection Protection"]
pub type VaultKeySelProtW<'a, REG> = crate::BitWriter<'a, REG, VaultKeySelProt>;
impl<'a, REG> VaultKeySelProtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "normal mode"]
    #[inline(always)]
    pub fn normal_mode(self) -> &'a mut crate::W<REG> {
        self.variant(VaultKeySelProt::NormalMode)
    }
    #[doc = "Disable write capability of register SEC80C\\[0\\]."]
    #[inline(always)]
    pub fn disable_write_capability_of_register_sec80c0(self) -> &'a mut crate::W<REG> {
        self.variant(VaultKeySelProt::DisableWriteCapabilityOfRegisterSec80c0)
    }
}
#[doc = "Field `Reserved0` reader - Reserved(0)"]
pub type Reserved0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - Vault Key Selection"]
    #[inline(always)]
    pub fn vault_key_sel(&self) -> VaultKeySelR {
        VaultKeySelR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Vault Key Write Protection"]
    #[inline(always)]
    pub fn vault_key_wr_prot(&self) -> VaultKeyWrProtR {
        VaultKeyWrProtR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Vault Key Selection Protection"]
    #[inline(always)]
    pub fn vault_key_sel_prot(&self) -> VaultKeySelProtR {
        VaultKeySelProtR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:31 - Reserved(0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits >> 3) & 0x1fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - Vault Key Selection"]
    #[inline(always)]
    pub fn vault_key_sel(&mut self) -> VaultKeySelW<Secure80cSpec> {
        VaultKeySelW::new(self, 0)
    }
    #[doc = "Bit 1 - Vault Key Write Protection"]
    #[inline(always)]
    pub fn vault_key_wr_prot(&mut self) -> VaultKeyWrProtW<Secure80cSpec> {
        VaultKeyWrProtW::new(self, 1)
    }
    #[doc = "Bit 2 - Vault Key Selection Protection"]
    #[inline(always)]
    pub fn vault_key_sel_prot(&mut self) -> VaultKeySelProtW<Secure80cSpec> {
        VaultKeySelProtW::new(self, 2)
    }
}
#[doc = "Secure Boot Vault Key Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`secure80c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure80c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Secure80cSpec;
impl crate::RegisterSpec for Secure80cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`secure80c::R`](R) reader structure"]
impl crate::Readable for Secure80cSpec {}
#[doc = "`write(|w| ..)` method takes [`secure80c::W`](W) writer structure"]
impl crate::Writable for Secure80cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SECURE80C to value 0"]
impl crate::Resettable for Secure80cSpec {}
