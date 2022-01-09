use regex::Regex;
pub fn decode_message(message: &str) -> (i32,i32,i32,i32){
    let re = Regex::new(r#"target area: x=([-0-9]{1,})..([-0-9]{1,}), y=([-0-9]{1,})..([-0-9]{1,})"#).unwrap();


    let capture =  re.captures_iter(message).collect::<Vec<regex::Captures>>();


    (capture[0][1].parse::<i32>().unwrap(),capture[0][2].parse::<i32>().unwrap(),capture[0][3].parse::<i32>().unwrap(),capture[0][4].parse::<i32>().unwrap())
}


pub fn is_inside_location(location: (i32,i32,i32,i32), x: i32, y : i32)  -> bool{
    if x >= location.0 && x <= location.1 && y >=location.2 && y<=location.3 { return true; }
    false
}

pub fn velocity_iterate(vel_x: &mut i32, vel_y: &mut i32, pos_x: i32, pos_y: i32) -> (i32,i32){
    *vel_x -= 1;
    *vel_y -= 1;

    (pos_x+*vel_x+1, pos_y+*vel_y+1)
}

pub fn part1() -> u32{
 todo!();
}

#[cfg(test)]
mod test {

    #[test]
    fn test_decode_message() {
        println!("{:?}",super::decode_message("target area: x=20..30, y=-10..-5"));
    }

    #[test]
    fn part1() {

    }
}