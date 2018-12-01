use std::collections::HashSet;
//59908
fn main() {   

    let mut freq = HashSet::new();
    const INPUT: &str = include_str!("input.txt");

    let mut counter = 0;
    
    for line in INPUT.lines().cycle(){
        let number = line.parse::<i32>().unwrap();

        if !freq.contains(&counter) {
            freq.insert(counter);
        }else{
            break;
        }
    }

    
    println!("The first repeated frequency is {}", counter); 
}
