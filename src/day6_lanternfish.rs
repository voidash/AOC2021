use crate::utils;

fn generate_lanternfish_age() -> Vec<u8> {
    let contents = utils::read_file_for_input("day6");

    contents.split(",").into_iter().map(|age_str| {
        age_str.parse().unwrap()
    }).collect()
}

pub fn part1() -> u64 {
    let mut lanternfishes_age = generate_lanternfish_age();
    for _ in 0..80{ 
    for i in 0..lanternfishes_age.len() {
        if lanternfishes_age[i] == 0 {
            // add in new fish
            lanternfishes_age.push(8);
            lanternfishes_age[i] = 6;
        }else {
            lanternfishes_age[i] -= 1;
        }
        } 
    }

    lanternfishes_age.len() as u64
}

pub fn part2() -> u64 {
    // another way 

    let mut fishes: [u64; 9] = generate_lanternfish_age().iter()
    .fold([0;9], |mut fishes, &days_left_to_reproduce| {
        fishes[days_left_to_reproduce as usize] += 1;
        fishes
    });


    for _ in 0..256 {
        let fish_who_just_reproduced = fishes[0];
        fishes[0] = 0;
        for fish_id in 1..9 {
            fishes[fish_id-1] = fishes[fish_id];
            fishes[fish_id] = 0;
        }
        fishes[8] = fish_who_just_reproduced;
        fishes[6] += fish_who_just_reproduced;
    }

    fishes.iter().fold(0, |acc,f| acc+f )

}