use std::collections::HashMap;

fn main() {   
    const INPUT: &str = include_str!("input.txt");
    
    let mut doubles = 0;
    let mut triples = 0;
    
    for line in INPUT.lines(){
        let mut frequency = HashMap::new();
        
        for letter in line.chars(){
            *frequency.entry(letter).or_insert(0) += 1;
        }
        
        let mut double = false;
        let mut triple = false;
        
        for (_key, value) in frequency.into_iter() {
            if value == 2{
                double = true;
            }
            else if value == 3 {
                triple = true;
            }
                
        }
        
        if double{
            doubles += 1;
        }
        
        if triple{
            triples += 1;
        }
    }
    
    println!("Result {}", doubles*triples);
}
