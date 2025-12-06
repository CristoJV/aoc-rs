use common::read_input;
use regex::Regex;

pub fn run() {
    let input = read_input("aoc_25/inputs/day06.txt");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn parse(input: &str) -> (Vec<Vec<u64>>, Vec<&str>) {
    let re_digits = Regex::new(r"\d+").unwrap();
    let re_operations = Regex::new(r"\*|\+").unwrap();
    let lines: Vec<&str> = input.lines().collect();
    let len = lines.len();

    let mut operands: Vec<Vec<u64>> = vec![];
    let mut operations: Vec<&str> = vec![];

    let operands_str = &lines[0..len - 1];
    let operations_str = lines[len - 1];

    for row_operands_str in operands_str {
        let row_operands: Vec<u64> = re_digits
            .find_iter(row_operands_str)
            .map(|x| x.as_str().parse::<u64>().unwrap())
            .collect();
        operands.push(row_operands);
    }

    let operations: Vec<&str> = re_operations
        .find_iter(operations_str)
        .map(|o| o.as_str())
        .collect();
    (operands, operations)
}

fn part1(input: &str) -> u64 {
    let (operands, operations) = parse(input);
    let mut result = 0;
    let mut acc_result = 0;
    for i in 0..operands[0].len() {
        let local_operands: Vec<u64> = operands.iter().map(|row| row[i]).collect();
        if operations[i] == "*" {
            result = local_operands.iter().product();
        } else if operations[i] == "+" {
            result = local_operands.iter().sum();
        } else {
            result = 0;
        }
        acc_result += result;
    }
    acc_result
}

fn part2(input: &str) -> u64 {
    42
}
