#[macro_use]
extern crate criterion;

mod custom_benches;

criterion_group!(benches, custom_benches::run_compression_benches, custom_benches::run_decompression_benches);
criterion_main!(benches);
