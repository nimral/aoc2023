use std::fs;
use std::collections::HashMap;


#[derive(Default, Debug)]
struct Rule {
    feature: char,
    threshold: isize,
    comp: char,
    jump: String,
}


fn main() {
    let binding = fs::read_to_string("i19.txt").unwrap();
    let (workflows_str, ratings_str) = binding.split_once("\n\n").unwrap();

    let workflows: HashMap::<String, Vec<Rule>> = workflows_str.lines().map(
        |line| {
            let (name, rest) = line.split_once("{").unwrap();
            let (rules_str, _) = rest.split_once("}").unwrap();
            let rules = rules_str.split(",").map(
                |rs| {
                    let mut rule = Rule{
                        jump: rs.to_string(),
                        ..Default::default() 
                    };
                    for comp in ['<', '>'] {
                        if rs.contains(comp) {
                            let (feature, threshold_jump_str) = rs.split_once(comp).unwrap();
                            let (threshold_str, jump) =
                                threshold_jump_str.split_once(":").unwrap();
                            rule = Rule{
                                feature: feature.chars().next().unwrap(),
                                threshold: threshold_str.parse().unwrap(),
                                comp: comp, 
                                jump: jump.to_string(),
                            };
                            break;
                        } 
                    }
                    rule
                }
            ).collect();
            (name.to_string(), rules)
        }
    ).collect();


    let mut sum = 0;
    ratings_str.lines().for_each(
        |rs| {
            let rating: HashMap<char, isize> = rs.replace("}", "")
                .replace("{", "").split(",").map(
                    |fs| {
                        let (feature, value) = fs.split_once("=").unwrap();
                        (feature.chars().next().unwrap(), value.parse().unwrap())
                    }
                ).collect();

            let mut state = "in".to_string();
            while !["A".to_string(), "R".to_string()].contains(&state) {
                let workflow = workflows.get(&state).unwrap();
                for rule in workflow {
                    if rule.comp == '<' {
                        if *rating.get(&rule.feature).unwrap() < rule.threshold {
                            state = rule.jump.clone();
                            break;
                        }
                    } else if rule.comp == '>' {
                        if *rating.get(&rule.feature).unwrap() > rule.threshold {
                            state = rule.jump.clone();
                            break;
                        }
                    } else {
                        state = rule.jump.clone();
                        break;
                    }
                }
            }
            if state == "A" {
                sum += rating.values().sum::<isize>();
            }
        }
    );
    println!("{sum}");
}
