use criterion::{criterion_main};
use holomorphic::simulation::benches_simulation;

pub mod holomorphic;

criterion_main!(benches_simulation);
