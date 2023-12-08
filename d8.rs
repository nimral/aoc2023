use std::fs;
use std::collections::HashMap;

fn main () {
    
    let mut edges = HashMap::<String, [String; 2]>::new();
    let binding = fs::read_to_string("i8.txt").unwrap();
    let mut input_lines = binding.lines();
    let instructions = input_lines.next().unwrap().trim();
    input_lines.next();
    for line in input_lines {        
        let binding = line.replace("=", "").replace("(", "").replace(")", "").replace(",", "");
        let nodes: Vec<&str> = binding.split_whitespace().collect();
        edges.insert(nodes[0].to_string(), [nodes[1].to_string(), nodes[2].to_string()]);
    }
    let mut pos = "AAA";
    let mut num_steps = 0;
    while pos != "ZZZ" {
        let index = (instructions.as_bytes()[num_steps % instructions.len()] as char == 'R') as usize;
        pos = &edges.get(pos).unwrap()[index];
        num_steps += 1;
    }
    println!("{}", num_steps);
}
