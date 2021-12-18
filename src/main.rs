use crate::day1_sonar_sweep::{day_1_sonar_sweep, day_1_sonar_sweep_part2};
mod utils;

mod day1_sonar_sweep;
mod day2_dive;
mod day3_binary_diagnostic;
mod day4_bingo;
mod day5_hydrothermal_venture;


fn main() {
    // println!("{}",day_1_sonar_sweep());  
    // println!("{}",day_1_sonar_sweep_part2());  
    // println!("{}",day2_dive::dive_day_2());  
    // println!("{}",day2_dive::part_2_dive_day_2());  
    // day3_binary_diagnostic::day_3_part_2();
    // println!("{}",day4_bingo::part1());
    // println!("{}",day4_bingo::part2());

    println!("{}", day5_hydrothermal_venture::part1());
    // println!("{}", day5_hydrothermal_venture::part2());


}
