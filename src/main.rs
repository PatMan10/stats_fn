use stats_fn::*;

use rand::{thread_rng, Rng};

fn main() {
    let mut data = init_numbers();
    data.sort();

    let mean = mean(&data);
    let median = median(&data);
    let mode = mode(&data);

    println!("{data:?}");
    println!("mean = {mean}");
    println!("median = {median}");
    println!("mode = {mode:?}");
}

fn r(a: u32, b: u32) -> u32 {
    thread_rng().gen_range(a..=b)
}

fn init_numbers() -> Vec<u32> {
    let mut numbers: Vec<u32> = Vec::new();
    let count = r(1, 20);
    for _i in 0..count {
        numbers.push(r(0, 100));
    }
    numbers
}
