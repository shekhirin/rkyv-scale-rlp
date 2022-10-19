use bytes::Bytes;

use rkyv::ser::{ScratchSpace, Serializer};
use rkyv::vec::{ArchivedVec, RawArchivedVec, VecResolver};
use rkyv::with::{ArchiveWith, SerializeWith};
use rkyv::{Archive, Serialize};
use ruint::aliases::U256;

use zstd::zstd_safe::WriteBuf;

pub struct U256Wrapper;

impl ArchiveWith<U256> for U256Wrapper {
    type Archived = [<u64 as Archive>::Archived; 4];
    type Resolver = [<u64 as Archive>::Resolver; 4];

    unsafe fn resolve_with(
        field: &U256,
        pos: usize,
        resolver: Self::Resolver,
        out: *mut Self::Archived,
    ) {
        crate::impls::Uint::<256, 4>(*field).resolve(pos, resolver, out)
    }
}

impl<S: Serializer> SerializeWith<U256, S> for U256Wrapper {
    #[inline]
    fn serialize_with(field: &U256, serializer: &mut S) -> Result<Self::Resolver, S::Error> {
        crate::impls::Uint::<256, 4>(*field).serialize(serializer)
    }
}

pub struct BytesWrapper;

impl ArchiveWith<Bytes> for BytesWrapper {
    type Archived = RawArchivedVec<<u8 as Archive>::Archived>;
    type Resolver = VecResolver;

    unsafe fn resolve_with(
        field: &Bytes,
        pos: usize,
        resolver: Self::Resolver,
        out: *mut Self::Archived,
    ) {
        RawArchivedVec::resolve_from_slice(field.as_slice(), pos, resolver, out);
    }
}

impl<S: Serializer + ScratchSpace> SerializeWith<Bytes, S> for BytesWrapper {
    #[inline]
    fn serialize_with(field: &Bytes, serializer: &mut S) -> Result<Self::Resolver, S::Error> {
        unsafe { ArchivedVec::serialize_copy_from_slice(field.as_slice(), serializer) }
    }
}
