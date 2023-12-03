use std::fs;
use std::collections::HashMap;

fn main() {
    
    let input: Vec<String> = fs::read_to_string("i3.txt").unwrap().lines().map(String::from).collect();
    let width = input[0].len();
    let height = input.len();

    let safe_input = [
        &[".".repeat(width + 2)],
        &input.into_iter().map(
            |line| ".".to_owned() + &line + "."
        ).collect::<Vec<String>>()[..],
        &[".".repeat(width + 2)],
    ].concat();

    let mut star_numbers: HashMap<(usize, usize), Vec<u32>> = HashMap::new();

    let mut previous_was_digit = false;
    let mut num = 0;
    let mut stars = Vec::new();
    for i in 1..height + 1 {
        for j in 1..width + 1 {
            let ch = safe_input[i].as_bytes()[j] as char;
            if ch.is_digit(10) {
                num = num * 10 + ch.to_digit(10).unwrap();
                let mut low = -1;
                if previous_was_digit {
                    low = 1
                }
                for i2 in -1..2 {
                    for j2 in low..2 {
                        let y = ((i as i32) + i2) as usize;
                        let x = ((j as i32) + j2) as usize;
                        let neighbor = safe_input[y].as_bytes()[x] as char;
                        if !neighbor.is_digit(10) && neighbor == '*' {
                            stars.push((y, x))
                        }
                    }
                }
                previous_was_digit = true;
            } else {
                if previous_was_digit {
                    for star in stars {
                        star_numbers.entry(star).or_insert(Vec::new()).push(num);
                    }
                    stars = Vec::new();
                }
                num = 0;
                previous_was_digit = false;
            }
        }
    }

    let mut sum = 0;
    for (_star, nums) in star_numbers.into_iter() {
        if nums.len() == 2 {
            sum += nums[0] * nums[1];
        }
    }
    println!("{}", sum);
}
