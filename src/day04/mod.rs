pub struct ScratchCard {
    card_number: i32,
    winning_numbers: Vec<i32>,
    actual_numbers: Vec<i32>,
    pub winning_cards: u32,
    pub total_points: i32,
}

impl ScratchCard {
    pub fn new() -> Self {
        Self {
            card_number: 0,
            winning_numbers: Vec::new(),
            actual_numbers: Vec::new(),
            winning_cards: 0,
            total_points: 0,
        }
    }
    pub fn read_data(&mut self, input: &str) {
        // First, we split by "Card X: "
        let game_str: Vec<&str> = input.split(":").collect();
        let num_string = game_str[0];
        let hand_str = game_str[1];
        let card_num: String = num_string.chars().filter(|c| c.is_numeric()).collect();
        let num: i32 = card_num.parse::<i32>().unwrap();

        let hand_vals: Vec<Vec<i32>> = Self::parse_winning_numbers(hand_str);
        let winning_numbers = &hand_vals[0];
        let actual_numbers = &hand_vals[1];
        self.winning_numbers = hand_vals[0].clone();
        self.actual_numbers = hand_vals[1].clone();
        let winning_cards = Self::calculate_total_points(winning_numbers, actual_numbers);
        self.winning_cards = winning_cards.clone();
        if winning_cards >= 1 {
            let base: i32 = 2;
            let tp: i32 = base.pow(winning_cards - 1);
            self.total_points = tp;
        }
    }

    fn parse_winning_numbers(input: &str) -> Vec<Vec<i32>> {
        let winning_hand_sep: Vec<&str> = input.split("|").collect();
        let winning_num_str = winning_hand_sep[0];
        let actual_hand_str = winning_hand_sep[1];
        let mut result: Vec<Vec<i32>> = Vec::new();
        // want to create a vector of the winning numbers
        let winning_numbers: Vec<i32> = winning_num_str
            .split_whitespace()
            .map(|num_str| -> i32 { num_str.parse::<i32>().unwrap() })
            .collect();
        let hand_numbers: Vec<i32> = actual_hand_str
            .split_whitespace()
            .map(|num_str| -> i32 { num_str.parse::<i32>().unwrap() })
            .collect();
        result.push(winning_numbers);
        result.push(hand_numbers);
        result
    }
    fn calculate_total_points(winning_numbers: &Vec<i32>, hand_numbers: &Vec<i32>) -> u32 {
        // make sure that we check for duplicate
        let mut checked_nums: Vec<i32> = Vec::new();
        let mut total_winning_cards: u32 = 0;
        for num in hand_numbers.iter() {
            if winning_numbers.contains(num) {
                if !checked_nums.contains(num) {
                    total_winning_cards += 1;
                    checked_nums.push(*num);
                }
            }
        }
        total_winning_cards
    }
}
