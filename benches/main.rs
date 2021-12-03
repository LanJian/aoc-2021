use criterion::criterion_main;

mod day_01;
mod day_02;
mod day_03;

criterion_main! {
    day_01::benches,
    day_02::benches,
    day_03::benches,
}
