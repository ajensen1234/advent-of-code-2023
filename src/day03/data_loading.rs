use nalgebra::{DMatrix, SMatrix};
use std::{collections::binary_heap, fs};

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
    pub fn binary_input_converter(input: &String, part2: bool) -> Vec<Vec<i32>> {
        input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|e| {
                        if e.is_numeric() {
                            1
                        } else if e == '.' {
                            0
                        } else if part2 && e == '*' {
                            3
                        } else {
                            2
                        }
                    })
                    .collect()
            })
            .collect()
    }
    pub fn find_row_hits(matrix: &SMatrix<i32, 140, 140>) -> Vec<RowHits> {
        let rows = 140;
        let cols = 140;
        let mut rowhits: Vec<RowHits> = Vec::new();
        for i in 0..140 {
            let mut good_idxs: Vec<usize> = Vec::new();
            for j in 0..140 {
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
        rowhits
    }
    pub fn new(data_path: &str) -> Result<Self, std::io::Error> {
        let input = fs::read_to_string(data_path)?;
        let str_input_as_lines: Vec<&str> = input.lines().collect();
        let part_2: bool = true;
        let mut part2_points: i32 = 0;
        let binary_input = Self::binary_input_converter(&input, part_2);
        let mut vec_of_contig_ones: Vec<ContiguousOnes> = binary_input
            .iter()
            .map(|bin_in| {
                let mut co = ContiguousOnes::new();
                co.find_contiguous_ones(bin_in);
                co
            })
            .collect();
        let cols: usize = binary_input.len();
        let rows: usize = binary_input[0].len();
        println!("Rows: {}, Columns: {}", rows, cols);
        let flattened: Vec<i32> = binary_input.clone().into_iter().flatten().collect();
        let matrix: SMatrix<i32, 140, 140> = SMatrix::from_row_slice(&flattened);
        let mut rowhits: Vec<RowHits> = Vec::new();
        if !part_2 {
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
        } else {
            for i in 0..rows {
                for j in 0..cols {
                    if matrix[(i, j)] == 3 {
                        let mut ones_idx: Vec<[i32; 2]> = Vec::new();
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
                        ones_idx =
                            find_ones_in3x3_matrix(matrix.view((r, c), (rs, cs)).clone().into());
                        let ones_rows: Vec<i32> = ones_idx.iter().map(|e| e[0]).collect();
                        let mut total_hits: usize = 0;
                        if i <= 5 {
                            println!("=======================");
                        }
                        let mut all_unique_hits: Vec<(usize, usize, usize)> = Vec::new();
                        for ones in ones_idx.iter() {
                            if i <= 5 {
                                let idx_vals = vec![ones[1] as usize + c];
                                println!("IDX VALS: {:?}", idx_vals);
                            }
                            vec_of_contig_ones[r + ones[0] as usize]
                                .find_hits(vec![ones[1] as usize + c]);
                            let hits_w_row: Vec<(usize, usize, usize)> = vec_of_contig_ones
                                [r + ones[0] as usize]
                                .hits
                                .iter()
                                .map(|hit| (r + ones[0] as usize, hit.0, hit.1))
                                .collect();
                            vec_of_contig_ones[r + ones[0] as usize].clear_hits();
                            for hit_wr in hits_w_row.iter() {
                                if !all_unique_hits.contains(hit_wr) {
                                    all_unique_hits.push(*hit_wr);
                                }
                            }
                            if i <= 4 {
                                println!(
                                    "Hits at [{:?},{:?}] - {:?},{:?}: {:?}",
                                    ones[0],
                                    ones[1],
                                    i,
                                    j,
                                    vec_of_contig_ones[r + ones[0] as usize].hits
                                );
                            }
                            total_hits += vec_of_contig_ones[r + ones[0] as usize].hits.len();
                        }
                        println!("ALL UNIQUE HITS: {:?}", all_unique_hits);
                        println!("TOTAL HITS: {:?}", all_unique_hits.len());
                        if all_unique_hits.len() == 2 {
                            {
                                let num1: String = str_input_as_lines[all_unique_hits[0].0]
                                    .chars()
                                    .enumerate()
                                    .filter_map(|(i, c)| {
                                        if i >= all_unique_hits[0].1 && i <= all_unique_hits[0].2 {
                                            Some(c)
                                        } else {
                                            None
                                        }
                                    })
                                    .collect();

                                let num2: String = str_input_as_lines[all_unique_hits[1].0]
                                    .chars()
                                    .enumerate()
                                    .filter_map(|(i, c)| {
                                        if i >= all_unique_hits[1].1 && i <= all_unique_hits[1].2 {
                                            Some(c)
                                        } else {
                                            None
                                        }
                                    })
                                    .collect();
                                part2_points +=
                                    num1.parse::<i32>().unwrap() * num2.parse::<i32>().unwrap();
                            }
                        }
                    }
                }
            }
        }
        println!("PART 2 POINTS: {:?}", part2_points);
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

fn find_ones_in3x3_matrix(in_mat: DMatrix<i32>) -> Vec<[i32; 2]> {
    let mut ones_idx: Vec<[i32; 2]> = Vec::new();
    for (i, row) in in_mat.row_iter().enumerate() {
        for (j, element) in row.iter().enumerate() {
            if *element == 1 {
                ones_idx.push([i as i32, j as i32]);
            }
        }
    }
    ones_idx
}
