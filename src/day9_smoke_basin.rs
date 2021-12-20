use crate::utils;

pub fn get_heightmap() -> Vec<Vec<u8>> {
    let heightmap_string = utils::read_file_for_input("day9");

    let mut data:Vec<Vec<u8>> = heightmap_string.lines().collect::<Vec<&str>>().iter().map(
        |&d| {
            let mut my_vec:Vec<u8> = d.chars().map(|d| {d.to_digit(10).unwrap() as u8}).collect();
            my_vec.splice(..0, [9]);
            my_vec.push(9);

            my_vec
        }
    ).collect();

    let mut padding_vec: Vec<u8> = Vec::new();

    for _ in 0..data[0].len() {
        padding_vec.push(9);
    }
    data.push(padding_vec.clone());
    data.splice(..0,[padding_vec.clone()]);

    data
}
pub fn part1() -> i32{
  32
}

pub fn part2() -> i32 {
    todo!();
}