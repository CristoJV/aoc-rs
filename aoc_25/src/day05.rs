use common::read_input;

pub fn run() {
    let input = read_input("aoc_25/inputs/day05.txt");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn parse(input: &str) -> (Vec<Vec<u64>>, Vec<u64>) {
    if let Some((ranges, ids)) = input.split_once("\n\n") {
        let mut ranges: Vec<Vec<u64>> = ranges
            .lines()
            .map(|line| {
                line.split("-")
                    .map(|n| n.parse::<u64>().unwrap())
                    .collect::<Vec<u64>>()
            })
            .collect();
        let mut ids: Vec<u64> = ids.lines().map(|n| n.parse::<u64>().unwrap()).collect();
        ranges.sort_by_key(|v| v[0]);
        ids.sort();
        ranges = merge_ranges(ranges);
        return (ranges, ids);
    } else {
        return (vec![vec![]], vec![]);
    }
}

fn merge_ranges(mut ranges: Vec<Vec<u64>>) -> Vec<Vec<u64>> {
    let mut start: u64;
    let mut previous_len: usize = ranges.len();
    loop {
        for i in (1..ranges.len()).rev() {
            start = ranges[i][0];
            // print!("current {}-{}, ", ranges[i][0], ranges[i][1]);
            // print!("previous {}-{}, ", ranges[i - 1][0], ranges[i - 1][1]);
            // print!("start {}, ", start);
            if start <= ranges[i - 1][1] {
                // print!("{} is <= {} ", start, ranges[i - 1][1]);
                let last = ranges.remove(i);
                if ranges[i - 1][1] < last[1] {
                    ranges[i - 1][1] = last[1];
                }
                // print!("poping {}-{}, ", last[0], last[1]);
                // println!(
                //     "and setting new range, {}-{}",
                //     ranges[i - 1][0],
                //     ranges[i - 1][1]
                // );
            }
        }
        if ranges.len() < previous_len {
            previous_len = ranges.len();
        } else if ranges.len() == previous_len {
            break;
        }
    }
    ranges
}

fn part1(input: &str) -> u64 {
    let (ranges, ids) = parse(input);

    let mut valid_count: u64 = 0;
    let mut current_range_idx: usize = 0;
    for id in ids {
        loop {
            // if id is withing range - is valid
            if id >= ranges[current_range_idx][0] && id <= ranges[current_range_idx][1] {
                valid_count += 1;
                break;

            // if not withing range and id is greater than last take next range
            } else if id > ranges[current_range_idx][1] {
                current_range_idx += 1;
                if current_range_idx >= ranges.len() {
                    println!("and there is no more - is not valid");
                    break;
                }
            } else if id < ranges[current_range_idx][0] {
                break;
            }
        }
    }
    valid_count
}

fn part2(input: &str) -> u64 {
    let (ranges, _) = parse(input);
    let mut valid_count: u64 = 0;

    for range in ranges {
        let count = range[1] - range[0] + 1; // +1 as the ID ranges are inclusive
        valid_count += count;
    }
    valid_count
}
