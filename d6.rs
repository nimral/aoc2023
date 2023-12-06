use std::fs;
use std::iter::zip;

fn parse_line(line: &str) -> Vec<usize> {
    return line.split_once(":").unwrap().1.split_whitespace().map(|x| x.parse().unwrap()).collect();
}

fn main() {
    let binding = fs::read_to_string("i6.txt").unwrap();
    let (time_str, dist_str) = binding.split_once("\n").unwrap();
    let times = parse_line(time_str);
    let dists = parse_line(dist_str);

    let mut prod = 1;
    for (time, dist) in zip(times, dists) {
        let speed1 = ((time as f64) - ((time * time - 4 * dist) as f64).sqrt()) / 2.0;
        let speed2 = ((time as f64) + ((time * time - 4 * dist) as f64).sqrt()) / 2.0;
        let low = speed1.min(speed2);
        let high = speed2.max(speed1);
        let mut margin = (high.floor() as isize) - (low.ceil() as isize) + 1;
        if low == low.ceil() {
            margin -= 1;
        }
        if high == high.floor() {
            margin -= 1;
        }
        prod *= margin;
    }
    println!("{}", prod);
}
