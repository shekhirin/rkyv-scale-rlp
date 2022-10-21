use crate::numbers::{Bloom, H160, H256, H64};
use crate::wrappers::{BytesWrapper, U256Wrapper};
use bytes::Bytes;
use ruint::aliases::U256;

#[derive(
    Clone,
    rkyv::Archive,
    rkyv::Serialize,
    rkyv::Deserialize,
    parity_scale_codec::Encode,
    parity_scale_codec::Decode,
    fastrlp::RlpEncodable,
    fastrlp::RlpDecodable,
)]
pub struct BlockHeader {
    pub parent_hash: H256,
    pub ommers_hash: H256,
    pub beneficiary: H160,
    pub state_root: H256,
    pub transactions_root: H256,
    pub receipts_root: H256,
    pub logs_bloom: Bloom,
    #[codec(compact)]
    #[with(U256Wrapper)]
    pub difficulty: U256,
    pub number: u64,
    pub gas_limit: u64,
    pub gas_used: u64,
    pub timestamp: u64,
    #[with(BytesWrapper)]
    pub extra_data: Bytes,
    pub mix_hash: H256,
    pub nonce: H64,
    #[codec(compact)]
    #[with(U256Wrapper)]
    pub base_fee_per_gas: U256,
}

#[derive(
    Clone,
    rkyv::Archive,
    rkyv::Serialize,
    rkyv::Deserialize,
    parity_scale_codec::Encode,
    parity_scale_codec::Decode,
    fastrlp::RlpEncodable,
    fastrlp::RlpDecodable,
)]
pub struct Block {
    pub header: BlockHeader,
}
