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
pub fn part1() -> u32{
    let heightmap = get_heightmap();
    let mut risk_level_sum: u32 = 0;
    for i in 1..heightmap.len() {
        for j in 1..heightmap[0].len() {
            if heightmap[i][j] < heightmap[i][j-1] && 
            heightmap[i][j] < heightmap[i][j+1] && 
            heightmap[i][j] < heightmap[i+1][j] && 
            heightmap[i][j] < heightmap[i-1][j] 
            {
                risk_level_sum += heightmap[i][j] as u32 + 1 ;
            }
        }
    }
    risk_level_sum
}

pub fn find_basins(acc: u32,heightmap: &Vec<Vec<u8>>,pos_x:i32,pos_y:i32) -> u32 {
    if heightmap[pos_y as usize][pos_x as usize] == 9 { return acc; }

    if pos_y <= 0 { return acc;}
    if pos_x <= 0 { return acc;}

    for i in [-1i32,1] {
        if heightmap[(pos_y) as usize][(pos_x+i) as usize] == heightmap[pos_y as usize][pos_x as usize]+1  {
            find_basins(acc+1, heightmap, pos_x, pos_y);
        }
        if heightmap[(pos_y+i) as usize][(pos_x) as usize] == heightmap[pos_y as usize][pos_x as usize]+1  {
            find_basins(acc+1, heightmap, pos_x, pos_y+i);
        } 
    }

    todo!();
}
pub fn part2() -> u32 {
    let heightmap = get_heightmap();
    let mut risk_level_sum: u32 = 0;
    for i in 1..heightmap.len() {
        for j in 1..heightmap[0].len() {
            if heightmap[i][j] < heightmap[i][j-1] && 
            heightmap[i][j] < heightmap[i][j+1] && 
            heightmap[i][j] < heightmap[i+1][j] && 
            heightmap[i][j] < heightmap[i-1][j] 
            {
                risk_level_sum += heightmap[i][j] as u32 + 1 ;
            }
        }
    }
    risk_level_sum
}