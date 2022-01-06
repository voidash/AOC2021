use crate::utils::read_file_for_input;
use std::{collections::HashMap, hash::Hash};


fn separate_polymer_template_and_insertion_rules(input_data: String) -> (String,HashMap<String,String>) {
    let (polymer_template, insertion_rules) = input_data.split_once("\n").unwrap();
    let mut insertion_set = HashMap::new();


    for line in insertion_rules.trim().lines() {
        let (a,b) = line.split_once("->").unwrap();
        insertion_set.insert(a.trim().to_string(), b.trim().to_string());
    }

    (polymer_template.trim().to_string(), insertion_set)
}
fn part1(mut template: String, insertion_set: HashMap<String,String>,step: u8) -> i32 {
    let mut count_map:HashMap<char,usize> = HashMap::new();
    for i in 0..step{
        println!("{}",i);
    let mut buffer_string= String::new();
    for i in 0..template.len()-1 {
        let str_pair = &template[i..i+2];
        let strp = String::from(str_pair);

        if let Some(data) = insertion_set.get(str_pair) {
            buffer_string = format!("{}{}",buffer_string,{
                format!("{}{}",String::from(&strp[0..1]),String::from(data))
            });
        }
    }
    buffer_string  = format!("{}{}", buffer_string, String::from(&template[template.len()-1..template.len()]));
    // println!("{}",buffer_string);
    template = buffer_string;
    }
    let mut cm: char = 'a';
    for t in template.chars() {
        cm = t;
        match count_map.get_mut(&t) {
            Some(num) => {
                *num += 1;
            }
            None => {
                count_map.insert(t,0);
            }
        }
    }
    println!("{}",cm);
    let max_char = count_map.iter().fold(cm, |acc, d| {
        if count_map.get(&acc).unwrap() < d.1 {
            return *d.0;
        }
        acc
    });

    let min_char = count_map.iter().fold(cm, |acc, d| {
        if count_map.get(&acc).unwrap() > d.1 {
            return *d.0;
        }
        acc
    });
    println!("{:?}",count_map);
    println!("{} - {}",count_map.get(&max_char).unwrap() , count_map.get(&min_char).unwrap());
    return (count_map.get(&max_char).unwrap() - count_map.get(&min_char).unwrap()) as i32;

}

#[cfg(test)]
mod test{
    use crate::utils;

    use super::{separate_polymer_template_and_insertion_rules, part1};

    #[test]
    fn check_for_smaller_input() {
        let check_data = r#"NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C"#;

// let (template, insertion_set) = separate_polymer_template_and_insertion_rules(String::from(check_data));
let val = utils::read_file_for_input("day14");
let (template, insertion_set) = separate_polymer_template_and_insertion_rules(val);
println!("{}",part1(template, insertion_set,40));



}

}

