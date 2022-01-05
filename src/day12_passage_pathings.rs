use std::collections::HashMap;
use std::collections::HashSet;

use crate::utils;



fn read_data() -> Vec<(String,String)> {
    let data = utils::read_file_for_input("day12");

    data.lines().collect::<Vec<&str>>().iter().map(
        |&d| {
            let paths = d.split("-").collect::<Vec<&str>>();
            (String::from(paths[0]),String::from(paths[1]))
        } 
    ).collect::<Vec<(String,String)>>()
}


fn push_to_map(h_map: &mut HashMap<String,Vec<String>>, a:&str, b:&str) {
        if h_map.contains_key(a) {
            if let Some(d) = h_map.get_mut(a) {
                d.push(String::from(b));
            }
        }else {
            h_map.insert(String::from(a), Vec::new());
            h_map.get_mut(a).unwrap().push(String::from(b));
        }
}

fn create_map() -> HashMap<String,Vec<String>> {
    let path_data = read_data();
    let mut h_map : HashMap<String,Vec<String>> = HashMap::new();
    for path in path_data.iter() {
        push_to_map(&mut h_map, &path.0, &path.1);
        push_to_map(&mut h_map, &path.1, &path.0);
    }

    h_map
}


// iterative  

pub fn part1() -> u32 {
    let map = create_map();
    println!("{:?}",map);

    let mut visited_path: HashMap<&str,Vec<&str>> = HashMap::new();

    let mut stack : Vec<&str> =   vec!["start"];
    let mut small_visited_caves = Vec::<&str>::new();

    let mut possible_ways = 0;
    loop {
        println!("{:?}",stack);
        if stack.len() == 0 {
            break;
        }
        let current_cave = stack.pop().unwrap();

        if small_visited_caves.contains(&current_cave) {
            visited_path.pop();
            continue;
        }
        if current_cave.to_lowercase() == current_cave {
            small_visited_caves.push(current_cave);
        }

        visited_path.push(current_cave);
        for connected_cave in map.get(current_cave).unwrap(){
            if connected_cave == "start" {
                continue;
            }
            if connected_cave == "end" {
                visited_path.push("end");
                // println!("{:?}", visited_path);
                possible_ways+=1;
                visited_path.pop();
                continue;
            }
            if small_visited_caves.contains(&connected_cave.as_str()) {
                //remove the path from stack 
                continue;
            }
            stack.push(connected_cave.as_str());
        }
        

        }
        possible_ways as u32
    }



pub fn part2() -> u32 {

    todo!();
}
