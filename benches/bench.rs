use criterion::{criterion_group, criterion_main, Criterion};
use rkyv_scale_rlp::{compress, rkyv, rlp, scale, BLOCK, MESSAGE};

fn bench_block(c: &mut Criterion) {
    c.benchmark_group("block")
        .bench_function("rlp", |b| b.iter(|| rlp(BLOCK.clone())))
        .bench_function("scale", |b| b.iter(|| scale(BLOCK.clone())))
        .bench_function("rkyv", |b| b.iter(|| rkyv(BLOCK.clone())));
}

fn bench_block_compress(c: &mut Criterion) {
    c.benchmark_group("block/compress")
        .bench_function("rlp", |b| b.iter(|| compress(rlp(BLOCK.clone()))))
        .bench_function("scale", |b| b.iter(|| compress(scale(BLOCK.clone()))))
        .bench_function("rkyv", |b| b.iter(|| compress(rkyv(BLOCK.clone()))));
}

fn bench_message(c: &mut Criterion) {
    c.benchmark_group("message")
        .bench_function("rlp", |b| b.iter(|| rlp(MESSAGE.clone())))
        .bench_function("scale", |b| b.iter(|| scale(MESSAGE.clone())))
        .bench_function("rkyv", |b| b.iter(|| rkyv(MESSAGE.clone())));
}

fn bench_message_compress(c: &mut Criterion) {
    c.benchmark_group("message/compress")
        .bench_function("rlp", |b| b.iter(|| compress(rlp(MESSAGE.clone()))))
        .bench_function("scale", |b| b.iter(|| compress(scale(MESSAGE.clone()))))
        .bench_function("rkyv", |b| b.iter(|| compress(rkyv(MESSAGE.clone()))));
}

criterion_group!(
    benches,
    bench_block,
    bench_block_compress,
    bench_message,
    bench_message_compress
);
criterion_main!(benches);
