pub use advent_of_code_2023::day01;


use advent_of_code_2023::{day02::SingleGame, day04::ScratchCard};

use std::env::current_dir;
fn main() {
    //day1_soln();l
    // let file = include_str!("./day02/day02-p1.txt");
    // println!(
    //     "Total Points from Example: {:?}\nPower Set Points: {:?}",
    //     day02_soln(file)[0],
    //     day02_soln(file)[1]
    // );
    //
    // let mut day03_result = ContiguousOnes::new();
    // let arr = vec![
    //     0, 0, 1, 0, 1, 1, 1, 0, 0, 0, 1, 1, 0, 1, 1, 1, 0, 0, 1, 1, 1,
    // ];
    // day03_result.find_contiguous_ones(&arr);
    // let hits = vec![2, 10, 11, 13, 14, 15];
    // day03_result.find_hits(hits);
    // let day_3_data = Day03Data::new("./src/day03/day03.txt");
    // let matrix = Matrix3::new(0, 1, 0, 1, 3, 0, 0, 0, 0);
    // let mut ones_locations = Vec::new();

    // for (i, row) in matrix.row_iter().enumerate() {
    //     for (j, element) in row.iter().enumerate() {
    //         if *element == 1 {
    //             ones_locations.push(Vector2::new(i as i32, j as i32));
    //         }
    //     }
    // }

    // println!("Locations of the 1s:");
    // for location in ones_locations {
    //     println!("{:?}", location);
    // }
    println!("Day 04 Solution: {:?}", day04_soln());
}
fn day1_soln() {
    let path = "./src/day01/day1.csv";
    println!("PWD: {:?}", current_dir());
    let mut sum: i32 = 0;
    let _ = day01::day1_answer(path, &mut sum);
    println!("Sum: {:?}", sum);
    assert_eq!(sum, 54706);
}

fn day02_soln(file: &str) -> [i32; 2] {
    let mut total_pts: i32 = 0;
    let mut power_set_points: i32 = 0;
    for line in file.lines() {
        let game: SingleGame = SingleGame::new(line);
        total_pts += game.points_from_game();
        power_set_points += game.power_set_from_game();
    }
    [total_pts, power_set_points]
}

fn day04_soln() -> i32 {
    let fp = std::fs::read_to_string("./src/day04/day04.txt");
    let mut test_soln: i32 = 0;
    let mut total_copies: [i32; 219] = [1; 219];
    match fp {
        Ok(contents) => {
            for (idx, line) in contents.lines().enumerate() {
                let mut scratch = ScratchCard::new();
                let copies_of_current_card = total_copies[idx];
                scratch.read_data(line);
                let current_card_winning_nums = scratch.winning_cards;

                for i in 1..current_card_winning_nums + 1 {
                    // if (idx + i as usize) < total_copies.len() {
                    total_copies[idx + i as usize] += copies_of_current_card;
                    // }
                }
            }
        }
        Err(e) => {
            println!("{:?}", e);
        }
    }
    for num_cards in total_copies.iter() {
        test_soln += num_cards
    }
    test_soln
}
