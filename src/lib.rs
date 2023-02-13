use std::collections::HashMap;

pub fn mean(data: &Vec<u32>) -> f32 {
    let mut mean = 0;
    for n in data {
        mean += n;
    }
    (mean as f32) / (data.len() as f32)
}

pub fn median(data: &Vec<u32>) -> f32 {
    if data.len() % 2 != 0 {
        let i = data.len() / 2;
        data[i] as f32
    } else {
        let i = data.len() / 2;
        let a = data[i - 1];
        let b = data[i];
        ((a + b) as f32) / 2.0
    }
}

pub fn mode(data: &Vec<u32>) -> Vec<u32> {
    let mut map1 = HashMap::new();
    let mut f = 0;
    for n in data {
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
    if mode.len() == data.len() {
        return vec![];
    }
    mode
}
