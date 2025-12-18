use itertools::Itertools;
use std::fs;

fn main() {
    let x = fs::read_to_string("/home/juusop/dev/adventofcode/2025/src/bin/3.test")
        .expect("Should have been able to read file");

    let mut total: u64 = 0;

    let k = 12;

    for line in x.lines().filter(|s| !s.is_empty()) {
        let digits: Vec<u32> = line.chars().filter_map(|d| d.to_digit(10)).collect();

        for subset in digits.into_iter().combinations(k) {
            if let Some(max_val) = subset.into_iter().max() {
                total += max_val as u64;
            }
        }
    }

    dbg!(total);
}
