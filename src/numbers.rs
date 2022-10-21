macro_rules! hash {
    ($name:ident, $n_bytes:expr) => {
        #[derive(
            Clone,
            parity_scale_codec::Encode,
            parity_scale_codec::Decode,
            fastrlp::RlpEncodable,
            fastrlp::RlpDecodable,
            fastrlp::RlpMaxEncodedLen,
        )]
        pub struct $name(pub ethereum_types::$name);

        impl rkyv::Archive for $name {
            type Archived = rkyv::Archived<[u8; $n_bytes]>;
            type Resolver = rkyv::Resolver<[u8; $n_bytes]>;

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

        impl<D: rkyv::Fallible + ?Sized> rkyv::Deserialize<$name, D>
            for rkyv::Archived<[u8; $n_bytes]>
        {
            fn deserialize(&self, deserializer: &mut D) -> Result<$name, D::Error> {
                Ok($name(ethereum_types::$name(
                    self.deserialize(deserializer)?,
                )))
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
    type Archived = rkyv::Archived<[u64; LIMBS]>;
    type Resolver = rkyv::Resolver<[u64; LIMBS]>;

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

impl<const BITS: usize, const LIMBS: usize, D: rkyv::Fallible + ?Sized>
    rkyv::Deserialize<Uint<BITS, LIMBS>, D> for rkyv::Archived<[u64; LIMBS]>
{
    fn deserialize(&self, deserializer: &mut D) -> Result<Uint<BITS, LIMBS>, D::Error> {
        Ok(Uint(ruint::Uint::<BITS, LIMBS>::from_limbs(
            self.deserialize(deserializer)?,
        )))
    }
}
