fn invert(c: char) -> char {
    if c.to_ascii_lowercase() == c {
        c.to_ascii_uppercase()
    }else{
        c.to_ascii_lowercase()
    }
}

fn destroy_component (s: &str , c:char) -> String {
    s.trim().chars().filter(|x| x.to_ascii_lowercase() != c && x.to_ascii_uppercase() != c).collect()
}

fn clean(data: String) -> String {
    data.trim().chars().fold(String::new(), |mut s, c| {
        
            if s.ends_with(invert(c)){
                s.pop();
            }else{
                s.push(c);
            }
            
            s
        })
}


#[aoc(day5, part1)]
pub fn part1 (input: &str) -> usize {   
    let data = input.lines().collect::<Vec<&str>>()[0];
 
    let clean = data.trim().chars().fold(String::new(), |mut s, c| {
        
            if s.ends_with(invert(c)){
                s.pop();
            }else{
                s.push(c);
            }
            
            s
        });
    
    return clean.len();
}

#[aoc(day5, part2)]
pub fn part2 (input: &str) -> usize {
    let data = input.lines().collect::<Vec<&str>>()[0];
    
    let min = "abcdefghijklmnopqrstuvwxyz".chars()
                                .map(|c| destroy_component(data,c))
                                .map(|s| clean(s).len())
                                .min().unwrap();
                                
     return min;      
}
