use csv;
use std::error::Error;
pub fn decode_calibration(input: &str) -> i32 {
    // First, we convert the input string into a list of chars
    let char_vec: Vec<char> = input.chars().collect();
    let mut first_val_found: bool = false;
    let mut output: [char; 2] = ['0', '0'];
    // iterate over the values in the calibration
    let mut first_num_index = 0;
    let mut last_num_index = 0;
    let mut idx = 0;
    for element in char_vec {
        if element.is_numeric() {
            output[1] = element;
            last_num_index = idx;
            if !first_val_found {
                output[0] = element;
                first_val_found = true;
                first_num_index = idx;
            }
        }
        idx += 1;
    }
    let spelled_num_vec = find_word_number_indices(input);
    if spelled_num_vec[0].1 < first_num_index {
        match std::char::from_digit(spelled_num_vec[0].0, 10) {
            Some(c) => output[0] = c,
            None => {}
        }
    }
    if spelled_num_vec[1].1 > last_num_index {
        match std::char::from_digit(spelled_num_vec[1].0, 10) {
            Some(c) => output[1] = c,
            None => {}
        }
    }
    let s: String = output.iter().collect();
    let int_out: i32 = s.parse().unwrap();
    return int_out;
}

pub fn day1_answer(path: &str, sum: &mut i32) -> Result<(), Box<dyn Error>> {
    let reader = csv::ReaderBuilder::new().has_headers(false).from_path(path);
    match reader {
        Ok(mut reader) => {
            for row in reader.records() {
                let val = row?;
                *sum += decode_calibration(val.as_slice());
            }
        }
        Err(reader) => {
            println!("{:?}", reader);
        }
    }

    return Ok(());
}

pub fn find_word_number_indices(input: &str) -> Vec<(u32, i32)> {
    let mut first_num_and_index = (0, 1000);
    let mut last_num_and_index = (0, 0);

    let vec_of_nums = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let mut iter = 1;
    for num in vec_of_nums.iter() {
        if let Some(fidx) = input.find(num) {
            if (fidx as i32) < first_num_and_index.1 {
                first_num_and_index = (iter, (fidx as i32));
            }
        }
        if let Some(lidx) = input.rfind(num) {
            if (lidx as i32) > last_num_and_index.1 {
                last_num_and_index = (iter, (lidx as i32));
            }
        }
        iter += 1;
    }

    return vec![first_num_and_index, last_num_and_index];
}
