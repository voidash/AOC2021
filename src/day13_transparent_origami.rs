use std::ops::Index;

use crate::utils;


fn get_coords() -> (Vec<(u32, u32)>, Vec<String>) {

    let string_file = utils::read_file_for_input("day13");
    let coords_and_instructions = string_file.clone().lines().map(|d| {
        String::from(d.trim())
    }).collect::<Vec<String>>();

    let split_position = coords_and_instructions.iter().position(|r| { r == ""}).unwrap();
    let coords = coords_and_instructions[0..split_position].iter().map(|d| {
        let us = d.split(",").into_iter().map(|d| {
            d.parse::<u32>().unwrap()
        }).collect::<Vec<u32>>();

        (us[0], us[1])
    }).collect::<Vec<(u32,u32)>>();
    let instructions = coords_and_instructions[split_position+1..].to_vec();

   let impr =  instructions.iter().map(|s| {
        let new_data = &s[11..];
        String::from(new_data)
    }).collect::<Vec<String>>();

    (coords, impr)
}

pub fn part1() -> u32 {
    let (coords, instructions) = get_coords();
    //get max coordinates
    let max_x = coords.iter().fold(0, |acc,d| {
        if acc < d.0 {
            return d.0;
        }
        acc
    });

    let max_y = coords.iter().fold(0, |acc,d| {
        if acc < d.1 {
            return d.1;
        }
        acc
    });

    //create a vector of size [max_x, max_y]
    let mut graph: Vec<Vec<bool>> = Vec::new();
    for _ in 0..=max_y {
        graph.push(vec![false; (max_x+1) as usize]);
    }

    // now add the coordinates to the graph
    for &c in coords.iter() {
        graph[c.1 as usize][c.0 as usize] = true;
    }


    let mut graph_size_x = max_x ;
    let mut graph_size_y = max_y ;
    // find along x or along y  
    for folding_mechanism in instructions.iter() {
        if &folding_mechanism[0..1] == "y" {
            // fold along horizontal rod 
            let rod_position = folding_mechanism[2..].parse::<u32>().unwrap();
            let mut above_rod = (rod_position - 1) as i32;
            let mut below_rod = (rod_position + 1) as i32;

            while above_rod >= 0 && below_rod <= graph_size_y as i32 {
                for i in 0..=graph_size_x {
                    if graph[below_rod as usize][i as usize] == true{
                        graph[above_rod as usize][i as usize] = true;
                    }
                }

            above_rod -= 1;
            below_rod += 1;
            }

            graph_size_y = rod_position;
        }else if &folding_mechanism[0..1] == "x" {
            // fold along vertical rod
            let rod_position = folding_mechanism[2..].parse::<u32>().unwrap();
            let mut left_of_rod: i32 = (rod_position - 1) as i32;
            let mut right_of_rod: i32 = (rod_position + 1) as i32;

            while left_of_rod >= 0 && right_of_rod <= graph_size_x as i32 {
                for i in 0..=graph_size_y {
                    if graph[i as usize][right_of_rod as usize] == true{
                        graph[i as usize][left_of_rod as usize] = true;
                    }
                }
                left_of_rod -= 1;
                right_of_rod += 1;
            }

            graph_size_x = rod_position;

        }
        println!("new graph size ({}, {})", graph_size_x, graph_size_y);
        let visible_points = print_graph(&graph, graph_size_x as usize, graph_size_y as usize);
        println!("total visible points are {}",visible_points);
    }

    12
}
fn print_graph(graph: &Vec<Vec<bool>>, size_x: usize, size_y: usize) -> u32{
    let mut visible_points = 0;
    for i in 0..=size_y {
       for j in 0..=size_x  {
           let data = {if graph[i][j] == false {
                '.'
            } else {
                visible_points+=1;
                '#'
            }};

            print!("{}",data)
          }
       println!();
    }
    visible_points

} 