use criterion::criterion_main;

mod day_01;
mod day_02;

criterion_main! {
    day_01::benches,
    day_02::benches,
}
