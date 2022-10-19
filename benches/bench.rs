use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rkyv_scale_rlp::{rkyv, rlp, scale, BLOCK, MESSAGE};

fn bench_block(c: &mut Criterion) {
    c.benchmark_group("block")
        .bench_function("rlp", |b| b.iter(|| rlp(black_box(BLOCK.clone()))));

    c.benchmark_group("block")
        .bench_function("scale", |b| b.iter(|| scale(black_box(&BLOCK.clone()))));

    c.benchmark_group("block")
        .bench_function("rkyv", |b| b.iter(|| rkyv(black_box(&BLOCK.clone()))));
}

fn bench_message(c: &mut Criterion) {
    c.benchmark_group("message")
        .bench_function("rlp", |b| b.iter(|| rlp(black_box(MESSAGE.clone()))));

    c.benchmark_group("message")
        .bench_function("scale", |b| b.iter(|| scale(black_box(&MESSAGE.clone()))));

    c.benchmark_group("message")
        .bench_function("rkyv", |b| b.iter(|| rkyv(black_box(&MESSAGE.clone()))));
}

criterion_group!(benches, bench_block, bench_message);
criterion_main!(benches);
