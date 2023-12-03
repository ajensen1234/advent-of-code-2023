use std::fs;
#[derive(Debug)]
pub struct Day03Data {
    input: Vec<i32>,
}
impl Day03Data {
    pub fn new(data_path: &str) -> Result<Self, std::io::Error> {
        let input = fs::read_to_string(data_path)?;
        // match fs::read_to_string(data_path) {
        //     Ok(c) => println!("Yay: {:?}", c),
        //     Err(c) => println!("Err: {:?}", c),
        // }
        for line in input.lines() {
            println!("Line: {:?}", line);
        }
        let din: Vec<i32> = Vec::new();
        Ok(Self { input: din })
    }
}
