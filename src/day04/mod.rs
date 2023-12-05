use std::usize;

use regex::Regex;

use crate::utils::read_input;

const INPUT: &str = include_str!("./input");

pub fn puzzle1() {
    let score: u32 = read_input(INPUT)
        .map(|line| {
            let winning_numbers_count = get_winning_count(line);

            if winning_numbers_count < 1 {
                return 0;
            }

            (2 as u32).pow(winning_numbers_count as u32 - 1)
        })
        .sum();

    println!("{:?}", score);
}

pub fn puzzle2() {
    let scores: Vec<u32> = read_input(INPUT)
        .map(|line| get_winning_count(line))
        .collect();

    let mut remaining_lines: Vec<(u32, u32)> = scores
        .clone()
        .into_iter()
        .enumerate()
        .map(|(index, _)| (index as u32, 1))
        .collect();

    let mut scratched = 0;

    while remaining_lines.len() > 0 {
        let (current_index, count) = remaining_lines.remove(0);
        let score = scores
            .get(current_index as usize)
            .expect("Should have score");

        for i in 0..*score {
            let (remaining_index, remaining_score) = remaining_lines
                .get(i as usize)
                .expect("should have remaining line");

            remaining_lines[i as usize] = (*remaining_index, remaining_score + count);
        }

        scratched += count;
    }

    println!("{:?}", scratched);
}

fn get_winning_count(line: &str) -> u32 {
    let re_numbers = Regex::new(r"(\d+) +").unwrap();

    let splited_line = line.split('|').collect::<Vec<&str>>();
    let target_numbers_str = splited_line.get(0).expect("Should have winning numbers");
    let played_numbers_str = splited_line.get(1).expect("Should have played numbers");
    let winning_numbers_count: usize = re_numbers
        .captures_iter(target_numbers_str)
        .filter(|cap| cap.get(1).is_some())
        .map(|cap| cap[1].parse::<i32>().expect("should be int"))
        .map(|target_number| {
            let re_target_number = Regex::new(&format!(r" +{}\b", target_number)).unwrap();
            let cap = re_target_number.captures(played_numbers_str);
            cap.is_some()
        })
        .filter(|is_matched| *is_matched)
        .collect::<Vec<bool>>()
        .len();

    winning_numbers_count as u32
}
