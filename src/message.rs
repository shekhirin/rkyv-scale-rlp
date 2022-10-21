use crate::numbers::{H160, H256};
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
pub struct EIP1559MessageWithSignature {
    pub message: EIP1559Message,
    pub signature: MessageSignature,
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
pub struct EIP1559Message {
    #[codec(compact)]
    pub chain_id: u64,
    #[codec(compact)]
    pub nonce: u64,
    #[codec(compact)]
    #[with(U256Wrapper)]
    pub max_priority_fee_per_gas: U256,
    #[codec(compact)]
    #[with(U256Wrapper)]
    pub max_fee_per_gas: U256,
    #[codec(compact)]
    pub gas_limit: u64,
    pub action: H160, // Call(Address)
    #[codec(compact)]
    #[with(U256Wrapper)]
    pub value: U256,
    #[with(BytesWrapper)]
    pub input: Bytes,
    pub access_list: Vec<AccessListItem>,
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
pub struct AccessListItem {
    pub address: H160,
    pub slots: Vec<H256>,
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
pub struct MessageSignature {
    pub odd_y_parity: bool,
    pub r: H256,
    pub s: H256,
}
