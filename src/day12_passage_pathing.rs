use crate::utils;


#[derive(Debug)]
enum CaveType {
    Start,
    End,
    Large,
    Small
}
#[derive(Debug)]
struct Cave<'a> {
    cave_type: CaveType,
    name:String,
    connects: Vec<&'a mut Cave<'a>>,
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

fn get_caves<'a>() -> Vec<Cave<'a>> {
    let data = utils::read_file_for_input("day12");
    let mut caves: Vec<Cave> = Vec::new();
    let all_paths = data.lines().collect::<Vec<&str>>();
   
    for path in all_paths{
            let paths = path.split("-").collect::<Vec<&str>>();
            for p  in paths{
                caves.push(Cave{
                    cave_type: get_cave_type_from_string(p),
                    name: p.to_string(),
                    connects: Vec::new()
                });
            }
    }
    for path in all_paths {
            let paths = path.split("-").collect::<Vec<&str>>();
            caves.iter_mut().find(|&d| {
                d.name == paths[0]
            }).unwrap().connects.push(
                caves.iter_mut().find(|&d|{
                    d.name == paths[1]
                }).unwrap()
            );

            caves.iter_mut().find(|&d| {
                d.name == paths[1]
            }).unwrap().connects.push(
                caves.iter_mut().find(|&d|{
                    d.name == paths[0]
                }).unwrap()
            );
    }

    caves
}
pub fn part1() -> u32 {
    let caves = get_caves();
    println!("{:?}",caves);
    12
}

pub fn part2() -> u32 {
    todo!();
}