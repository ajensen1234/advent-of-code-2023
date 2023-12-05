pub use advent_of_code_2023::day01;
use advent_of_code_2023::day02::SingleGame;
use advent_of_code_2023::day03::data_loading::Day03Data;
use advent_of_code_2023::day03::BinaryComparison::ContiguousOnes;
use nalgebra::{Matrix3, Vector2};
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
    let mut day03_result = ContiguousOnes::new();
    let arr = vec![
        0, 0, 1, 0, 1, 1, 1, 0, 0, 0, 1, 1, 0, 1, 1, 1, 0, 0, 1, 1, 1,
    ];
    day03_result.find_contiguous_ones(&arr);
    let hits = vec![2, 10, 11, 13, 14, 15];
    day03_result.find_hits(hits);
    let day_3_data = Day03Data::new("./src/day03/day03.txt");
    let matrix = Matrix3::new(0, 1, 0, 1, 3, 0, 0, 0, 0);
    let mut ones_locations = Vec::new();

    for (i, row) in matrix.row_iter().enumerate() {
        for (j, element) in row.iter().enumerate() {
            if *element == 1 {
                ones_locations.push(Vector2::new(i as i32, j as i32));
            }
        }
    }

    println!("Locations of the 1s:");
    for location in ones_locations {
        println!("{:?}", location);
    }
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
