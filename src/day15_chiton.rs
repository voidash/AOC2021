fn map_path(data: &str) -> Vec<Vec<u32>>{

    data.lines().map(|d|
        {
            let mut data = Vec::new();
            for num in d.chars() {
                data.push(num.to_digit(10).unwrap());
            }
            data
        }).collect::<Vec<Vec<u32>>>()
} 
fn min((a ,x):(u32,u32), (b,y):(u32,u32) ) -> u32{
    if a < b {
        return x;
    }
    y
}

fn find_smallest_path(data: &Vec<Vec<u32>>, x: i32 , y: i32) -> u32{
    //     1
    // -2      2
    //     -1

    let mut top: u32 = 11;
    let mut bottom: u32 = 11;
    let mut left: u32 = 11;
    let mut right: u32 = 11;


    if y-1 > 0 {
         top = data[(y-1) as usize][(x) as usize];
    }

    if y+1 < data.len() as i32  {
        bottom = data[(y+1) as usize][(x) as usize];
    } 
    if  x-1 > 0 {
        left = data[(y) as usize][(x-1) as usize];
    }
    if  x+1 > data.len() as i32 {
        right = data[(y) as usize][(x+1) as usize];
    }

    let mut count = 0;
    if top <= bottom && top <= left && top <= right {
        count += 1;
    }

    if bottom <= top && bottom <= left && bottom <= right {
        count += 2;
    }

    if left <= top && left <= bottom && left <= right {
        count += 5;
    }

    if right <= top && right <= left && right <= bottom {
        count += 10;
    }

    count
}
fn path_map(sm_path: u32, x: u32, y: u32) -> (u32,u32) {
    if sm_path == 1 {
        // top
        return (x,y-1);
    } else if sm_path == 2 {
        // bottom
        return (x,y+1);
    } else if sm_path == 5 {
        //left
        return (x-1,y);
    }

    //right
    (x+1,y)
}

pub fn part1(data: &Vec<Vec<u32>>, x: u32, y: u32) {
    let sm_path = find_smallest_path(data,x as i32, y as i32);
    let mut next_path = vec![(0,0)];
    let possible_paths = [1,2,5,10];
    // only one possible path
    if possible_paths.contains(&sm_path) {
        next_path.push(path_map(sm_path, x, y));
    }
    // two possible paths
    if sm_path > 10 && sm_path <= 15 {
        while sm_path != 0 {
            sm_path
        }
    }
    // three possible paths
    // all possible paths

}



#[cfg(test)]
mod test{
    use crate::day15_chiton::*;

    #[test]
    pub fn test_data() {
        let test_data = r#"1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581"#;

    println!("{:?}", map_path(test_data));

    }
}