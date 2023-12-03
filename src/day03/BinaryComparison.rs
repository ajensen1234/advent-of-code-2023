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
    pub fn find_contiguous_ones(&mut self, arr: &[u8]) {
        let mut current_group_start = 0;

        for (i, &n) in arr.iter().enumerate() {
            if n == 1 {
                if current_group_start == 0 {
                    current_group_start = i;
                }
            } else if current_group_start != 0 {
                let group_end = i - 1;
                self.groups.push((current_group_start, group_end));
                current_group_start = 0;
            }
        }

        if current_group_start != 0 {
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
}
