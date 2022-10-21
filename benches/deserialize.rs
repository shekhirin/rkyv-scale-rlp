use criterion::{criterion_group, Criterion};
use rkyv_scale_rlp::{
    compress, decompress, rkyv_deserialize, rkyv_serialize, rlp_deserialize, rlp_serialize,
    scale_deserialize, scale_serialize, Block, EIP1559MessageWithSignature, BLOCK, MESSAGE,
};

fn bench_block(c: &mut Criterion) {
    c.benchmark_group("deserialize/block")
        .bench_function("rlp", |b| {
            let serialized = rlp_serialize(BLOCK.clone());
            b.iter(|| rlp_deserialize::<Block>(serialized.clone()))
        })
        .bench_function("scale", |b| {
            let serialized = scale_serialize(BLOCK.clone());
            b.iter(|| scale_deserialize::<Block>(serialized.clone()))
        })
        .bench_function("rkyv", |b| {
            let serialized = rkyv_serialize(BLOCK.clone());
            b.iter(|| rkyv_deserialize::<Block>(serialized.clone()))
        });
}

fn bench_block_compress(c: &mut Criterion) {
    c.benchmark_group("deserialize/block/compress")
        .bench_function("rlp", |b| {
            let serialized = rlp_serialize(BLOCK.clone());
            let capacity = serialized.len();
            let compressed = compress(serialized);
            b.iter(|| rlp_deserialize::<Block>(decompress(compressed.clone(), capacity)))
        })
        .bench_function("scale", |b| {
            let serialized = scale_serialize(BLOCK.clone());
            let capacity = serialized.len();
            let compressed = compress(serialized);
            b.iter(|| scale_deserialize::<Block>(decompress(compressed.clone(), capacity)))
        })
        .bench_function("rkyv", |b| {
            let serialized = rkyv_serialize(BLOCK.clone());
            let capacity = serialized.len();
            let compressed = compress(serialized);
            b.iter(|| rkyv_deserialize::<Block>(decompress(compressed.clone(), capacity)))
        });
}

fn bench_message(c: &mut Criterion) {
    c.benchmark_group("deserialize/message")
        .bench_function("rlp", |b| {
            let serialized = rlp_serialize(MESSAGE.clone());
            b.iter(|| rlp_deserialize::<EIP1559MessageWithSignature>(serialized.clone()));
        })
        .bench_function("scale", |b| {
            let serialized = scale_serialize(MESSAGE.clone());
            b.iter(|| scale_deserialize::<EIP1559MessageWithSignature>(serialized.clone()));
        })
        .bench_function("rkyv", |b| {
            let serialized = rkyv_serialize(MESSAGE.clone());
            b.iter(|| rkyv_deserialize::<EIP1559MessageWithSignature>(serialized.clone()));
        });
}

fn bench_message_compress(c: &mut Criterion) {
    c.benchmark_group("deserialize/message/compress")
        .bench_function("rlp", |b| {
            let serialized = rlp_serialize(MESSAGE.clone());
            let capacity = serialized.len();
            let compressed = compress(serialized);
            b.iter(|| {
                rlp_deserialize::<EIP1559MessageWithSignature>(decompress(
                    compressed.clone(),
                    capacity,
                ))
            });
        })
        .bench_function("scale", |b| {
            let serialized = scale_serialize(MESSAGE.clone());
            let capacity = serialized.len();
            let compressed = compress(serialized);
            b.iter(|| {
                scale_deserialize::<EIP1559MessageWithSignature>(decompress(
                    compressed.clone(),
                    capacity,
                ))
            });
        })
        .bench_function("rkyv", |b| {
            let serialized = rkyv_serialize(MESSAGE.clone());
            let capacity = serialized.len();
            let compressed = compress(serialized);
            b.iter(|| {
                rkyv_deserialize::<EIP1559MessageWithSignature>(decompress(
                    compressed.clone(),
                    capacity,
                ))
            });
        });
}

criterion_group!(
    bench,
    bench_block,
    bench_block_compress,
    bench_message,
    bench_message_compress
);
