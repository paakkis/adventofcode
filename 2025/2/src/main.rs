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
            if len % 2 != 0 {
                continue;
            }
            let middle = len / 2;
            let left: String = s[..middle].parse().ok().unwrap();
            let right: String = s[middle..].parse().ok().unwrap();

            // println!("Left = {}, Right {}", left, right);

            if left == right {
                total = total + n;
            }
        }
    }
    println!("Total is {}", total);
}
