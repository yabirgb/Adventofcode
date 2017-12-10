const INPUT: &'static str = include_str!("input.txt");


fn parse(input:&str) -> Vec<i64>{
    input.lines().map(|line| line.parse::<i64>().expect("Error"))
        .collect::<Vec<_>>()
}

fn part1(mut state: Vec<i64>) -> i64{

    let mut steps = 0;
    let mut jump:i64 = 0;
    
    while let Some(pos) = state.get_mut(jump as usize){
        jump += *pos;
        *pos += 1;
        steps += 1;
    }

    return steps;

}

fn part2(mut state: Vec<i64>) -> i64{

    let mut steps = 0;
    let mut jump:i64 = 0;
    
    while let Some(pos) = state.get_mut(jump as usize){
        jump += *pos;
        if *pos >= 3{
            *pos -= 1;
        }else{
            *pos += 1;
        }
        steps += 1;
    }

    return steps;

}

fn main(){

    let data = parse(INPUT);
    println!("Part 1: {}", part1(data.clone()));
    println!("Part 1: {}", part2(data.clone()));

}
