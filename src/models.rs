use fastrlp::{MaxEncodedLen, MaxEncodedLenAssoc};

macro_rules! impl_rkyv {
    ($name:ident, $base_ty:ident, $len:expr) => {
        impl rkyv::Archive for $name {
            type Archived = [<$base_ty as rkyv::Archive>::Archived; $len];
            type Resolver = [<$base_ty as rkyv::Archive>::Resolver; $len];

            unsafe fn resolve(&self, pos: usize, resolver: Self::Resolver, out: *mut Self::Archived) {
                self.0.0.resolve(pos, resolver, out)
            }
        }

        impl<S: rkyv::ser::Serializer> rkyv::Serialize<S> for $name {
            fn serialize(&self, serializer: &mut S) -> Result<Self::Resolver, S::Error> {
                self.0.0.serialize(serializer)
            }
        }
    }
}

macro_rules! impl_hash {
    ($name:ident, $n_bytes:expr) => {
        #[derive(
            parity_scale_codec::Encode, parity_scale_codec::Decode,
            fastrlp::RlpEncodable, fastrlp::RlpDecodable, fastrlp::RlpMaxEncodedLen,
        )]
        pub struct $name(pub ethereum_types::$name);

        impl_rkyv!($name, u8, $n_bytes);
    };
}

impl_hash!(H64, 8);
impl_hash!(H160, 20);
impl_hash!(H256, 32);
impl_hash!(Bloom, 256);

#[derive(
    parity_scale_codec::Encode, parity_scale_codec::Decode,
    fastrlp::RlpEncodable, fastrlp::RlpDecodable,
)]
pub struct U256(pub ethereum_types::U256);

fastrlp::impl_max_encoded_len!(U256, { fastrlp::length_of_length(32) + 32 });

impl_rkyv!(U256, u64, 4);

#[derive(
    rkyv::Archive,
    rkyv::Serialize,
    rkyv::Deserialize,
    parity_scale_codec::Encode,
    parity_scale_codec::Decode,
    fastrlp::RlpEncodable, fastrlp::RlpDecodable, fastrlp::RlpMaxEncodedLen,
)]
pub struct BlockHeader {
    pub parent_hash: H256,
    pub ommers_hash: H256,
    pub beneficiary: H160,
    pub state_root: H256,
    pub transactions_root: H256,
    pub receipts_root: H256,
    pub logs_bloom: Bloom,
    pub difficulty: U256,
    pub number: u64,
    pub gas_limit: u64,
    pub gas_used: u64,
    pub timestamp: u64,
    // pub extra_data: bytes::Bytes,
    pub mix_hash: H256,
    pub nonce: H64,
    pub base_fee_per_gas: U256,
}

#[derive(
    rkyv::Archive, rkyv::Serialize, rkyv::Deserialize,
    parity_scale_codec::Encode, parity_scale_codec::Decode,
    fastrlp::RlpEncodable, fastrlp::RlpDecodable, fastrlp::RlpMaxEncodedLen,
)]
pub struct Block {
    pub header: BlockHeader,
}
