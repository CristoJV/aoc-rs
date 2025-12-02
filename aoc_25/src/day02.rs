use std::collections::HashMap;

use common::read_input;

pub fn run() {
    let input = read_input("aoc_25/inputs/day02.txt");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn check_id(id: &str)->u64{
    let len = id.len();
    if len%2 == 1{
        return 0;
    }
    let (part1, part2) = id.split_at(len/2);
    if part1 == part2{
        return id.parse::<u64>().unwrap();
    }
    0
}

fn part1(input: &str) -> u64 {
    let input = input.replace("\n","");
    let id_ranges:Vec<&str> = input.split(",").collect();
    let mut invalid_ids: u64= 0;
    for id_range in id_ranges{
        let limits: Vec<&str>= id_range.split("-").collect();
        let first_id: u64 = limits[0].parse().unwrap();
        let last_id: u64 = limits[1].parse().unwrap();
        for id in first_id..(last_id+1){
            invalid_ids += check_id(&id.to_string());
        }
    }
    invalid_ids as u64 
}

fn check_id_part2(id: &str, cache: &mut HashMap<u64, Vec<u64>>)-> u64{
    let len = id.len() as u64;
    let divs: &mut Vec<u64> = cache.entry(len).or_insert_with(|| {
        divisors(len)
    });

    for div in divs{
        let parts: Vec<&str> = split_in_parts(id, *div);
        
        if parts.windows(2).all(|w| w[0] == w[1]){
            return id.parse::<u64>().unwrap();
        }
    }
    return 0;
}

fn split_in_parts(s: &str, parts: u64) -> Vec<&str> {
    let len = s.len();
    let part_size = len as u64 / parts;
    let mut result = Vec::new();
    for i in 0..part_size{
        let start = (i * parts) as usize;
        let end = ((i+1) * parts) as usize;
        result.push(&s[start..end]);
    }
    result
}
fn divisors(num: u64)->Vec<u64>{
    let mut divisors: Vec<u64> = Vec::new();
    for i in 1..(num){
        if num % i == 0{
            divisors.push(i);
        }
    }

    return divisors;
}
fn part2(input: &str) -> u64 {
    let input = input.replace("\n","");
    let id_ranges:Vec<&str> = input.split(",").collect();
    let mut invalid_ids: u64= 0;
    let mut cache = HashMap::<u64,Vec<u64>>::new();
    for id_range in id_ranges{
        let limits: Vec<&str>= id_range.split("-").collect();
        let first_id: u64 = limits[0].parse().unwrap();
        let last_id: u64 = limits[1].parse().unwrap();
        for id in first_id..(last_id+1){
            invalid_ids += check_id_part2(&id.to_string(), &mut cache);
        }
    }
    invalid_ids as u64 
}