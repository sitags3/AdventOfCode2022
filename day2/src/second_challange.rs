pub fn calc_score_two() -> i32 {
    let lines = utils::lines_from_file("./src/input.txt");

    let mut score: i32 = 0;

    for line in lines{
        match line.chars().nth(0).unwrap() {
            'A' => {
                match line.chars().nth(2).unwrap() {
                    'X' => {
                        score += 3;
                    },
                    'Y' => {
                        score += 4;
                    },
                    'Z' => {
                        score += 8;
                    },
                    _ => break
                }
            },
            'B' => {
                match line.chars().nth(2).unwrap() {
                    'X' => {
                        score += 1;
                    },
                    'Y' => {
                        score += 5;
                    },
                    'Z' => {
                        score += 9;
                    },
                    _ => break
                }
            },
            'C' => {
                match line.chars().nth(2).unwrap() {
                    'X' => {
                        score += 2;
                    },
                    'Y' => {
                        score += 6;
                    },
                    'Z' => {
                        score += 7;
                    },
                    _ => break
                }
            },
            _ => break
        }
    }

    score
}