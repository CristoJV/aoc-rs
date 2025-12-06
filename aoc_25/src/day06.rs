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
    let mut acc_result = 0;
    for i in 0..operands[0].len() {
        let local_operands: Vec<u64> = operands.iter().map(|row| row[i]).collect();
        let mut result: u64 = 0;
        if operations[i] == "*" {
            result = local_operands.iter().product();
        } else if operations[i] == "+" {
            result = local_operands.iter().sum();
        }
        acc_result += result;
    }
    acc_result
}

fn parse2(input: &str) -> u64 {
    let rows: Vec<&str> = input.lines().collect();
    let row_count = rows.len();
    let operant_digits_count: usize = row_count - 1;
    let grid: Vec<Vec<char>> = rows.iter().map(|row| row.chars().collect()).collect();
    let grid_operands: Vec<Vec<char>> = grid[0..operant_digits_count].to_vec();
    let grid_operations: Vec<char> = grid[operant_digits_count].to_vec();

    let col_count = grid[0].len();
    let mut operands: Vec<u64> = vec![];
    let mut operation: char = '-';
    let mut acc_result = 0;

    for col_i in 0..col_count {
        let c = grid_operations[col_i];

        if c == '*' || c == '+' {
            // found new operation
            // Resolve previous operation
            if operands.len() > 0 {
                let mut result: u64 = 0;
                if operation == '*' {
                    result = operands.iter().product();
                } else if operation == '+' {
                    result = operands.iter().sum();
                }
                acc_result += result;
            }
            // Set new operation
            operation = c;
            // Clear operands
            operands.clear();
        }

        //Push new operand
        if let Some(operand) = grid_operands
            .iter()
            .map(|row| row[col_i])
            .filter(|x| x.is_digit(10))
            .collect::<String>()
            .parse::<u64>()
            .ok()
        {
            operands.push(operand);
        }
    }
    // compute last
    if operands.len() > 0 {
        let mut result = 0;
        if operation == '*' {
            result = operands.iter().product();
        } else if operation == '+' {
            result = operands.iter().sum();
        }
        acc_result += result;
    }
    acc_result
}

fn part2(input: &str) -> u64 {
    parse2(input)
}
