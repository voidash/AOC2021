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

pub fn already_visited(visited: &Vec<(i32,i32)>, pos_y:i32, pos_x: i32 ) -> bool {
    visited.iter().find(|&&d| {
        d.0 == pos_y && d.1 == pos_x
    }) != None
}

pub fn find_basins(visited: &mut Vec<(i32,i32)>,heightmap: &Vec<Vec<u8>>,pos_x:i32,pos_y:i32) -> u32 {
    let mut count = 0;

    if pos_y <= 0 || pos_y >= heightmap.len() as i32 { return 0;}
    if pos_x <= 0 || pos_x >= heightmap[0].len() as i32 { return 0;}
    if heightmap[pos_y as usize][pos_x as usize] == 9 { return 0; }

    let mut found = false;

    for i in [-1i32,1] {
        if heightmap[(pos_y) as usize][(pos_x+i) as usize] == heightmap[pos_y as usize][pos_x as usize]+1  {
            if !already_visited(visited, pos_y, pos_x+i) {
                count += find_basins(visited ,heightmap, pos_x+i, pos_y);
                count += 1;
                visited.push((pos_y,pos_x+i));
            }
        }
        if heightmap[(pos_y+i) as usize][(pos_x) as usize] == heightmap[pos_y as usize][pos_x as usize]+1  {
            if !already_visited(visited, pos_y+i, pos_x) {
                count += find_basins(visited,heightmap, pos_x, pos_y+i);
                count += 1;
                visited.push((pos_y+i,pos_x));
            }
        } 
    }

    count
}

pub fn part2() -> u32 {
    let heightmap = get_heightmap();
    let mut basins: Vec<u32> = Vec::new();
    let mut visited: Vec<(i32,i32)> = Vec::new();
    for i in 1..heightmap.len() {
        for j in 1..heightmap[0].len() {
            if heightmap[i][j] < heightmap[i][j-1] && 
            heightmap[i][j] < heightmap[i][j+1] && 
            heightmap[i][j] < heightmap[i+1][j] && 
            heightmap[i][j] < heightmap[i-1][j] 
            {
               basins.push(find_basins(&mut visited,&heightmap, j as i32, i as i32));
            }
        }
    }
    println!("{:?}",basins);
    let mut three_prods = 1;
    for _ in 0..3 {
        let mut largest = 0;
        let mut position = 0;
        for c in 0..basins.len() {
            if basins[c]>largest {
                largest = basins[c];
                position = c;
            }
        }
        basins[position] = 0;
        three_prods*=largest;

    }
    three_prods
}