use common::read_input;
use std::collections::HashMap;
use std::collections::HashSet;

pub fn run() {
    let input = read_input("aoc_25/inputs/day07.txt");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> u64 {
    let mut ways = HashSet::<usize>::new();
    let lines: Vec<&str> = input.lines().collect();
    let first_row: Vec<char> = lines[0].chars().collect();
    for i in 0..first_row.len() {
        if first_row[i] == 'S' {
            ways.insert(i as usize);
            break;
        }
    }
    print!("ways");
    for way in &ways {
        print!("{} ", way);
    }
    println!();
    let mut split_count = 0;
    for i in 1..lines.len() {
        let chars: Vec<char> = lines[i].chars().collect();
        let mut new_ways = HashSet::<usize>::new();
        for way in ways {
            if chars[way] == '^' {
                //Split and insert
                if (way - 1) > 0 {
                    new_ways.insert(way - 1);
                }
                if (way + 1) < chars.len() {
                    new_ways.insert(way + 1);
                }
                split_count += 1;
            } else {
                new_ways.insert(way);
            }
        }
        ways = new_ways;
    }
    split_count
}

fn part2(input: &str) -> u64 {
    let mut ways = HashMap::<usize, u64>::new();
    let lines: Vec<&str> = input.lines().collect();
    let first_row: Vec<char> = lines[0].chars().collect();
    for i in 0..first_row.len() {
        if first_row[i] == 'S' {
            ways.insert(i as usize, 1);
            break;
        }
    }
    println!();
    let mut split_count = 0;
    for i in 1..lines.len() {
        let chars: Vec<char> = lines[i].chars().collect();
        let mut new_ways = HashMap::<usize, u64>::new();
        for (way, count) in ways {
            if chars[way] == '^' {
                //Split and insert
                if (way - 1) >= 0 {
                    *new_ways.entry(way - 1).or_insert(0) += count;
                }
                if (way + 1) < chars.len() {
                    *new_ways.entry(way + 1).or_insert(0) += count;
                }
            } else {
                *new_ways.entry(way).or_insert(0) += count;
            }
        }
        ways = new_ways;
    }
    for (_, count) in ways {
        split_count += count;
    }
    split_count
}
