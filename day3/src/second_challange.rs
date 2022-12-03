pub fn find_common_item_two() -> i32 {
    let mut score: i32 = 0;

    let lines = utils::lines_from_file("./src/input.txt");

    for i in 0..lines.len(){
        if i % 3 != 0{
            continue;
        }
        for letter in lines[i].clone().chars() {
            if lines[i+1].contains(letter) && lines[i+2].contains(letter){
                if letter.is_uppercase(){
                    score += letter as i32 - 38;
                }else{
                    score += letter as i32 - 96;
                }
                break;
            }
        }
    }

    score
}