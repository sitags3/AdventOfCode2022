pub fn part2() -> i32 {
    let lines = utils::lines_from_file("./src/input.txt");

    for x in 13..lines[0].len(){
        let s = &lines[0][x-13..=x];
        match unique(s) {
            Some(_) => continue,
            None => return (x + 1) as i32,
        }
    }
    0
}

fn unique(s: &str) -> Option<(usize, usize, char)> {
    s.chars().enumerate().find_map(|(i, c)| {
        s.chars()
            .enumerate()
            .skip(i + 1)
            .find(|(_, other)| c == *other)
            .map(|(j, _)| (i, j, c))
    })
}