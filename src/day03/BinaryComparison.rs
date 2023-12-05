pub struct ContiguousOnes {
    pub groups: Vec<(usize, usize)>,
    pub hits: Vec<(usize, usize)>,
}

impl ContiguousOnes {
    pub fn new() -> Self {
        Self {
            groups: Vec::new(),
            hits: Vec::new(),
        }
    }
    pub fn find_contiguous_ones(&mut self, arr: &Vec<i32>) {
        let mut current_group_start = 0;
        let mut in_group: bool = false;

        for (i, &n) in arr.iter().enumerate() {
            if n == 1 {
                // if current_group_start == 0 {
                if !in_group {
                    current_group_start = i;
                    in_group = true;
                }
            } else if in_group {
                // } else if current_group_start != 0 {
                let group_end = i - 1;
                self.groups.push((current_group_start, group_end));
                in_group = false;
            }
        }

        if in_group {
            self.groups.push((current_group_start, arr.len()));
        }
    }
    pub fn find_hits(&mut self, input_idx: Vec<usize>) {
        for idx in input_idx.iter() {
            let matching_groups = self
                .groups
                .iter()
                .filter(|range| (*idx >= range.0) && (*idx <= range.1));
            let mut new: Vec<(usize, usize)> = Vec::new();
            for range in matching_groups {
                if !self.hits.contains(range) {
                    new.push(*range);
                }
            }
            self.hits.extend(new);
        }
    }
    pub fn determine_sum_from_hits(&self, input_str: &str) -> i32 {
        let mut sum = 0;
        for hit in &self.hits {
            let num: String = input_str
                .chars()
                .enumerate()
                .filter_map(|(i, c)| {
                    if i >= hit.0 && i <= hit.1 {
                        Some(c)
                    } else {
                        None
                    }
                })
                .collect();
            sum += num.parse::<i32>().unwrap();
        }
        sum
    }
}
