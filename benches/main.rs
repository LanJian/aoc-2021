use criterion::criterion_main;

mod day_1;
mod day_2;

criterion_main! {
    day_1::benches,
    day_2::benches,
}
