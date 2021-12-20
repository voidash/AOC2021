use crate::utils;
use std::collections::HashMap;

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

fn sort_str(data: &str) -> String {
   let mut sorted_str = data.to_owned().chars().collect::<Vec<char>>();
    sorted_str.sort();

    sorted_str.iter().collect()
}

fn map_hash(data: &str) -> HashMap<String,u8>{
    let mut map_u8_string = HashMap::<u8,String>::new();
    let data_vec =  data.split(" ").map(|d| {
        d.trim()
    }).collect::<Vec<&str>>();

    map_u8_string.insert(1,sort_str(*data_vec.iter().find(|&&data| data.len() == 2).unwrap()));
    map_u8_string.insert(7,sort_str(*data_vec.iter().find(|&&data| data.len() == 3).unwrap()));
    map_u8_string.insert(4,sort_str(*data_vec.iter().find(|&&data| data.len() == 4).unwrap()));
    map_u8_string.insert(8,sort_str(*data_vec.iter().find(|&&data| data.len() == 7).unwrap()));

    // from 4 we can derive 9 
    if let Some(data) =  data_vec.iter().find(|&&data| {
        let mut contains_all_chars = true;
        if data.len() != 6 {return false;}
        for c in map_u8_string.get(&4).unwrap().chars() {
            if !data.contains(c) {
                contains_all_chars = false;
                break;
            }
        }
        contains_all_chars
    }) {
        map_u8_string.insert(9,sort_str(*data));
    };

    //can find 3 from 7
    if let Some(data) =  data_vec.iter().find(|&&data| {
        let mut contains_all_chars = true;
        if data.len() != 5 { return false; }
        for c in map_u8_string.get(&7).unwrap().chars() {
            if !data.contains(c){
                contains_all_chars = false;
                break;
            }
        }
        contains_all_chars
    }) {
        map_u8_string.insert(3,sort_str(*data));
    }


    // find 5 after that
    // 9 has all the segments 5 has and we also know 3  
    if let Some(data) =  data_vec.iter().find(|&&data| {
        let mut contains_all_chars = true;
        if data.len() != 5 { return false; }
        if &sort_str(data) == map_u8_string.get(&3).unwrap() { return false; }
        for c in data.chars(){
            if !map_u8_string.get(&9).unwrap().contains(c)  {
                contains_all_chars = false;
                break;
            }
        }
        contains_all_chars
    }) {

        map_u8_string.insert(5,sort_str(*data));
    }

    // 2 is simply another 5 length based character that is not 3 and 5 
    if let Some(data) =  data_vec.iter().find(|&&data| {
        let sorted_data = sort_str(data);
        data.len() == 5 && &sorted_data != map_u8_string.get(&3).unwrap() && &sorted_data != map_u8_string.get(&5).unwrap()
    }) {
        map_u8_string.insert(2,sort_str(*data));

        
    }

    // find 0 from 8 

    if let Some(data) =  data_vec.iter().find(|&&data| {
        let mut contains_all_chars = true;
        let mut contains_all_chars_for_five = true;
        if data.len() != 6 { return false; }
        if &sort_str(data) == map_u8_string.get(&9).unwrap() { return false; }

        for c in data.chars(){
            if !map_u8_string.get(&8).unwrap().contains(c){
                contains_all_chars = false;
                break;
            }
        }
        for c in map_u8_string.get(&5).unwrap().chars(){
            if !data.contains(c) {
                contains_all_chars_for_five = false;
                break;
            }
        }

        contains_all_chars && !contains_all_chars_for_five

    }) {
        map_u8_string.insert(0,sort_str(*data));
    }
        

    // 6 is the only number  with length of 6 that is not 9 or 0
    if let Some(data) =  data_vec.iter().find(|&&data| {
        let sorted_data = sort_str(data);
        data.len() == 6 && &sorted_data != map_u8_string.get(&0).unwrap() && &sorted_data != map_u8_string.get(&9).unwrap()
    }) {
        map_u8_string.insert(6,sort_str(*data));
    }

    let m_string_u8 =  HashMap::<String, u8>::from(
        [
    (map_u8_string.get(&0).unwrap().to_owned(),0),
    (map_u8_string.get(&1).unwrap().to_owned(),1),
    (map_u8_string.get(&7).unwrap().to_owned(),7),
    (map_u8_string.get(&4).unwrap().to_owned(),4),
    (map_u8_string.get(&8).unwrap().to_owned(),8),
    (map_u8_string.get(&6).unwrap().to_owned(),6),
    (map_u8_string.get(&2).unwrap().to_owned(),2),
    (map_u8_string.get(&3).unwrap().to_owned(),3),
    (map_u8_string.get(&5).unwrap().to_owned(),5),
    (map_u8_string.get(&9).unwrap().to_owned(),9),
        ]
    );

    return m_string_u8;
}

pub fn part2()  -> i32{

    let data = get_input();
    let mut count = 0;
    for (a,b) in data {

        let seven_segment_map = map_hash(&a);

        println!("{:?}",seven_segment_map);
        let num_str = b.trim().split(" ").into_iter().fold(String::from(""), |acc, d| {
            format!("{}{}",acc,seven_segment_map.get(sort_str(&d).as_str()).unwrap())
        });

        println!("{}+{}",count,num_str);
        count += num_str.parse::<i32>().unwrap();
        println!("{}",count);
    }

    count
}