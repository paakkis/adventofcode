fn main() {
    use std::io::stdin;
    let mut s = String::new();
    println!("Enter puzzle input");
    stdin().read_line(&mut s).expect("Incorrect string");
    if let Some('\n') = s.chars().next_back() {
        s.pop();
    }
    if let Some('\r') = s.chars().next_back() {
        s.pop();
    }

    let split = s.split(',');
    let mut total = 0;

    for x in split {
        let mut bounds = x.split('-');
        let lower_b: i64 = bounds.next().unwrap().parse().unwrap();
        let upper_b: i64 = bounds.next().unwrap().parse().unwrap();
        // println!("Bounds = {} - {}", lower_b, upper_b);

        for n in lower_b..upper_b + 1 {
            // println!("n = {}", n);
            let s = n.to_string();
            let len = s.len();
            for k in 2..=len {
                if len % k != 0 {
                    continue;
                }

                let chunk_size = len / k;

                let parts: Vec<&str> = s
                    .as_bytes()
                    .chunks(chunk_size)
                    .map(|c| std::str::from_utf8(c).unwrap())
                    .collect();

                let first = parts[0];
                if parts.iter().all(|&item| item == first) {
                    println!("Found = {}", n);
                    total = total + n;
                    break;
                }
            }
        }
    }
    println!("Total is {}", total);
}
