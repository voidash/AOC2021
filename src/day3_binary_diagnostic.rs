use crate::utils;

pub fn binary_array() -> Vec<String> {
    let data = utils::read_file_for_input("day3");

    data.split("\n").into_iter().map(
        |single| {
            String::from(single)
        }
    ).collect() 
} 
pub fn day_3_part_1(data:Vec<String>) -> (Vec<i32>,Vec<i32>) {
    // println!("{}",data.len());

    let mut gamma_rate: String = String::new();
    let mut epsilon_rate: String = String::new();


    let mut number_of_ones = vec![0; data[0].len()];
    let mut number_of_zeroes = vec![0; data[0].len()];
    let k_len = number_of_ones.len();
    for i in 0..data.len() {
        for k  in 0..k_len{
            if data[i][k..].starts_with('1'){
                number_of_ones[k]+=1;
            }else{
                number_of_zeroes[k]+=1;
            }
        }
    }
 (number_of_ones,number_of_zeroes)

    // println!("{}",gamma_rate); println!("{}",epsilon_rate);
}
pub fn day_3_part_2(){

    let data = binary_array();
    println!("{}",data.len());

    let mut data_copy = data.clone();
    let mut data_copy2 = data.clone();

    // for part 2
    for m in 0..data[0].len() {
        let (number_of_ones, number_of_zeroes) = day_3_part_1(data_copy.clone());
        data_copy = data_copy.into_iter().filter(
            |d| {
                if number_of_ones[m]<number_of_zeroes[m] {
                    if d[m..].starts_with('0') { return false;}
                    else {return true;}
                }else {
                    if d[m..].starts_with('1') { return false;}
                    else {return true;}
                }
            }
        ).collect();

        if data_copy.len() == 1{
            break;
        }
    }

    // for part 2
    for m in 0..data[0].len() {

        let (number_of_ones, number_of_zeroes) = day_3_part_1(data_copy2.clone());
        data_copy2 = data_copy2.into_iter().filter(
            |d| {
                if number_of_ones[m]>=number_of_zeroes[m] {
                    if d[m..].starts_with('0') { return false;}
                    else {return true;}
                }else {
                    if d[m..].starts_with('1') { return false;}
                    else {return true;}
                }
            }
        ).collect();

        if data_copy2.len() == 1{
            break;
        }
    }

    println!("{}",u32::from_str_radix(&data_copy[0], 2).unwrap());
    println!("{:?}",data_copy);
    println!("{}",u32::from_str_radix(&data_copy2[0], 2).unwrap());
    println!("{:?}",data_copy2);

}

