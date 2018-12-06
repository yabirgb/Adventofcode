use std::collections::HashMap;
//59908

#[aoc(day2, part1)]
pub fn part1 (input: &str) -> u32 {   
    let mut doubles = 0;
    let mut triples = 0;
    
    for line in input.lines(){
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
    
    return doubles*triples;
}

#[aoc(day2, part2)]
pub fn part2 (input: &str) -> Option<String> {
    let ids: Vec<&str> = input.split("\n").collect();
    
    
    for word1 in ids.iter(){
        for word2 in ids.iter(){
                
            if word2.chars().zip(word1.chars()).filter(|(a,b)| a != b).count() == 1{
                let a:String = word1.chars().zip(word2.chars()).filter(|(x,y)| x == y).map(|(a,_)| a).collect();
                
                return Some(a);
            }

        } 
    }
    
     return None;
}
