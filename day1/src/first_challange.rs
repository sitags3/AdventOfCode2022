pub fn find_most_calories() -> i32{
    let lines = utils::lines_from_file("./src/input.txt");

    let mut current: i32 = 0;
    let mut highest: i32 = 0;

    for line in lines {
        if line != ""{
            current += line.parse::<i32>().unwrap();
        }else{
            if current > highest {
                highest = current;
            }
            current = 0;
        }
    }

    highest
}