pub fn part2() -> i32{
    let mut result:i32 = 0;

    let lines = utils::lines_from_file("./src/input.txt");

    for line in lines{
        let vec: Vec<&str> = line.split(['-', ',']).collect();
        if vec[0].parse::<i32>().unwrap() <= vec[3].parse::<i32>().unwrap() && vec[1].parse::<i32>().unwrap() >= vec[2].parse::<i32>().unwrap(){
            result += 1;
        }
    }

    result
}