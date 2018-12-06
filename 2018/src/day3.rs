use std::collections::HashMap;
use std::collections::HashSet;
//59908

#[aoc(day3, part1)]
pub fn part1 (input: &str) -> usize {   
    // Create a set of points that are intersection
    let mut points = HashMap::new();
    
    let data = input
        .lines()
        .map(|s1| s1.split(" ").collect::<Vec<&str>>() );
    
    for tuple in data{
        let coord =  tuple[2].split(",").collect::<Vec<&str>>();
        let x = coord[0].parse::<i32>().unwrap();
        let y = coord[1].chars()
                        .rev().skip(1).collect::<String>()
                        .chars().rev().collect::<String>()
                        .parse::<i32>().unwrap();
                        
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
    
    return points.values().filter(|v| **v > 1).count()
}

#[aoc(day3, part2)]
pub fn part2 (input: &str) -> String {
    // Create a set of points that are intersection
    let mut points = HashMap::new();
    let mut ids = HashSet::new();
    let mut bads = HashSet::new();
    let data = input
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
    
    let data = input
        .lines()
        .map(|s1| s1.split(" ").collect::<Vec<&str>>() );
    

    
    for tuple in data{
        let id = tuple[0];
        ids.insert(id);
        let coord =  tuple[2].split(",").collect::<Vec<&str>>();
        let x = coord[0].parse::<i32>().unwrap();
        let y = coord[1].chars().rev().skip(1).collect::<String>()
                        .chars().rev().collect::<String>()
                        .parse::<i32>().unwrap();
                        
        let size = tuple[3].split("x").collect::<Vec<&str>>();
        let width = size[0].parse::<i32>().unwrap();
        let height = size[1].parse::<i32>().unwrap();

        
        for px in x..width+x{
            for py in y..height+y{
                let entry = points.entry((px,py)).or_insert(0);
                if *entry > 1{
                    bads.insert(id);
                }
            }
        }
        

    } 
    
    return ids.difference(&bads).next().unwrap().to_string();
}
