use crate::utils;
// https://adventofcode.com/2021/day/2
pub fn dive_day_2()  -> i32{
    let mut depth = 0;
    let mut h_pos = 0;

    let commands = utils::string_array_from_file_day2();

    for command in commands {
        let command_arr = command.split(" ").collect::<Vec<&str>>();
        match command_arr[0] {
        "forward" => {
            h_pos += command_arr[1].trim().parse::<i32>().unwrap();
        } 
        "up" => {
            depth -= command_arr[1].trim().parse::<i32>().unwrap();
        }
        "down" => {
            depth += command_arr[1].trim().parse::<i32>().unwrap();
        }
        _ => { }
    }

    }
    h_pos * depth
}

pub fn part_2_dive_day_2() -> i32 {
    let mut aim = 0;
    let mut depth = 0;
    let mut h_pos = 0;

    let commands = utils::string_array_from_file_day2();

    for command in commands {
        let command_arr = command.split(" ").collect::<Vec<&str>>();
        match command_arr[0] {
        "forward" => {
            let pos = command_arr[1].trim().parse::<i32>().unwrap();
            h_pos += pos;
            depth += aim * pos;
        } 
        "up" => {
            aim -= command_arr[1].trim().parse::<i32>().unwrap();
        }
        "down" => {
            aim += command_arr[1].trim().parse::<i32>().unwrap();
        }
        _ => { }
    }

    }
    h_pos * depth
}