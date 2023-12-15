use std::fs;

fn hash(s: String) -> usize {
    let mut current: usize = 0;
    for b in s.as_bytes() {
        current += *b as usize;
        current = (current * 17) % 256;
    }
    return current;
}


fn main() {
    let steps: Vec<String> = fs::read_to_string("i15.txt").unwrap()
        .replace("\n", "").split(",").map(|s| s.to_string()).collect();
    println!("{}", steps.iter().map(|x| hash(x.clone())).sum::<usize>());
}
