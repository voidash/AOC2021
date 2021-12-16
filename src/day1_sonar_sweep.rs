use crate::utils;


// https://adventofcode.com/2021/day/1 
pub fn day_1_sonar_sweep() -> u32{
    let sonar_ping_depth = utils::i32_array_from_file(); 
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

pub fn day_1_sonar_sweep_part2() -> u32 {
    let sonar_ping_depth = utils::i32_array_from_file(); 
    let mut counter = 0;
    let mut prev_sum = sonar_ping_depth[0]+sonar_ping_depth[1]+sonar_ping_depth[2];
    for i in 0..sonar_ping_depth.len()-2 {
        let buf_sum = sonar_ping_depth[i]+sonar_ping_depth[i+1]+sonar_ping_depth[i+2];
        if buf_sum > prev_sum {
            counter+=1;
        }
        prev_sum = buf_sum;
    }

    counter as u32
}