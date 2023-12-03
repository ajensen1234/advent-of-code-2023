use std::fs;
#[derive(Debug)]
pub struct Day03Data {
    input: Vec<i32>,
}
impl Day03Data {
    pub fn new(data_path: &str) -> Result<Self, std::io::Error> {
        let input = fs::read_to_string(data_path)?;
        for line in input.lines() {
            println!("Line: {:?}", line);
        }
        let binary_input: Vec<Vec<i32>> = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|e| {
                        if e.is_numeric() {
                            1
                        } else if (e == '.') {
                            0
                        } else {
                            2
                        }
                    })
                    .collect()
            })
            .collect();
        for val in binary_input {
            println!("Val: {:?}", val);
        }
        let din: Vec<i32> = Vec::new();
        Ok(Self { input: din })
    }
}
