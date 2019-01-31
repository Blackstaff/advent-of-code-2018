use std::collections::HashMap;

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<String> {
    input
        .lines()
        .map(|line| {
            String::from(line)
        })
        .collect()
}


#[aoc(day2, part1)]
pub fn solve_part1(input: &[String]) -> i32 {
    let mut double_letter_count = 0;
    let mut triple_letter_count = 0;
    for id in input {
        let mut letter_counts: HashMap<char, i32> = HashMap::new();
        for letter in id.chars() {
            *letter_counts.entry(letter).or_insert(0) += 1;
        }

        if letter_counts.iter().any(|(_letter, &count)| count == 2) {
            double_letter_count += 1;
        }
        if letter_counts.iter().any(|(_letter, &count)| count == 3) {
            triple_letter_count += 1;
        }
    }
    double_letter_count * triple_letter_count
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &[String]) -> String {
}
