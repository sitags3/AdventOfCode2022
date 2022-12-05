pub fn part1() -> String{
    let mut stacks: Vec<Vec<char>> = vec![Vec::new(); 9];

    //manual initialization
    //                         [R] [J] [W]
    //             [R] [N]     [T] [T] [C]
    // [R]         [P] [G]     [J] [P] [T]
    // [Q]     [C] [M] [V]     [F] [F] [H]
    // [G] [P] [M] [S] [Z]     [Z] [C] [Q]
    // [P] [C] [P] [Q] [J] [J] [P] [H] [Z]
    // [C] [T] [H] [T] [H] [P] [G] [L] [V]
    // [F] [W] [B] [L] [P] [D] [L] [N] [G]
    //  1   2   3   4   5   6   7   8   9 

    stacks[0] = ['F','C','P','G','Q','R'].to_vec();
    stacks[1] = ['W','T','C','P'].to_vec();
    stacks[2] = ['B','H','P','M','C'].to_vec();
    stacks[3] = ['L','T','Q','S','M','P','R'].to_vec();
    stacks[4] = ['P','H','J','Z','V','G','N'].to_vec();
    stacks[5] = ['D','P','J'].to_vec();
    stacks[6] = ['L','G','P','Z','F','J','T','R'].to_vec();
    stacks[7] = ['N','L','H','C','F','P','T','J'].to_vec();
    stacks[8] = ['G','V','Z','Q','H','T','C','W'].to_vec();

    let mut result:String = "".to_string();

    let lines = utils::lines_from_file("./src/input.txt");

    //3 to 5
    for line in lines{
        let input:Vec<&str> = line.split(" ").collect();
        let from = input[3].parse::<usize>().unwrap()-1;
        let to = input[5].parse::<usize>().unwrap()-1;
        for _ in 0..input[1].parse::<usize>().unwrap(){
            let last = stacks[from][stacks[from].len()-1];
            stacks[to].push(last);
            stacks[from].pop();
        }
    }

    for x in 0..stacks.len(){
        result += &stacks[x][stacks[x].len()-1].to_string();
    }

    result
}