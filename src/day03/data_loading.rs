use nalgebra::SMatrix;

use std::fs;

use crate::day03::BinaryComparison::ContiguousOnes;
#[derive(Debug)]
pub struct Day03Data {
    input: Vec<i32>,
}
#[derive(Debug)]
struct RowHits {
    row_idx: usize,
    hits: Vec<usize>,
}
impl RowHits {
    pub fn new(row: usize, list_of_hits: &Vec<usize>) -> Self {
        Self {
            row_idx: row,
            hits: list_of_hits.to_vec(),
        }
    }
}
impl Day03Data {
    pub fn new(data_path: &str) -> Result<Self, std::io::Error> {
        let input = fs::read_to_string(data_path)?;
        let str_input_as_lines: Vec<&str> = input.lines().collect();
        let binary_input: Vec<Vec<i32>> = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|e| {
                        if e.is_numeric() {
                            1
                        } else if e == '.' {
                            0
                        } else {
                            2
                        }
                    })
                    .collect()
            })
            .collect();
        let cols: usize = binary_input.len();
        let rows: usize = binary_input[0].len();
        println!("Rows: {}, Columns: {}", rows, cols);
        let flattened: Vec<i32> = binary_input.clone().into_iter().flatten().collect();
        let matrix: SMatrix<i32, 140, 140> = SMatrix::from_row_slice(&flattened);
        let submat = matrix.view((0, 0), (3, 3));
        let mut rowhits: Vec<RowHits> = Vec::new();
        for i in 0..rows {
            let mut good_idxs: Vec<usize> = Vec::new();
            for j in 0..cols {
                if matrix[(i, j)] == 1 {
                    // Placeholder
                    let mut r = i;
                    let mut c = j;
                    let mut rs = 3;
                    let mut cs = 3;
                    if i == 0 {
                        r = i;
                        rs = 2;
                    } else if i == rows - 1 {
                        r = i - 1;
                        rs = 2;
                    } else {
                        r = i - 1;
                        rs = 3;
                    }
                    if j == 0 {
                        c = j;
                        cs = 2;
                    } else if j == cols - 1 {
                        c = j - 1;
                        cs = 2;
                    } else {
                        c = j - 1;
                    }
                    if matrix.view((r, c), (rs, cs)).amax() == 2 {
                        good_idxs.push(j);
                    }
                }
            }
            let curr_row_hits: RowHits = RowHits::new(i, &good_idxs);
            rowhits.push(curr_row_hits);
        }
        let mut total_sum: i32 = 0;
        for rowhit in rowhits {
            let idx = rowhit.row_idx;
            let hits = rowhit.hits;
            let bin_in = &binary_input[idx];
            let mut contig_ones = ContiguousOnes::new();
            contig_ones.find_contiguous_ones(bin_in);
            contig_ones.find_hits(hits);

            total_sum += contig_ones.determine_sum_from_hits(str_input_as_lines[idx]);
        }
        println!("TOTAL SUM: {:?}", total_sum);
        let din: Vec<i32> = Vec::new();
        Ok(Self { input: din })
    }
}
