fn main() {   
    const INPUT: &str = include_str!("input.txt");
    let mut counter = 0;
    for line in INPUT.lines(){
        let number = line.parse::<i32>().unwrap();

        counter += number;
    }

    println!("Frequency: {}", counter);
}
