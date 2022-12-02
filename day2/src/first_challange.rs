pub fn calc_score() -> i32 {
    let lines = utils::lines_from_file("./src/input.txt");

    let mut score: i32 = 0;

    for line in lines{
        match line.chars().nth(0).unwrap() {
            'A' => {
                match line.chars().nth(2).unwrap() {
                    'X' => {
                        score += 4;
                    },
                    'Y' => {
                        score += 8;
                    },
                    'Z' => {
                        score += 3;
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
                        score += 7;
                    },
                    'Y' => {
                        score += 2;
                    },
                    'Z' => {
                        score += 6;
                    },
                    _ => break
                }
            },
            _ => break
        }
    }

    score
}