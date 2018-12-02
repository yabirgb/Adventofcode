fn main() {   
    const INPUT: &str = include_str!("input.txt");

    let ids: Vec<&str> = INPUT.split("\n").collect();
    
    
    for word1 in ids.iter(){
        for word2 in ids.iter(){
                
            if word2.chars().zip(word1.chars()).filter(|(a,b)| a != b).count() == 1{
                let a:String = word1.chars().zip(word2.chars()).filter(|(x,y)| x == y).map(|(a,_)| a).collect();
                
                println!("{}", a);
                return;
            }

        } 
    }
   
}
