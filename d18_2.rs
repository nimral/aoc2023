use std::fs;
use std::i64;
use std::collections::HashMap;


#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
struct VerticalSegment {
    x: isize,
    y_start: isize,
    y_end: isize,
}


#[derive(Debug, Clone)]
struct Interval {
    y_start: isize,
    y_end: isize,
    top_inside: bool,
    bottom_inside: bool,
}


fn main() {
    let mut pos: [isize; 2] = [0, 0];
    let mut segments: Vec<VerticalSegment> = Vec::new();
    fs::read_to_string("i18.txt").unwrap().lines().for_each(
        |line| {
            let hexa: String = line.split_whitespace().collect::<Vec<_>>()
                .last().unwrap().to_string();
            let step_size: isize = i64::from_str_radix(&hexa[2..7], 16).unwrap() as isize;
            match hexa.as_bytes()[7] as char {
                '3' => {
                    segments.push(
                        VerticalSegment{
                            x: pos[1],
                            y_start: pos[0] - step_size,
                            y_end: pos[0] + 1
                        }
                    );
                    pos[0] -= step_size;
                },
                '1' => {
                    segments.push(
                        VerticalSegment{
                            x: pos[1],
                            y_start: pos[0],
                            y_end: pos[0] + step_size + 1
                        }
                    );
                    pos[0] += step_size
                },
                '2' => {pos[1] -= step_size},
                '0' => {pos[1] += step_size},
                _ => unreachable!(),
            }
        }
    );

    segments.sort();

    let mut events = Vec::<isize>::new();
    segments.iter().for_each(
        |s| {
            events.push(s.y_start);
            events.push(s.y_start + 1);
            events.push(s.y_end - 1);
            events.push(s.y_end);
        }
    );
    events.sort();
    events.dedup();

    let mut intervals = Vec::<Interval>::new();
    let mut y2interval_index = HashMap::<isize, usize>::new();
    for i in 0..events.len() - 1 {
        intervals.push(
            Interval{
                y_start: events[i],
                y_end: events[i + 1],
                top_inside: false,
                bottom_inside: false
            }
        );
        y2interval_index.insert(events[i], i);
    }
    y2interval_index.insert(events[events.len() - 1], events.len() - 1);

    let mut new_intervals = intervals.clone();

    let mut x: isize = segments.iter().min_by_key(
        |segment: &&VerticalSegment| segment.x
    ).unwrap().x;
    let mut area = 0;
    let mut currently_inside = 0;
    for segment in &segments {
        if segment.x > x {
            area += currently_inside * (segment.x - x);
            x = segment.x;
            intervals = new_intervals;
            new_intervals = intervals.clone();
        }
        let from_interval_index = *y2interval_index.get(&segment.y_start).unwrap();
        let until_interval_index = *y2interval_index.get(&segment.y_end).unwrap();

        // would be nice, but https://stackoverflow.com/questions/30073684/how-to-get-mutable-references-to-two-array-elements-at-the-same-time
        // let first = &mut intervals[from_interval_index];
        // let second = &intervals[from_interval_index + 1];
        let first_was_inside = intervals[from_interval_index].top_inside ||
            intervals[from_interval_index].bottom_inside;
        let first_inside = intervals[from_interval_index].top_inside ||
            !intervals[from_interval_index + 1].top_inside;
        let first_length = intervals[from_interval_index].y_end -
            intervals[from_interval_index].y_start;
        if first_was_inside && !first_inside {
            currently_inside -= first_length;
            area += first_length;
        }
        if !first_was_inside && first_inside {
            currently_inside += first_length;
        }
        new_intervals[from_interval_index].bottom_inside = !intervals[from_interval_index + 1].top_inside;

        // let last = &mut intervals[until_interval_index - 1];
        // let second_last = &intervals[until_interval_index - 2];
        let last_was_inside = intervals[until_interval_index - 1].top_inside ||
            intervals[until_interval_index - 1].bottom_inside;
        let last_inside = intervals[until_interval_index - 1].bottom_inside ||
            !intervals[until_interval_index - 2].bottom_inside;
        let last_length = intervals[until_interval_index - 1].y_end -
            intervals[until_interval_index - 1].y_start;
        if last_was_inside && !last_inside {
            currently_inside -= last_length;
            area += last_length;
        }
        if !last_was_inside && last_inside {
            currently_inside += last_length;
        }
        new_intervals[until_interval_index - 1].top_inside = !intervals[until_interval_index - 2].bottom_inside;

        for i in from_interval_index + 1..until_interval_index - 1 {
            let length = intervals[i].y_end - intervals[i].y_start;
            if intervals[i].top_inside {
                currently_inside -= length;
                area += length;
                new_intervals[i].top_inside = false;
                new_intervals[i].bottom_inside = false;
            } else {
                currently_inside += length;
                new_intervals[i].top_inside = true;
                new_intervals[i].bottom_inside = true;
            }
        }
    }
    println!("{area}");
}
