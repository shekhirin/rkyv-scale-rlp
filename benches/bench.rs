mod deserialize;
mod serialize;

criterion::criterion_main!(serialize::bench, deserialize::bench);
