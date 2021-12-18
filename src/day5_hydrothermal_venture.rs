use std::collections::btree_set::Difference;

use crate::utils;
use regex::Regex;



enum LineSegmentType {
    Vertical,
    Horizontal,
    Diagonal
}
struct LineSegment {
    start: (usize,usize),
    stop: (usize,usize),
}
impl LineSegment {
    pub fn find_line_segment_type(&self) -> LineSegmentType {
          if self.start.0 == self.stop.0 {
            return LineSegmentType::Vertical;
          }else if self.start.1 == self.stop.1 {
            return LineSegmentType::Horizontal;
          }else {
            return LineSegmentType::Diagonal;
          }
    }

    pub fn get_all_line_segment_points(&self)-> Vec<(usize,usize)> {
        let mut points: Vec<(usize,usize)> = Vec::new();
        // points.push(self.start);
        // points.push(self.stop);
        let min = |a,b| {
            if a < b { return a;}
            else { return b; }
        };

        let max = |a,b| {
            if a > b { return a;}
            else { return b; }
        };

        let line_type =  self.find_line_segment_type(); 
             if matches!(line_type,LineSegmentType::Horizontal)  {
                 let min_val = min(self.start.0,self.stop.0);
                 let max_val = max(self.start.0,self.stop.0);
                 for i in min_val..=max_val{
                    points.push((i,self.start.1));
                 }
                }else if matches!(line_type,LineSegmentType::Vertical)  {

                 let min_val = min(self.start.1,self.stop.1);
                 let max_val = max(self.start.1,self.stop.1);
                 for i in min_val..=max_val {
                    points.push((self.start.0,i));
                 }
                
                }else{
                    // same for x and y coordinates
                    let max_x = max(self.start.0,self.stop.0) ;
                    let min_x = min(self.start.1,self.stop.1);

                    let mut difference = 0;
                    if max_x > min_x {
                        difference = max_x - min_x;
                    }

                    let mut should_add_x:bool = false;
                    let mut should_add_y:bool = false; 
                    if self.start.0 < self.stop.0 {
                        should_add_x = true;
                    }
                    if self.start.1 < self.stop.1 {
                        should_add_y = true;
                    }
                    for i in 0..=difference {
                        let mut x = self.start.0;
                        let mut y = self.start.1;
                        if should_add_x {
                            x += i;
                        }else {
                            if x > difference {
                                x -= i;
                            }
                        }
                        if should_add_y {
                            y += i;
                        }else {
                            if y > difference {
                                y -= i;
                            }
                        }
                        points.push((x,y));
                    }

                }
                

            // LineSegmentType::Diagonal =>  {
            // }
        points
    }

}

fn get_largest_point_coordinates(segments: &Vec<LineSegment>) -> (usize,usize){
    let mut largest_x = 0;
    let mut largest_y = 0;
    let max = |a:usize,b:usize| {
        if a >= b {return a;}
        else {return b;}
    };

    for segment in segments {
        if max(segment.start.0, segment.stop.0) >= largest_x {
            largest_x = max(segment.start.0, segment.stop.0);
        }
        if max(segment.start.1, segment.stop.1) >= largest_y {
            largest_y = max(segment.start.1, segment.stop.1);
        }
    }

    (largest_x,largest_y)
}

fn regex_puzzle_input() -> Vec<LineSegment>{
    let mut line_segments = Vec::<LineSegment>::new();
    let contents = utils::read_file_for_input("day5");
    let re = Regex::new(r"(\d+,\d+) -> (\d+,\d+)").unwrap();
    for capture in re.captures_iter(&contents) {
        let  generate_vec_closure = |cap: &str| -> Vec<usize> {
            return cap.split(",").collect::<Vec<&str>>().iter().map(
                |&d| {
                    d.parse::<usize>().unwrap()
                }
            ).collect();
        };
        let start_points = generate_vec_closure(&capture[1]);
        let end_points = generate_vec_closure(&capture[2]);



         line_segments.push(
             LineSegment {
                 start: (start_points[0],start_points[1]),
                 stop: (end_points[0],end_points[1])
             }
         );
    }
    line_segments    
}

pub fn part1() -> usize{
    let points: Vec<LineSegment> = regex_puzzle_input();
    let largest_coordinates : (usize,usize)= get_largest_point_coordinates(&points);
    let mut grid : Vec<Vec<usize>> = Vec::new();

    let mut row : Vec<usize> = Vec::new();
    for _ in 0..largest_coordinates.0+1 {
        row.push(0);
    }
    for _ in 0..largest_coordinates.1+1 {
        grid.push(row.clone());
    }


    for point in points {
        for to_mark_points  in point.get_all_line_segment_points() {
            grid[to_mark_points.1][to_mark_points.0] += 1;
        }
    }

    let mut counter = 0;
    for j in 0..=largest_coordinates.1 {
        for i in 0..=largest_coordinates.0 {
            if grid[j][i] > 1 {
                counter+=1;
            }
        }
    }

    counter
}

pub fn part2() -> usize{
    todo!();
}
