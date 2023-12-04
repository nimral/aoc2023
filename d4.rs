use std::fs;
use std::collections::HashSet;

fn main() {
    let mut sum = 0;
    for line in fs::read_to_string("i4.txt").unwrap().lines() {
        let (first_part, nums_we_have_str) = line.split_once("|").unwrap();
        let (_, winning_nums_str) = first_part.split_once(":").unwrap();
        let winning_nums: HashSet<&str> = winning_nums_str.split_whitespace().collect();
        let nums_we_have: HashSet<&str> = nums_we_have_str.split_whitespace().collect();
        let num_matches = winning_nums.intersection(&nums_we_have).count();
        if num_matches > 0 {
            sum += usize::pow(2, num_matches as u32 - 1)
        }
    }
    println!("{sum}");
}
