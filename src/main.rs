use rand::{thread_rng, Rng};
use std::collections::HashMap;

fn main() {
    let mut numbers = init_numbers();
    numbers.sort();
    let mean = mean(&numbers);
    let median = median(&numbers);
    let mode = mode(&numbers);
    println!("{numbers:?}");
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

fn mean(numbers: &Vec<u32>) -> f32 {
    let mut mean = 0;
    for n in numbers {
        mean += n;
    }
    (mean as f32) / (numbers.len() as f32 * 1.0)
}

fn median(numbers: &Vec<u32>) -> f32 {
    if numbers.len() % 2 != 0 {
        let i = numbers.len() / 2;
        numbers[i] as f32 * 1.0
    } else {
        let i = numbers.len() / 2;
        let a = numbers[i - 1];
        let b = numbers[i];
        ((a + b) as f32) / 2.0
    }
}

fn mode(numbers: &Vec<u32>) -> Vec<u32> {
    let mut map1 = HashMap::new();
    let mut f = 0;
    for n in numbers {
        let count = map1.entry(*n).or_insert(0);
        *count += 1;
        if count > &mut f {
            f = *count;
        }
    }

    let mut mode = Vec::new();
    for (k, v) in map1.iter() {
        if v == &f {
            mode.push(*k);
        }
    }
    mode.sort();
    if mode.len() == 1 {
        return mode;
    }
    if mode.len() == numbers.len() {
        return vec![];
    }
    mode
}
