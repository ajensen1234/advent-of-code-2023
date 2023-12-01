pub use advent_of_code_2023::day1::day1;
use std::env::current_dir;
fn main() {
    day1_soln();
}

fn day1_soln() {
    let path = "./src/day1/day1.csv";
    println!("PWD: {:?}", current_dir());
    let mut sum: i32 = 0;
    let _ = day1::day1_answer(path, &mut sum);

    println!("Sum: {:?}", sum);
}
