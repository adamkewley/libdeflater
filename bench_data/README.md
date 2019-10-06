# Benchmark Input Data

All files in this directory (apart from this `README.md` file) will be
exercised against flate2 for performance comparisons.

Running `cargo bench` after putting data in here runs the
suite. There's a convenience ruby script at `scripts/*` that processes
the `Criterion` output data into a table.

See `benches/custom_benches.rs` for implementation details.
