const INPUT: &'static str = include_str!("input.txt");


fn parse(input: &str) -> Vec<(i32, i32)> {
    input
        .lines()
        .map(|line| {
            let mut it = line.split(": ").map(|item| {
                item.parse::<i32>().expect("Failed to parse")
            });
            (it.next().unwrap(), it.next().unwrap())
        })
        .collect()
}

fn penalty(data: &Vec<(i32,i32)>) -> i32{
    //t.0 level
    //t.1 size
    data.iter().filter(|&t| t.0 % ((t.1-1)*2) == 0)
        .map(|&t| t.0*t.1).sum()

}

fn penalty_p2(data: &Vec<(i32,i32)>, delay:i32) -> bool{
    let mut pos:i32;
    for &(level, size) in data{
        pos = (level + delay) % ((size-1)*2);

        if pos == 0{
            return false;
        }
    }

    return true;

}


fn main(){
    
    let v = parse(INPUT);
    //println!("{:?}", v);
    //let test = vec![(0,3), (1,2), (4,4), (6,4)];
    println!("{}", penalty(&v));

    let mut ok = false;
    let mut seconds = 0;

    while !ok{
        if penalty_p2(&v,seconds){
            println!("Part2: {}", seconds);
            ok = true;
        }

        seconds += 1;
    }
}
