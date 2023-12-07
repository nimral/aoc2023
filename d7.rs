use std::fs;
use std::collections::HashMap;

struct Hand {
    card_values: Vec<usize>,
    bid: usize,
    poker_hand_type: usize,
}


fn build_hand(s: &str, card_order: &HashMap<char, usize>) -> Hand {
    let x = s.split_once(" ").unwrap();
    let cvs = x.0.chars().map(|x| card_order[&x]).collect::<Vec<usize>>();
    let mut sorted_cvs = cvs.clone();
    sorted_cvs.sort();
    let mut uniq_sorted_cvs = sorted_cvs.clone();
    uniq_sorted_cvs.dedup();
    let num_uniq = uniq_sorted_cvs.len();
    
    // high card
    let mut poker_hand_type = 0;
    if sorted_cvs[0] == sorted_cvs[4] {
        // five of a kind
        poker_hand_type = 6;
    } else if sorted_cvs[0] == sorted_cvs[3] || sorted_cvs[1] == sorted_cvs[4] {
        // four of a kind
        poker_hand_type = 5;
    } else if num_uniq == 2 {
        // full house
        poker_hand_type = 4;
    } else if num_uniq == 3 {
        if sorted_cvs[0] == sorted_cvs[2] || sorted_cvs[1] == sorted_cvs[3] || sorted_cvs[2] == sorted_cvs[4] {
            // three of a kind
            poker_hand_type = 3;
        } else {
            // two pair
            poker_hand_type = 2;
        }
    } else if num_uniq == 4 {
        // one pair
        poker_hand_type = 1;
    }

    Hand {
        card_values: cvs,
        bid: x.1.parse().unwrap(),
        poker_hand_type: poker_hand_type,
    }
}


fn main() {
    let card_order: HashMap<char, usize> = HashMap::from(
        ['A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2']
            .iter().rev().enumerate().map(|(a, b)| (*b, a)).collect::<HashMap<char, usize>>()
    );
    let mut hands: Vec<Hand> = fs::read_to_string("i7.txt").unwrap().lines().map(|x| build_hand(x, &card_order)).collect();
    hands.sort_by_key(|x| (x.poker_hand_type, x.card_values.clone()));

    let mut sum = 0;
    for (i, hand) in hands.iter().enumerate() {
        sum += hand.bid * (i + 1);
    }

    println!("{}", sum);
}
