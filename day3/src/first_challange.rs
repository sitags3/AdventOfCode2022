pub fn find_common_item() -> i32 {
    let mut score: i32 = 0;

    let lines = utils::lines_from_file("./src/input.txt");

    for line in lines{
        let middle = line.chars().count() / 2;

        for letter in line.clone()[..middle].chars(){
            if line[middle..].contains(letter){
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