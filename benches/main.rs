use criterion::criterion_main;

mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_08;
mod day_09;
mod day_10;
mod day_11;
mod day_12;
mod day_13;
mod day_14;
mod day_15;
mod day_16;

criterion_main! {
    day_01::benches,
    day_02::benches,
    day_03::benches,
    day_04::benches,
    day_05::benches,
    day_06::benches,
    day_07::benches,
    day_08::benches,
    day_09::benches,
    day_10::benches,
    day_11::benches,
    day_12::benches,
    day_13::benches,
    day_14::benches,
    day_15::benches,
    day_16::benches,
}
