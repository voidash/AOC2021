use crate::utils::{self, read_file_for_input};

pub fn read_lines() -> Vec<Vec<char>> {
    let data = read_file_for_input("day10");
    data.lines()
        .collect::<Vec<&str>>()
        .iter()
        .map(|&d| d.chars().collect::<Vec<char>>())
        .collect()
}


fn find_opening_from_closing(closing: char) -> char {
    match closing {
        '}' => '{',
        ')' => '(',
        ']' => '[',
        '>' => '<',
        _ => '0',
    }
}

fn get_points(closing: char) -> u32 {
    match closing {
        '}' => 1197,
        ')' => 3,
        ']' => 57,
        '>' => 25137,
        _ => 0,
    }
}

fn is_opening(c: char) -> bool {
    let s = String::new();
    c == '(' || c == '{' || c == '<' || c == '['
}

fn get_points_part2(c: char) -> u32 {
    match c {
        '{' => 3,
        '(' => 1,
        '[' => 2,
        '<' => 4,
        _ => 0,
    }
}

pub fn part1() -> u32 {
    let data = read_lines();
    let mut total_points = 0;
    for line in data {
        let mut stack: Vec<char> = Vec::new();
        for c in line {
            if is_opening(c) {
                stack.push(c);
            } else {
                if *stack.last().unwrap() == find_opening_from_closing(c) {
                    stack.pop();
                } else {
                    total_points += get_points(c);
                    break;
                }
            }
        }
    }
    total_points
}

pub fn part2() -> u64 {
    let data = read_lines();
    let mut individual_line_points: Vec<u64> = Vec::new();
    for line in data {
        let mut total_points: u64 = 0;
        let mut stack: Vec<char> = Vec::new();
        let mut should_discard = false;
        for c in line {
            if is_opening(c) {
                stack.push(c);
            } else {
                if *stack.last().unwrap() == find_opening_from_closing(c) {
                    stack.pop();
                } else {
                    should_discard = true;
                    break;
                }
            }
        }
        if !should_discard {
            while !stack.is_empty() {
                total_points = total_points * 5 + get_points_part2(stack.pop().unwrap()) as u64;
            }

            individual_line_points.push(total_points);
        }
    }

    individual_line_points.sort();

    individual_line_points[(individual_line_points.len() / 2)]
}
