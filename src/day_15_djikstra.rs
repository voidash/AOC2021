use std::collections::{HashMap, VecDeque};
use crate::utils::read_file_for_input;


pub fn part1(input: &str) -> i32 {
    let mut map = HashMap::new();
    let mut best = HashMap::new();
    let mut max_x = 0;
    let mut max_y  = 0;
    for (y,line) in input.trim().split('\n').enumerate() {
        for (x,c) in line.chars().enumerate() {
            map.insert((y,x),c.to_digit(10).unwrap() as i32);
            best.insert((y,x), i32::MAX);
            max_x = x;
            max_y = y;
        }
    }
    let mut visit = VecDeque::new();
    visit.push_back(((0,0),0));
    while let Some(((y,x), cost)) = visit.pop_front() {
        if cost < best[&(y,x)] {
            best.insert((y,x),cost);
            for (dy,dx) in [(1isize, 0),(-1,0),(0,1), (0,-1)] {
                let y = (y as isize) + dy;
                let x = (x as isize) + dx;
                if y >= 0 && x >= 0 && y <= max_y as isize && x <= max_x as isize {
                    visit.push_back(((y as usize ,x as usize),cost+map[&(y as usize ,x as usize)]));
                }
            }
        }
    }

    best[&(max_y,max_x)]
}

pub fn part_2(input: &str) {
    
}

#[cfg(test)]
mod test{ 
    #[test] 
    fn part_b() {
{
        assert_eq!(super::part2("1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581
"),315);

    }
    }
    #[test]
    fn part_a() {
        assert_eq!(super::part1("1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581
"),40);

    }

#[test]
 fn question() {
    println!("{}", super::part1(crate::utils::read_file_for_input("day15").as_str()));
}
}