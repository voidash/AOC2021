use crate::day1_sonar_sweep::{day_1_sonar_sweep, day_1_sonar_sweep_part2};
mod utils;

mod day1_sonar_sweep;
mod day2_dive;

fn main() {
    println!("{}",day_1_sonar_sweep());  
    println!("{}",day_1_sonar_sweep_part2());  
    println!("{}",day2_dive::dive_day_2());  
    println!("{}",day2_dive::part_2_dive_day_2());  

}
