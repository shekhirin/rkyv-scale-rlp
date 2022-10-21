use bytes::Bytes;

use rkyv::ser::{ScratchSpace, Serializer};
use rkyv::vec::{ArchivedVec, VecResolver};
use rkyv::with::{ArchiveWith, DeserializeWith, SerializeWith};
use rkyv::{Archive, Archived, Deserialize, Fallible, Resolver, Serialize};
use ruint::aliases::U256;

use zstd::zstd_safe::WriteBuf;

pub struct U256Wrapper;

impl ArchiveWith<U256> for U256Wrapper {
    type Archived = Archived<[u64; 4]>;
    type Resolver = Resolver<[u64; 4]>;

    unsafe fn resolve_with(
        field: &U256,
        pos: usize,
        resolver: Self::Resolver,
        out: *mut Self::Archived,
    ) {
        crate::numbers::Uint::<256, 4>(*field).resolve(pos, resolver, out)
    }
}

impl<S: Serializer> SerializeWith<U256, S> for U256Wrapper {
    fn serialize_with(field: &U256, serializer: &mut S) -> Result<Self::Resolver, S::Error> {
        crate::numbers::Uint::<256, 4>(*field).serialize(serializer)
    }
}

impl<D: Fallible + ?Sized> DeserializeWith<Archived<[u64; 4]>, U256, D> for U256Wrapper {
    fn deserialize_with(
        field: &Archived<[u64; 4]>,
        deserializer: &mut D,
    ) -> Result<U256, D::Error> {
        Ok(U256::from_limbs(field.deserialize(deserializer)?))
    }
}

pub struct BytesWrapper;

impl ArchiveWith<Bytes> for BytesWrapper {
    type Archived = ArchivedVec<Archived<u8>>;
    type Resolver = VecResolver;

    unsafe fn resolve_with(
        field: &Bytes,
        pos: usize,
        resolver: Self::Resolver,
        out: *mut Self::Archived,
    ) {
        ArchivedVec::resolve_from_slice(field.as_slice(), pos, resolver, out);
    }
}

impl<S: Serializer + ScratchSpace> SerializeWith<Bytes, S> for BytesWrapper {
    fn serialize_with(field: &Bytes, serializer: &mut S) -> Result<Self::Resolver, S::Error> {
        unsafe { ArchivedVec::serialize_copy_from_slice(field.as_slice(), serializer) }
    }
}

impl<D: Fallible + ?Sized> DeserializeWith<ArchivedVec<Archived<u8>>, Bytes, D> for BytesWrapper {
    fn deserialize_with(
        field: &ArchivedVec<Archived<u8>>,
        deserializer: &mut D,
    ) -> Result<Bytes, D::Error> {
        let bytes: Vec<_> = field.deserialize(deserializer)?;
        Ok(Bytes::from(bytes))
    }
}
