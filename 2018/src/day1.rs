use std::collections::HashSet;
//59908

#[aoc(day1, part1)]
pub fn part1 (input: &str) -> i32 {   
    let mut counter = 0;
    for line in input.lines(){
        let number = line.parse::<i32>().unwrap();
        counter += number;
    }

    return counter;
}

#[aoc(day1, part2)]
pub fn part2 (input: &str) -> i32 {
    let mut freq = HashSet::new();
    let mut counter = 0;
    
    for line in input.lines().cycle(){
        let number = line.parse::<i32>().unwrap();
        counter += number;
        if !freq.contains(&counter) {
            freq.insert(counter);
        }else{
            return counter;
        }
    }
    
    return counter;
}
