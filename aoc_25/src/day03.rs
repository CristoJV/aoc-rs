use common::read_input;

pub fn run() {
    let input = read_input("aoc_25/inputs/day03.txt");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn update(next_joltage: u64, max_joltages: &mut Vec<u64>, depth: usize, remaining_joltages:u64){
    let len = max_joltages.len();
    if depth >= len  {return}
    if remaining_joltages > (len-1-depth) as u64 && next_joltage > max_joltages[depth]{
        for i in depth..len{
            max_joltages[i] = 0;
        }
        max_joltages[depth] = next_joltage;
    }else{
        update(next_joltage, max_joltages, depth+1, remaining_joltages);
    }
}

fn part1(input: &str) -> u64 {
    let lines = input.lines();
    let mut total_joltage: u64 = 0;
    for line in lines{
        let joltages: Vec<u64> = line.chars().into_iter().map(|x| x.to_digit(10).unwrap() as u64).collect();
        let mut max_joltages = vec![0,0];
        let n = joltages.len();
        for i in 0..n{
            update(joltages[i], &mut max_joltages, 0, (n as u64)-(i as u64));
        }
        let joltage = max_joltages.iter().fold(0, |acc, elem| acc * 10 +elem);
        total_joltage +=joltage;
    }
    total_joltage
}

fn part2(input: &str) -> u64 {let lines = input.lines();
    let mut total_joltage: u64 = 0;
    for line in lines{
        let joltages: Vec<u64> = line.chars().into_iter().map(|x| x.to_digit(10).unwrap() as u64).collect();
        let mut max_joltages = vec![0;12];
        let n = joltages.len();
        for i in 0..n{
            update(joltages[i], &mut max_joltages, 0, (n as u64)-(i as u64));
        }
        let joltage = max_joltages.iter().fold(0, |acc, elem| acc * 10 +elem);
        total_joltage +=joltage;
    }
    total_joltage
}