use std::fs;
use std::collections::HashMap;

// 56 minutes on Core 2 Duo


#[derive(Default, Debug)]
struct Rule {
    feature: char,
    threshold: isize,
    comp: char,
    jump: String,
}


fn main() {
    let binding = fs::read_to_string("i19.txt").unwrap();
    let (workflows_str, _ratings_str) = binding.split_once("\n\n").unwrap();

    let mut feature_splits = HashMap::<char, Vec<isize>>::new();

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
                    if rule.comp == '<' {
                        feature_splits.entry(rule.feature).or_default().push(
                            rule.threshold
                        );
                    } else if rule.comp == '>' {
                        feature_splits.entry(rule.feature).or_default().push(
                            rule.threshold + 1
                        );
                    }
                    rule
                }
            ).collect();
            (name.to_string(), rules)
        }
    ).collect();


    for (_feature, splits) in &mut feature_splits {
        splits.push(1);
        splits.push(4001);
        splits.sort();
        splits.dedup();
    }

    let a_splits = feature_splits.get(&'a').unwrap().clone();
    let m_splits = feature_splits.get(&'m').unwrap().clone();
    let s_splits = feature_splits.get(&'s').unwrap().clone();
    let x_splits = feature_splits.get(&'x').unwrap().clone();

    let mut sum = 0;
    let mut rating = HashMap::<char, isize>::new();
    for ai in 0..a_splits.len() - 1 {
        rating.insert('a', a_splits[ai]);
        for mi in 0..m_splits.len() - 1 {
            rating.insert('m', m_splits[mi]);
            for si in 0..s_splits.len() - 1 {
                rating.insert('s', s_splits[si]);
                for xi in 0..x_splits.len() - 1 {
                    rating.insert('x', x_splits[xi]);

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
                        sum += 
                            (a_splits[ai + 1] - a_splits[ai]) *
                            (m_splits[mi + 1] - m_splits[mi]) *
                            (s_splits[si + 1] - s_splits[si]) *
                            (x_splits[xi + 1] - x_splits[xi])
                        ;
                    }
                }
            }
        }
    }
    println!("{sum}");
}
