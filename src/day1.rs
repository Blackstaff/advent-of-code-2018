use std::collections::HashSet;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<i32> {
    input
        .lines()
        .map(|line| {
            line.parse::<i32>().unwrap()
        })
        .collect()
}


#[aoc(day1, part1)]
pub fn solve_part1(input: &[i32]) -> i32 {
    input.iter().sum()
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &[i32]) -> i32 {
    let mut known_frequencies = HashSet::new();
    let mut current_frequency = 0;

    for value in input.iter().cycle() {
        current_frequency = current_frequency + value;
        if known_frequencies.contains(&current_frequency) {
            break;
        } else {
            known_frequencies.insert(current_frequency);
        }
    }
    current_frequency
}
