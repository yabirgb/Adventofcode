use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

fn main() {   
    let f = File::open("input.txt").unwrap();
    let file = BufReader::new(&f);
    let mut counter = 0;
    for line in file.lines(){
        let l = line.unwrap();
        
        let sign = l.chars().next().unwrap();
        let number = &l[1..].parse::<i32>().unwrap();
        if sign == '+' {
            counter += number;
        }else{
            counter -= number;
        }
    }

    println!("Frequency: {}", counter);
}
