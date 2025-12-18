use std::fs;

fn main() {
    let x = fs::read_to_string("/home/juusop/dev/adventofcode/2025/src/bin/3.txt")
        .expect("Should have been able to read file");

    let mut total: u64 = 0;

    for line in x.lines().filter(|s| !s.is_empty()) {
        let digits: Vec<u32> = line.chars().filter_map(|d| d.to_digit(10)).collect();

        let mut largest = 0;

        for i in 0..digits.len() {
            for j in (i + 1)..digits.len() {
                let jolt = digits[i] * 10 + digits[j];
                if jolt > largest {
                    largest = jolt;
                }
            }
        }

        total += largest as u64;
    }

    dbg!(total);
}
