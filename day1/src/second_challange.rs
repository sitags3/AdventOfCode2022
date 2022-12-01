pub fn find_top_3_calories() -> i32{
    let lines = utils::lines_from_file("./src/input.txt");

    let mut current: i32 = 0;
    let mut first: i32 = 0;
    let mut second: i32 = 0;
    let mut third: i32 = 0;

    for line in lines {
        if line != ""{
            current += line.parse::<i32>().unwrap();
        }else{
            if current > third {
                third = current;
                if third > second{
                    (third, second) = (second, third);
                    if second > first{
                        (second, first) = (first, second);
                    }
                }
            }
            current = 0;
        }
    }

    first + second + third
}