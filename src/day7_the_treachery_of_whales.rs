use crate::utils;

fn get_crab_submarine_position() -> Vec<i32> {
    let contents = utils::read_file_for_input("day7");

    contents.split(",").into_iter().map(|age_str| {
        age_str.parse().unwrap()
    }).collect()
}


fn find_largest(data: &Vec<i32>) -> i32 {
    let mut max_number = data[0];
    for &d in data {
        if d >= max_number {
            max_number = d;
        }
    }
    max_number
}
fn find_smallest(data: &Vec<i32>) -> i32 {
    let mut min_number = data[0];
    for &d in data{
        if d < min_number {
            min_number = d;
        }
    }
    min_number
}

pub fn part1() -> i32 {
    let pos =  get_crab_submarine_position();
    println!("{}",pos.len());
    let largest_number = find_largest(&(pos.clone()));
    let mut fuel_required = Vec::<i32>::new();
    for i in (0..=largest_number).rev() {
        let mut cum_sum =  0;
        for &each_crab_position in &pos {
            cum_sum += (each_crab_position - i).abs();
        }

        fuel_required.push(cum_sum);
    }

    find_smallest(&fuel_required)
}

pub fn natural_sum(val:i32) -> i32 {
    (val * (val-1)) /2
}

pub fn part2() -> i32 {
    let pos =  get_crab_submarine_position();
    println!("{}",pos.len());
    let largest_number = find_largest(&(pos.clone()));
    let mut fuel_required = Vec::<i32>::new();
    for i in (0..=largest_number).rev() {
        let mut cum_sum =  0;
        for &each_crab_position in &pos {
            cum_sum += natural_sum((each_crab_position - i).abs()+1);

        }

        fuel_required.push(cum_sum);
    }

    find_smallest(&fuel_required)
}


