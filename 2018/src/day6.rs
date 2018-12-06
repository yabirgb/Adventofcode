use std::collections::HashMap;
use std::collections::HashSet;
use std::cmp::max;

//5358
#[aoc(day6, part1)]
pub fn part1 (input: &str) -> i32 {

    
    let data = input.lines().collect::<Vec<&str>>();
    
    let mut points = HashSet::new();
    let mut at_inf = HashSet::<usize>::new();
    let mut ids = HashMap::new();
    let mut regions = HashMap::new();
    
    let (mut max_x, mut max_y) = (0,0);
    
    for line in data {
        let point:Vec<i32> = line.replace(" ", "")
                    .split(",").map(|x| x.parse().unwrap())
                    .collect();
        max_x = max(max_x, point[0]);
        max_y = max(max_y, point[1]);
        points.insert(point);
        
    } 
    
    for (index, point) in points.iter().enumerate(){
        ids.insert(index, point);
    }
    
    for x in 0..max_x+2{
        for y in 0..max_y+2{
            
            let mut dists = Vec::<(i32, usize)>::new();
            
            for (key, value) in ids.clone() {
                dists.push( ( (value[0]-x).abs() + (value[1]-y).abs(), key) );
            }
            
            let min = *dists.iter().min_by_key(|&(x,_y)| x).unwrap();

            let area = regions.entry(min.1).or_insert(0);
            *area += 1;
            
            if x == 0 || y== 0 || x == max_x || y == max_y {
                at_inf.insert(min.1);
            }
            
        }
    }
    
    return *regions.iter().filter(|x| !at_inf.contains(&x.0)).max_by_key(|&(_a,b)| b).unwrap().1 as i32;   
}

#[aoc(day6, part2)]

pub fn part2 (input: &str) -> i32 {

    let data = input.lines().collect::<Vec<&str>>();
    let limit = 10000;
    
    let mut points = HashSet::new();
    let mut region = 0;
    
    let (mut max_x, mut max_y) = (0,0);
    
    for line in data {
        let point:Vec<i32> = line.replace(" ", "")
                    .split(",").map(|x| x.parse().unwrap())
                    .collect();
        max_x = max(max_x, point[0]);
        max_y = max(max_y, point[1]);
        points.insert(point);
        
    } 
    
    for x in 0..max_x+2{
        for y in 0..max_y+2{
        
            let mut sum = 0;
            
            for p in points.iter() {
                sum += (p[0] - x).abs() + (p[1] - y).abs();
            } 
            
            if sum < limit {
                region += 1;
            }
        }
        

    }
    
    return region
}
