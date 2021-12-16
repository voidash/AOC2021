
use std::io::Read;
use std::fs::File;

// https://adventofcode.com/2021/day/1 
fn read_file_for_input() -> Vec<i32> {
    let mut data = String::new();
    let mut read_file = File::open("./input/day1.txt").expect("unable to open file");
    read_file.read_to_string(&mut data).expect("Unable to read data");

    data.split("\n").into_iter().map(
        |single| {
            single.parse::<i32>().unwrap()
        }
    ).collect() 
}

pub fn day_1_sonar_sweep() -> u32{
    let sonar_ping_depth = read_file_for_input(); 
    let mut counter = 0;
    let mut prev_measure = sonar_ping_depth[1];

    sonar_ping_depth.iter().for_each(|&depth| {
        if depth > prev_measure {
            counter+=1;
        }
        prev_measure = depth;
    });
    counter as u32
}