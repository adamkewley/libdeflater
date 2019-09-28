#[macro_use]
extern crate criterion;

mod custom_benches;

criterion_group!(benches, custom_benches::run_custom_benches);
criterion_main!(benches);
