use crate::utils;
use std::rc::Rc;
use std::cell::RefCell;
use std::rc::Weak;

#[derive(Debug,Clone)]
enum CaveType {
    Start,
    End,
    Large,
    Small
}

#[derive(Debug,Clone)]
struct Cave {
    cave_type: CaveType,
    name:String,
    connects: RefCell<Vec<Rc<Cave>>>,
}

fn get_cave_type_from_string(cave: &str) -> CaveType {
    // can be either start, end , uppercase letter or 
    match cave {
        "start" => CaveType::Start,
        "end" => CaveType::End,
        cave => {
            if cave == cave.to_uppercase() {
                CaveType::Large
            }else {
                CaveType::Small
            }
        } 
    }    
}

fn get_caves() -> Vec<Rc<Cave>> {
    let data = utils::read_file_for_input("day12");
    let mut caves: Vec<Rc<Cave>> = Vec::new();

   let all_paths = data.lines().collect::<Vec<&str>>().iter().map(|&d| {d.to_string()}).collect::<Vec<String>>();
   
    for path in all_paths.iter(){
            let paths = path.split("-").collect::<Vec<&str>>();
            for p  in paths{
                caves.push(Rc::new(Cave{
                    cave_type: get_cave_type_from_string(p),
                    name: p.to_string(),
                    connects: RefCell::new(vec![])
                }));
            }
    }

    for path in all_paths.iter() {
            let paths = path.split("-").collect::<Vec<&str>>();
            let c1= caves.iter().find(|d| {
                d.name == paths[0]
            }).unwrap();
            let mut c1_vec = c1.connects.borrow_mut();

            let c2= caves.iter().find(|d| {
                d.name == paths[1]
            }).unwrap();
            // c1_vec.push(Rc::downgrade(c2));

            c1_vec.push(c2.to_owned());

            let mut c2_vec = c2.connects.borrow_mut();
            c2_vec.push(c1.to_owned());
            // c2_vec.push(Rc::downgrade(c1));
    }

    caves
}
pub fn part1() -> u32 {
    let caves = get_caves();

    let mut stack : Vec<Rc<Cave>> = Vec::new();
    // let mut visited: Vec<Rc<Cave>> = Vec::new();

    // stack.push(caves[0]);
    for connected in caves[0].connects.borrow().iter() {
        println!("{}",connected.name);
    }

    

    32
}

pub fn part2() -> u32 {
    todo!();
}
