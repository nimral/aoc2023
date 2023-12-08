use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;

pub fn lcm(nums: &[usize]) -> usize {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}

fn main () {
    
    let mut edges = HashMap::<String, [String; 2]>::new();
    let binding = fs::read_to_string("i8.txt").unwrap();
    let mut input_lines = binding.lines();
    let instructions = input_lines.next().unwrap().trim();
    input_lines.next();
    let mut positions = Vec::<String>::new();
    for line in input_lines {        
        let binding = line.replace("=", "").replace("(", "").replace(")", "").replace(",", "");
        let nodes: Vec<&str> = binding.split_whitespace().collect();
        edges.insert(nodes[0].to_string(), [nodes[1].to_string(), nodes[2].to_string()]);
        if nodes[0].as_bytes()[2] as char == 'A' {
            positions.push(nodes[0].to_string());
        }
    }

    let mut ending_num_steps = Vec::<usize>::new();
    for orig_pos in positions.iter() {
        let mut pos = orig_pos;
        let mut num_steps: usize = 0;
        let mut visited = HashSet::<(String, usize)>::new();
        while !visited.contains(&(pos.to_string(), num_steps % instructions.len())) {
            visited.insert((pos.to_string(), num_steps % instructions.len()));
            if pos.as_bytes()[2] as char == 'Z' {
                ending_num_steps.push(num_steps);
            }
            let index = (instructions.as_bytes()[num_steps % instructions.len()] as char == 'R') as usize;
            pos = &edges.get(pos).unwrap()[index];
            num_steps += 1;
        }
        // turns out there is only one end pos for each
    }
    println!("{}", lcm(&ending_num_steps));
}
