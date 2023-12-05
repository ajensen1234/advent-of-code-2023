pub struct SingleGame {
    total_points: i32,
    total_red: i32,
    total_green: i32,
    total_blue: i32,
    allowable_game: bool,
}
trait GameParser {
    fn parse_points(input: &str) -> i32;
    fn parse_marbles(input: &str) -> [i32; 3];
}
impl SingleGame {
    pub fn new(input: &str) -> Self {
        // Here, we take in the input string representing a single game.
        let game_string_vec: Vec<&str> = input.split(':').collect();
        let total_points = Self::parse_points(game_string_vec[0]);
        let [total_red, total_green, total_blue] = Self::parse_marbles(game_string_vec[1]);
        // assume game is not allowable to start
        let allowable_game = (total_red <= 12) && (total_green <= 13) && (total_blue <= 14);

        Self {
            total_points,
            total_red,
            total_green,
            total_blue,
            allowable_game,
        }
    }
    pub fn points_from_game(&self) -> i32 {
        if self.allowable_game {
            self.total_points
        } else {
            0
        }
    }
    pub fn power_set_from_game(&self) -> i32 {
        self.total_blue * self.total_green * self.total_red
    }
}
impl GameParser for SingleGame {
    fn parse_points(input: &str) -> i32 {
        let it: String = input.chars().filter(|char| char.is_numeric()).collect();
        it.parse().unwrap()
    }
    fn parse_marbles(input: &str) -> [i32; 3] {
        // Starting off with rgb values at zero
        let mut rgb: [i32; 3] = [0, 0, 0];
        // splitting up into each of the individual games
        // Will likely create a function that looks at an
        // individual game and then determines with
        // a match statement how many points to ada
        let games: Vec<&str> = input.split(';').collect();
        for hand in games {
            let rgb_in_hand = hand_count(hand);
            if rgb_in_hand[0] > rgb[0] {
                rgb[0] = rgb_in_hand[0];
            }
            if rgb_in_hand[1] > rgb[1] {
                rgb[1] = rgb_in_hand[1];
            }
            if rgb_in_hand[2] > rgb[2] {
                rgb[2] = rgb_in_hand[2];
            }
        }
        rgb
    }
}

fn hand_count(hand: &str) -> [i32; 3] {
    let mut hand_rgb: [i32; 3] = [0, 0, 0];
    let marble_vals: Vec<&str> = hand.split(',').collect();

    for marble in marble_vals {
        // Get the number here
        let num_marble_str: String = marble.chars().filter(|char| char.is_numeric()).collect();
        let num_marble: i32 = num_marble_str.parse().unwrap();
        if marble.contains("green") {
            hand_rgb[1] = num_marble;
        } else if marble.contains("red") {
            hand_rgb[0] = num_marble;
        } else if marble.contains("blue") {
            hand_rgb[2] = num_marble;
        }
    }
    hand_rgb
}
