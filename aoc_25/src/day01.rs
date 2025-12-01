use common::read_input;

pub fn run() {
    let input = read_input("aoc_25/inputs/day01.txt");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> i32 {
    let lines = input.lines();
    let mut dial: i32 = 50;
    let mut count: u32 = 0;
    let max_size: i32 = 100;
    for line in lines{
        let direction = line.get(0..1).unwrap_or("V");
        let steps: i32 =line.get(1..).unwrap_or("0").parse().unwrap_or(0);
        if direction == "R"{
            dial = (dial + steps) % max_size;
        }
        else if direction == "L"{
            dial = dial - steps;
            while dial < 0{
                dial = max_size + dial;
            }
        }
        if dial == 0{
            count +=1;
        }

    };
    count as i32
}

fn part2(input: &str) -> i32 {let lines = input.lines();
    let mut dial: i32 = 50;
    let mut count: i32 = 0;
    let max_size: i32 = 100;

    for line in lines{
        let direction = line.get(0..1).unwrap_or("V");
        let steps: i32 =line.get(1..).unwrap_or("0").parse().unwrap_or(0);

        if direction == "R"{
            let addition: i32 = dial + steps;
            let over = addition / (max_size);
            dial = (dial + steps) % max_size;
            if over > 0 {
                count = count + if dial >0 {over} else if dial==0 {over -1} else {0};
            }
        }
        else if direction == "L"{
            let is_zero = if dial == 0 {1} else {0};
            dial = dial - steps;
            let mut on_top = 0;
            while dial < 0{
                dial = max_size + dial;
                on_top +=1;
            }
            count += on_top - is_zero;
        }

        if dial == 0{
            count +=1;
        }

    };
    count as i32
}