use std::collections::HashMap;

fn main(){
    
    const INPUT: &str = include_str!("input.txt");

    
    let mut data:Vec<&str> = INPUT.lines().collect();
    
    // Sorted vector with the ordered log
    data.sort();
    
    // A hasmap to store sleep of a guard
    // (id, min) -> freq
    let mut sleep: HashMap<(String, u32), u32> = HashMap::new();
    // id -> minutes
    let mut minutes_asleep: HashMap<String, u32> = HashMap::new();
    
    let mut guard_id = "";
    for (index, entry) in data.iter().enumerate(){
        let d = entry.to_string();
        //println!("{}",guard_id);
        if d.ends_with("begins shift"){
            guard_id = entry.split(" ").collect::<Vec<&str>>()[3];
        }else if d.ends_with("falls asleep"){
            let start_sleep = d.split(" ").collect::<Vec<&str>>()[1][3..5].parse::<i32>().unwrap();
            let end_sleep = data[index+1].split(" ").collect::<Vec<&str>>()[1][3..5].parse::<i32>().unwrap();
            for time in start_sleep..end_sleep {
                let point = sleep.entry((guard_id.to_string(), time as u32)).or_insert(1);
                *point += 1;
                let g = minutes_asleep.entry(guard_id.to_string()).or_insert(1);
                *g+=1;
            }
        }
    }
    
    //retrive id of the guardian as integer
    let max_guard = minutes_asleep.iter().max_by(|a, b| a.1.cmp(&b.1)).expect("No max");
    
    //retrive most frequent minute
    
    let most_freq = sleep.iter().filter(|g| (g.0).0 == *max_guard.0).max_by(|a,b| a.1.cmp(&b.1)).expect("No");
    let id_as_int = max_guard.0[1..].parse::<i32>().unwrap();
    
    println!("Max guard: {}*{} = {}", id_as_int, (most_freq.0).1, id_as_int*(most_freq.0).1 as  i32);
    
    // The guard sleep more often in the same minute

    let most_often = sleep.iter().max_by(|a,b| a.1.cmp(&b.1)).expect("No");
    //println!("{:?}", most_often);
    let id = (most_often.0).0[1..].parse::<i32>().unwrap();
    
    println!("Part 2: {}*{} = {}", id , (most_often.0).1, id*(most_often.0).1 as i32);
}
