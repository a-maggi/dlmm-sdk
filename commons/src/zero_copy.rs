use crate::dlmm::types::{Bin, ProtocolFee, RewardInfo, StaticParameters, VariableParameters};
use bytemuck::{bytes_of, pod_read_unaligned, Pod, Zeroable};
use solana_sdk::pubkey::Pubkey;
use std::io::{Result as IoResult, Write};

const DISC: usize = 8;

#[inline(always)]
pub fn read_unchecked<T: Pod>(buf: &[u8]) -> T {
    // Assumes `buf.len() >= DISC + size_of::<T>()` and includes the 8-byte discriminator prefix
    pod_read_unaligned::<T>(&buf[DISC..DISC + core::mem::size_of::<T>()])
}

pub const BIN_ARRAY_BITMAP_EXTENSION_ACCOUNT_DISCM: [u8; 8] = [80, 111, 124, 113, 55, 237, 18, 5];

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct BinArrayBitmapExtension {
    pub lb_pair: Pubkey,
    pub positive_bin_array_bitmap: [[u64; 8]; 12],
    pub negative_bin_array_bitmap: [[u64; 8]; 12],
}

#[derive(Clone, Debug)]
pub struct BinArrayBitmapExtensionAccount(pub BinArrayBitmapExtension);
impl BinArrayBitmapExtensionAccount {
    #[inline(always)]
    pub fn deserialize(buf: &[u8]) -> Self {
        Self(read_unchecked(buf))
    }

    pub fn serialize<W: Write>(&self, mut writer: W) -> IoResult<()> {
        writer.write_all(&BIN_ARRAY_BITMAP_EXTENSION_ACCOUNT_DISCM)?;
        writer.write_all(bytes_of(&self.0))
    }

    pub fn try_to_vec(&self) -> IoResult<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}

pub const BIN_ARRAY_ACCOUNT_DISCM: [u8; 8] = [92, 142, 92, 220, 5, 148, 70, 181];

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct BinArray {
    pub index: i64,
    pub version: u8,
    pub _padding: [u8; 7],
    pub lb_pair: Pubkey,
    pub bins: [Bin; 70],
}

#[derive(Clone, Debug)]
pub struct BinArrayAccount(pub BinArray);
impl BinArrayAccount {
    #[inline(always)]
    pub fn deserialize(buf: &[u8]) -> Self {
        Self(read_unchecked(buf))
    }

    pub fn serialize<W: Write>(&self, mut writer: W) -> IoResult<()> {
        writer.write_all(&BIN_ARRAY_ACCOUNT_DISCM)?;
        writer.write_all(bytes_of(&self.0))
    }

    pub fn try_to_vec(&self) -> IoResult<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}

pub const LB_PAIR_ACCOUNT_DISCM: [u8; 8] = [33, 11, 49, 98, 181, 101, 177, 13];

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct LbPair {
    pub parameters: StaticParameters,
    pub v_parameters: VariableParameters,
    pub bump_seed: [u8; 1],
    pub bin_step_seed: [u8; 2],
    pub pair_type: u8,
    pub active_id: i32,
    pub bin_step: u16,
    pub status: u8,
    pub require_base_factor_seed: u8,
    pub base_factor_seed: [u8; 2],
    pub activation_type: u8,
    pub creator_pool_on_off_control: u8,
    pub token_x_mint: Pubkey,
    pub token_y_mint: Pubkey,
    pub reserve_x: Pubkey,
    pub reserve_y: Pubkey,
    pub protocol_fee: ProtocolFee,
    pub _padding_1: [u8; 32],
    pub reward_infos: [RewardInfo; 2],
    pub oracle: Pubkey,
    pub bin_array_bitmap: [u64; 16],
    pub last_updated_at: i64,
    pub _padding_2: [u8; 32],
    pub pre_activation_swap_address: Pubkey,
    pub base_key: Pubkey,
    pub activation_point: u64,
    pub pre_activation_duration: u64,
    pub _padding_3: [u8; 8],
    pub _padding_4: u64,
    pub creator: Pubkey,
    pub token_mint_x_program_flag: u8,
    pub token_mint_y_program_flag: u8,
    pub _reserved: [u8; 22],
}

#[derive(Clone, Debug)]
pub struct LbPairAccount(pub LbPair);
impl LbPairAccount {
    pub const LEN: usize = 904;

    #[inline(always)]
    pub fn deserialize(buf: &[u8]) -> Self {
        Self(read_unchecked(buf))
    }

    pub fn serialize<W: Write>(&self, mut writer: W) -> IoResult<()> {
        writer.write_all(&LB_PAIR_ACCOUNT_DISCM)?;
        writer.write_all(bytes_of(&self.0))
    }

    pub fn try_to_vec(&self) -> IoResult<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
