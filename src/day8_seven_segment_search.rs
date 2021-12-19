use crate::utils;

fn get_input() -> Vec<(String,String)> {
    let contents  = utils::read_file_for_input("day8");
    contents.split("\n").map(|data| {
        let split_data:Vec<&str> = data.split("|").collect();
        (String::from(split_data[0]),String::from(split_data[1]))
    }).collect()
}

pub fn part1() -> i32 {
    let data = get_input();
    let mut count = 0;
    for (_,b) in data {
        count += b.split(" ").into_iter().fold(0, |acc, d| {
            let mut val = acc;
            let want_list = [2,4,3,7];
            if want_list.contains(&d.len())  {
                val += 1 ;
            }
            val
        });
    }
    count as i32
}

pub fn part2()  -> i32{
    todo!();
}