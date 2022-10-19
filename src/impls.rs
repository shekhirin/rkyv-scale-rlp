macro_rules! hash {
    ($name:ident, $n_bytes:expr) => {
        #[derive(
            Clone, parity_scale_codec::Encode, fastrlp::RlpEncodable, fastrlp::RlpMaxEncodedLen,
        )]
        pub struct $name(pub ethereum_types::$name);

        impl rkyv::Archive for $name {
            type Archived = [<u8 as rkyv::Archive>::Archived; $n_bytes];
            type Resolver = [<u8 as rkyv::Archive>::Resolver; $n_bytes];

            unsafe fn resolve(
                &self,
                pos: usize,
                resolver: Self::Resolver,
                out: *mut Self::Archived,
            ) {
                self.0 .0.resolve(pos, resolver, out)
            }
        }

        impl<S: rkyv::ser::Serializer> rkyv::Serialize<S> for $name {
            fn serialize(&self, serializer: &mut S) -> Result<Self::Resolver, S::Error> {
                self.0 .0.serialize(serializer)
            }
        }
    };
}

hash!(H64, 8);
hash!(H160, 20);
hash!(H256, 32);
hash!(Bloom, 256);

pub struct Uint<const BITS: usize, const LIMBS: usize>(pub ruint::Uint<BITS, LIMBS>);

impl<const BITS: usize, const LIMBS: usize> rkyv::Archive for Uint<BITS, LIMBS> {
    type Archived = [<u64 as rkyv::Archive>::Archived; LIMBS];
    type Resolver = [<u64 as rkyv::Archive>::Resolver; LIMBS];

    unsafe fn resolve(&self, pos: usize, resolver: Self::Resolver, out: *mut Self::Archived) {
        self.0.as_limbs().resolve(pos, resolver, out)
    }
}

impl<const BITS: usize, const LIMBS: usize, S: rkyv::ser::Serializer> rkyv::Serialize<S>
    for Uint<BITS, LIMBS>
{
    fn serialize(&self, serializer: &mut S) -> Result<Self::Resolver, S::Error> {
        self.0.as_limbs().serialize(serializer)
    }
}
