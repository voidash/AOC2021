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

pub fn part1() -> u32 {
    let mut get_map = create_map();
    println!("{:?}",get_map);
    let mut keys : Vec<&str> = vec!["start"];

    let mut count = 0;
    let mut i = 0;

    loop {
        if i == keys.len() {
            break;
        }
        println!("{:?}",count);

        let paths = get_map.get(keys[i]).unwrap();

        if keys[i] == "end" {
            count+=1;
            continue;
        }

        for path in paths {

                if !keys[i..].contains(&path.as_str()) {
                    
            let mut visited_small_caves :Vec<&str> = Vec::new();
            visited_small_caves.extend_from_slice(&keys[i..]);

            if path.as_str() == path.to_lowercase() {
                visited_small_caves.push(path);
            }

                 keys.push(&path.as_str()); 

            }
        }

        i+=1;
    }
    println!("{}",count);
    12

}

pub fn part2() -> u32 {

    todo!();
}
