use std::collections::HashMap;

fn part_one() -> (i32, i32){

    let mut memory = vec![4,1,15,12,0,9,9,5,5,8,7,3,14,5,12,3];
    //let mut memory = vec![0, 2,7, 0];
    let mut states = HashMap::new();
    let mut steps = 0;

    let elements = memory.len();

    while !states.contains_key(&memory){
        states.insert(memory.clone(),steps);

        //Use -i to get the first one. 
        if let Some((pos, &val)) = memory.iter().enumerate()
            .max_by_key(|&(x, y)| (y, -(x as isize))){

            //println!("{:?}", memory);
            memory[pos] = 0;

            for i in 0 .. (val as usize) {
                memory[(pos+i+1) % elements] += 1;
            }
        }
        steps += 1;        
    }

    return (steps, steps - states.get(&memory).unwrap());
    
}

fn main(){
    let (p1, p2) = part_one();
    println!("Solution to part 1: {}", p1);
    println!("Solution to part 2: {}", p2);
}
