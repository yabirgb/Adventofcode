fn invert(c: char) -> char {
    if c.to_ascii_lowercase() == c {
        c.to_ascii_uppercase()
    }else{
        c.to_ascii_lowercase()
    }
}

fn main(){
    
    const INPUT: &str = include_str!("input.txt");

    
    let data = INPUT.lines().collect::<Vec<&str>>()[0];
    
    
        
    let clean = data.trim().chars().fold(String::new(), |mut s, c| {
        
            if s.ends_with(invert(c)){
                s.pop();
            }else{
                s.push(c);
            }
            
            s
        });
    
    println!("Len of result is {}", clean.len());
    
        
}
