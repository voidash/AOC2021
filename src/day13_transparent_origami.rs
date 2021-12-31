use crate::utils;


fn get_coords() -> (Vec<(u32, u32)>, Vec<&str>) {

    let string_file = utils::read_file_for_input("day13");
    let coords_and_instructions = string_file.lines().collect::<Vec<&str>>().iter().map(|&d| {
        d.trim()
    }).collect::<Vec<&str>>();

    let split_position = coords_and_instructions.iter().position(|&r| { r == ""}).unwrap();
    let coords = coords_and_instructions[0..split_position].iter().map(|d| {
        let us = d.split(",").into_iter().map(|d| {
            d.parse::<u32>().unwrap()
        }).collect::<Vec<u32>>();

        (us[0], us[1])
    });
    let instructions = coords_and_instructions[split_position..].to_vec();
    println!("{:?}",coords_and_instructions);

    (coords, instructions)
}

pub fn part1() -> u32 {
    let (coords, instructions) = get_coords();
    12
}