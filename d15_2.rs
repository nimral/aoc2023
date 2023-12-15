use std::fs;
use std::collections::HashMap;

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

    let mut boxes = vec![HashMap::<String, (usize, usize)>::new(); 256];
    for (i, step) in steps.iter().enumerate() {
        if step.contains("=") {
            let (label, focal_length) = step.split_once("=").unwrap(); 
            let step_i = match boxes[hash(label.to_string())].get(label) {
                Some((x, _)) => *x,
                None => i
            };
            boxes[hash(label.to_string())].insert(
                label.to_string(),
                (step_i, focal_length.parse().unwrap())
            );
        } else {
            let label = step.replace("-", "");
            boxes[hash(label.to_string())].remove(&label);
        }
    }
    
    let mut sum = 0;
    for (box_i, map) in boxes.iter().enumerate() {
        let mut v: Vec<(usize, usize)> = map.values().cloned().collect();
        v.sort();
        sum += v.iter().enumerate().map(
            |(i, (_, focal_length))| (box_i + 1) * (i + 1) * focal_length
        ).sum::<usize>();
    }
    println!("{sum}");
}
