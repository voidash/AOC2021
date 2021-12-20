use std::io::Read;
use std::fs::File;

pub fn i32_array_from_file() -> Vec<i32> {
    let data = read_file_for_input("day1");
    data.split("\n").into_iter().map(
        |single| {
            single.parse::<i32>().unwrap()
        }
    ).collect() 
}

pub fn string_array_from_file_day2() -> Vec<String> {
    let data = read_file_for_input("day2");

    data.split("\n").into_iter().map(
        |single| {
            String::from(single)
        }
    ).collect() 
} 


#[allow(dead_code)]
pub fn read_from_url(url: &str) -> String {
    /*
    AOC doesn't allow reading from URL
    */
    let mut body = String::new();
    let mut response = reqwest::blocking::get(url).expect("Couldn't process request");
    response.read_to_string(&mut body).unwrap();

    body
}

pub fn read_file_for_input(day: &str) -> String {
    let mut data = String::new();
    let mut read_file = File::open(format!("./input/{}.txt",day)).expect("unable to open file");
    read_file.read_to_string(&mut data).expect("Unable to read data");

    data
}
