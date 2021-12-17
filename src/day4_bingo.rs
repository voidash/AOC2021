use std::vec;

use crate::utils;

fn generate_bingo_boards() -> (Vec<u32>, Vec<[[u32;5];5]>){
    let  data = utils::read_file_for_input("day4");
    let initial_vec:Vec<&str> = data[..].split("\n").collect();

    let number_to_draw: Vec<u32> = initial_vec[0].trim().split(",").into_iter().map(
        |f|{
            f.parse::<u32>().unwrap()
        }
    ).collect();


    let mut boards: Vec<[[u32;5];5]> = Vec::new();

    for l in (2..initial_vec.len()).step_by(6) {
        let mut table  =  [[0u32;5];5];
        for c in 0..=4 {
            let row : Vec<u32> =  initial_vec[l+c].split(" ").into_iter().map(
                |f| {
                    f.parse::<u32>().unwrap()
                }
            ).collect();

            table[c] = row[..].try_into().unwrap();
        }
        boards.push(table);
    }

    (number_to_draw,boards)
}
pub fn part1() -> u32{
    let (number_to_draw,boards) =generate_bingo_boards();
    let mut boards_state = vec![[[false;5];5];boards.len()] ;
    for number_drawn in number_to_draw {
        for i  in 0..boards.len() {
              if let Some(l) = board_contains_drawn_number(number_drawn, boards[i]) {
                  boards_state[i][l.0][l.1] = true;
              }
        }

        if let Some(winner_board) = any_board_has_won(&boards_state) {
            // now sum of all false parts
            let mut sum:u32 = 0;
            for i in 0..5 {
                for j in 0..5 {
                    if boards_state[winner_board][i][j] == false {
                        sum += boards[winner_board][i][j] as u32;
                    }
                }
            }
            return sum * number_drawn as u32 ;
        }
    }
    0
}

pub fn part2() -> u32 {
    let (number_to_draw,boards) =generate_bingo_boards();
    let mut boards_state = vec![[[false;5];5];boards.len()] ;
    let mut board_which_already_won = vec![false; boards.len()];
    let mut board_who_won_last: usize = 0;
    let mut last_number_drawn:u32 = 0;
    for number_drawn in number_to_draw {
        let mut every_board_solved = true;
        for p in board_which_already_won.iter() {
            if *p == false {
               every_board_solved = false; 
               break;
            }
        }
        if every_board_solved {
            break;
        }

        for i  in 0..boards.len() {
              if let Some(l) = board_contains_drawn_number_part2(number_drawn, boards[i],&board_which_already_won) {
                  boards_state[i][l.0][l.1] = true;
              }
        }
        if let Some(winner_board) = any_board_has_won_for_part2(&boards_state,&board_which_already_won) {
            board_which_already_won[winner_board] = true;  
            board_who_won_last = winner_board ; 
            last_number_drawn = number_drawn as u32;
        }
    }

    let mut sum:u32 = 0;
    for i in 0..5 {
        for j in 0..5 {
            if boards_state[board_who_won_last][i][j] == false {
                sum += boards[board_who_won_last][i][j] as u32;
            }
        }
    }
    println!("{}",last_number_drawn);
    return sum * last_number_drawn as u32;
}

fn any_board_has_won_for_part2(boards_state: &Vec<[[bool;5];5]>,board_which_already_won: &Vec<bool>) -> Option<usize> {

    for i in 0..boards_state.len() {

    if board_which_already_won[i] {
        continue;
    }
       //check row
    for m in 0..5 {
        let mut is_solved = true;
       'inner:for l in 0..5{
           if boards_state[i][m][l] == false {
                is_solved = false;
                break 'inner;
           }
       }
       if is_solved {
            return Some(i);
       }
    }
    //check column
    for m in 0..5 {
        let mut is_solved = true;
       'cinner:for l in 0..5{
           if boards_state[i][l][m] == false {
                is_solved = false;
                break 'cinner;
           }
       }
       if is_solved {
            return Some(i);
       }
    }
    }
    None
    //check column
}
fn any_board_has_won(boards_state: &Vec<[[bool;5];5]>) -> Option<usize> {

    for i in 0..boards_state.len() {
       //check row
    for m in 0..5 {
        let mut is_solved = true;
       'inner:for l in 0..5{
           if boards_state[i][m][l] == false {
                is_solved = false;
                break 'inner;
           }
       }
       if is_solved {
            return Some(i);
       }
    }
    //check column
    for m in 0..5 {
        let mut is_solved = true;
       'cinner:for l in 0..5{
           if boards_state[i][l][m] == false {
                is_solved = false;
                break 'cinner;
           }
       }
       if is_solved {
            return Some(i);
       }
    }
    }
    None
    //check column
}


fn board_contains_drawn_number(drawn_num: u32, board: [[u32;5];5]) -> Option<(usize,usize)>{
    for i in 0..board.len() {
        for k in 0..board[i].len() {
            if board[i][k] == drawn_num {
                return Some((i,k));
            }
        }
    }
    None
}
fn board_contains_drawn_number_part2(drawn_num: u32, board: [[u32;5];5],board_which_already_won: &Vec<bool>) -> Option<(usize,usize)>{
    for i in 0..board.len() {
        if board_which_already_won[i] { continue; }
        for k in 0..board[i].len() {
            if board[i][k] == drawn_num {
                return Some((i,k));
            }
        }
    }
    None
}


