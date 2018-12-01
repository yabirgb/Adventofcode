use std::collections::HashSet;
//59908
fn main() {   

    let mut freq = HashSet::new();
    const INPUT: &str = include_str!("input.txt");

    let mut counter = 0;
    
    for line in INPUT.lines().cycle(){
        let sign = line.chars().next().unwrap();
        let number = &line[1..].parse::<i32>().unwrap();
        if sign == '+' {
            counter += number;
        }else{
            counter -= number;
        } 
        
        if !freq.contains(&counter) {
            freq.insert(counter);
        }else{
            break;
        }
    }

    
    println!("The first repeated frequency is {}", counter); 
}
