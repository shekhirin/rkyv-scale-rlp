use criterion::{criterion_group, Criterion};
use rkyv_scale_rlp::{compress, rkyv_serialize, rlp_serialize, scale_serialize, BLOCK, MESSAGE};

fn bench_block(c: &mut Criterion) {
    c.benchmark_group("serialize/block")
        .bench_function("rlp", |b| b.iter(|| rlp_serialize(BLOCK.clone())))
        .bench_function("scale", |b| b.iter(|| scale_serialize(BLOCK.clone())))
        .bench_function("rkyv", |b| b.iter(|| rkyv_serialize(BLOCK.clone())));
}

fn bench_block_compress(c: &mut Criterion) {
    c.benchmark_group("serialize/block/compress")
        .bench_function("rlp", |b| b.iter(|| compress(rlp_serialize(BLOCK.clone()))))
        .bench_function("scale", |b| {
            b.iter(|| compress(scale_serialize(BLOCK.clone())))
        })
        .bench_function("rkyv", |b| {
            b.iter(|| compress(rkyv_serialize(BLOCK.clone())))
        });
}

fn bench_message(c: &mut Criterion) {
    c.benchmark_group("serialize/message")
        .bench_function("rlp", |b| b.iter(|| rlp_serialize(MESSAGE.clone())))
        .bench_function("scale", |b| b.iter(|| scale_serialize(MESSAGE.clone())))
        .bench_function("rkyv", |b| b.iter(|| rkyv_serialize(MESSAGE.clone())));
}

fn bench_message_compress(c: &mut Criterion) {
    c.benchmark_group("serialize/message/compress")
        .bench_function("rlp", |b| {
            b.iter(|| compress(rlp_serialize(MESSAGE.clone())))
        })
        .bench_function("scale", |b| {
            b.iter(|| compress(scale_serialize(MESSAGE.clone())))
        })
        .bench_function("rkyv", |b| {
            b.iter(|| compress(rkyv_serialize(MESSAGE.clone())))
        });
}

criterion_group!(
    bench,
    bench_block,
    bench_block_compress,
    bench_message,
    bench_message_compress
);
