use crate::utils::read_file_for_input;

fn read_octopus_grid() -> Vec<Vec<u8>> {
    let data = read_file_for_input("day11");
    data.lines().collect::<Vec<&str>>().iter().map(
        |&d| {
            d.trim().chars().map(
                |m|  {
                    m.to_digit(10).unwrap() as u8
                }
            ).collect::<Vec<u8>>()
        }
    ).collect::<Vec<Vec<u8>>>()
}

fn octopuses_flash(recent_exploded:&mut Vec<(usize,usize)>,octopus_grid:&mut Vec<Vec<u8>>,col:isize, row:isize) -> u32{
let mut count = 0;
if col < 0 || col as usize>= octopus_grid.len() {return 0;}
if row < 0 || row as usize >= octopus_grid[0].len() {return 0;}

let row = row as usize;
let col = col as usize;
if !recent_exploded.contains(&(col,row)) {
    octopus_grid[col][row] += 1;
}
 if octopus_grid[col][row] <= 9 { return 0; }
 // check if they flash
 if octopus_grid[col][row] > 9 {
    recent_exploded.push((col,row));
    count+=1;
    octopus_grid[col][row] = 0;
 //increase the energy level of all the adjacent octopuses 
 let row = row as isize;
 let col = col as isize;
    count+=octopuses_flash(recent_exploded,octopus_grid, col-1, row-1);
    count+=octopuses_flash(recent_exploded,octopus_grid, col+1, row+1);
    count+=octopuses_flash(recent_exploded,octopus_grid, col-1, row);
    count+=octopuses_flash(recent_exploded,octopus_grid, col+1, row);
    count+=octopuses_flash(recent_exploded,octopus_grid, col, row-1);
    count+=octopuses_flash(recent_exploded,octopus_grid, col, row+1);
    count+=octopuses_flash(recent_exploded,octopus_grid, col-1, row+1);
    count+=octopuses_flash(recent_exploded,octopus_grid, col+1, row-1);
 }
 return count;
}

pub fn part1() -> u32{ 
    let mut octopus_grid = read_octopus_grid();
    let mut total_flashes = 0;
    for _ in 0..100  {
    let mut recent_exploded : Vec<(usize,usize)> = Vec::new();
    for j in 0..octopus_grid.len() {
        for i in 0..octopus_grid[0].len() {
            total_flashes += octopuses_flash(&mut recent_exploded,&mut octopus_grid, j as isize, i as isize);
        }
    }
    // for i in 0..octopus_grid.len(){
    //     println!("{:?}",octopus_grid[i]);
    // }
    // println!("  ");
}
    total_flashes
}

pub fn part2() -> u32{ 
    let mut octopus_grid = read_octopus_grid();
    let mut total_flashes = 0;
    let mut counter = 0;
    loop {
    counter+=1;
    let mut recent_exploded : Vec<(usize,usize)> = Vec::new();
    let mut flashes = 0;
    for j in 0..octopus_grid.len() {
        for i in 0..octopus_grid[0].len() {
              flashes += octopuses_flash(&mut recent_exploded,&mut octopus_grid, j as isize, i as isize);
        }
    }
    if flashes as usize == octopus_grid.len() * octopus_grid[0].len() {
        break;
    }
    // println!("counter: {}",counter);
    // println!("total_flashes: {}",flashes);
}
    counter
}
