use std::fs;

fn main() {
    let digit_names = &[
        "one",
        "two",
        "three",
        "four",
        "five",
        "six",
        "seven",
        "eight",
        "nine",
    ];

    let mut sum = 0;
    for line in fs::read_to_string("i1.txt").unwrap().lines() {
        let mut first_digit = 0;
        let mut last_digit = 0;
        let mut seen_a_digit = false;
        let mut first_pos = 0;
        let mut last_pos = 0;

        for (pos, ch) in line.chars().enumerate() {
            if ch.is_digit(10) {
                let d = ch.to_digit(10).unwrap();
                if !seen_a_digit {
                    first_digit = d;
                    first_pos = pos;
                    seen_a_digit = true;
                }
                last_digit = d;
                last_pos = pos;
            }
        }
        for (i, digit_name) in digit_names.iter().enumerate() {
            let positions: Vec<usize> = line.match_indices(digit_name).map(|(i, _)| i).collect();
            if positions.len() > 0 {
                if positions[0] < first_pos {
                    first_pos = positions[0];
                    first_digit = (i + 1) as u32;
                }
                if *positions.last().unwrap() > last_pos {
                    last_pos = *positions.last().unwrap();
                    last_digit = (i + 1) as u32;
                }
            }

        }
        sum += 10 * first_digit + last_digit;
    }
    println!("{sum}");
}
