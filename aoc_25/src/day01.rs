use common::read_input;

pub fn run() {
    let input = read_input("aoc_25/inputs/day01.txt");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> i32 {
    let lines = input.lines();
    let mut dial: i32 = 50;
    let mut password: i32 = 0;

    for line in lines{
        let direction = line.get(0..1).unwrap_or("V");
        let steps: i32 =line.get(1..).unwrap_or("0").parse().unwrap_or(0);
        
        // It is not necessary to apply modulo operator in each step.
        // Just apply it on the total count.

        dial += if direction=="L" {-steps} else {steps};
        password +=i32::from(dial % 100 == 0);

    };
    password
}

fn part2(input: &str) -> i32 {let lines = input.lines();
    let mut dial: i32 = 50;
    let mut password: i32 = 0;

    for line in lines{
        let direction = line.get(0..1).unwrap_or("V");
        let steps: i32 =line.get(1..).unwrap_or("0").parse().unwrap_or(0);
        let delta = if direction=="L" {-steps} else {steps};

        if delta >=0{
            password += (dial + steps) / 100;
        }
        else{
            let reversed  = (100 - dial) % 100;
            password += (reversed - delta) / 100;
        }
        dial = (dial + delta).rem_euclid(100);
    };
    password
}