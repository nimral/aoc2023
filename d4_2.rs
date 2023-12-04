use std::fs;
use std::collections::HashSet;
use std::collections::HashMap;

fn main() {
    let mut sum = 0;
    
    let mut won_cards: HashMap::<usize, usize> = HashMap::new();

    for (i, line) in fs::read_to_string("i4.txt").unwrap().lines().enumerate() {
        let (first_part, nums_we_have_str) = line.split_once("|").unwrap();
        let (_, winning_nums_str) = first_part.split_once(":").unwrap();
        let winning_nums: HashSet<&str> = winning_nums_str.split_whitespace().collect();
        let nums_we_have: HashSet<&str> = nums_we_have_str.split_whitespace().collect();
        let num_matches = winning_nums.intersection(&nums_we_have).count();

        let mut num_won = 1;
        if won_cards.contains_key(&i) {
            num_won += won_cards.get(&i).unwrap();
        }
        sum += num_won;

        for j in i+1..i+num_matches+1 {
            *won_cards.entry(j).or_default() += num_won;
        }
    }
    println!("{sum}");
}
