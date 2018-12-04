use std::collections::HashMap;

fn main(){
    
    const INPUT: &str = include_str!("input.txt");
    
    // Create a set of points that are intersection
    let mut points = HashMap::new();
    
    let data = INPUT
        .lines()
        .map(|s1| s1.split(" ").collect::<Vec<&str>>() );
    
    for tuple in data{
        let coord =  tuple[2].split(",").collect::<Vec<&str>>();
        let x = coord[0].parse::<i32>().unwrap();
        let y = coord[1].chars().rev().skip(1).collect::<String>().chars().rev().collect::<String>().parse::<i32>().unwrap();
        let size = tuple[3].split("x").collect::<Vec<&str>>();
        let width = size[0].parse::<i32>().unwrap();
        let height = size[1].parse::<i32>().unwrap();
        
        for px in x..width+x{
            for py in y..height+y{
                let entry = points.entry((px,py)).or_insert(0);
                *entry += 1;
            }
        }
    }
    
    println!("{}",points.values().filter(|v| **v > 1).count());
}
