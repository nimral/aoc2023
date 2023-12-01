use std::fs;

fn main() {
    let mut sum = 0;
    for line in fs::read_to_string("i1.txt").unwrap().lines() {
        let mut first_digit = 0;
        let mut last_digit = 0;
        let mut seen_a_digit = false;
        for ch in line.chars() {
            if ch.is_digit(10) {
                let d = ch.to_digit(10).unwrap();
                if !seen_a_digit {
                    first_digit = d;
                    seen_a_digit = true;
                }
                last_digit = d;
            }
        }
        sum += 10 * first_digit + last_digit;
    }
    println!("{sum}");
}
