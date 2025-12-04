use common::read_input;

pub fn run() {
    let input = read_input("aoc_25/inputs/day04.txt");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn parse(input: &str) -> Vec<Vec<u64>> {
    input
        .lines()
        .collect::<Vec<&str>>()
        .iter()
        .map(|x| {
            x.chars()
                .collect::<Vec<char>>()
                .iter()
                .map(|x| if *x == '@' { 1 } else { 0 })
                .collect()
        })
        .collect::<Vec<Vec<u64>>>()
}

fn remove_forklifts(mut grid: Vec<Vec<u64>>, iterations: u32) -> u64 {
    let n_rows = grid.len();
    let n_cols = grid[0].len();
    let mut total_forklift = 0;
    let mut forklift = 0;
    let mut positions: Vec<Vec<u64>> = Vec::new();
    let mut iteration = 0;

    loop {
        for r in 0..n_rows {
            for c in 0..n_cols {
                if grid[r][c] == 1 {
                    let start_r = if r > 0 { r - 1 } else { r };
                    let end_r = if r < (n_rows - 1) { r + 1 } else { r };
                    let start_c = if c > 0 { c - 1 } else { c };
                    let end_c = if c < (n_cols - 1) { c + 1 } else { c };
                    let mut sum = 0;
                    for i in start_r..=end_r {
                        for j in start_c..=end_c {
                            sum += grid[i][j];
                        }
                    }
                    if sum < 5 {
                        forklift += 1;
                        positions.push(vec![r as u64, c as u64]);
                    }
                }
            }
        }
        total_forklift += forklift;
        iteration += 1;
        if forklift == 0 || iteration >= iterations {
            break;
        };
        for position in positions {
            grid[position[0] as usize][position[1] as usize] = 0
        }
        positions = Vec::new();
        forklift = 0;
    }
    total_forklift
}

fn part1(input: &str) -> u64 {
    let grid = parse(input);
    remove_forklifts(grid, 1)
}

fn part2(input: &str) -> u64 {
    let grid = parse(input);
    remove_forklifts(grid, 100)
}
