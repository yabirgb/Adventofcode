// Solution improved by https://www.reddit.com/r/adventofcode/comments/a5eztl/2018_day_12_solutions/ebm6h87

//use std::collections::HashSet;
use std::collections::HashMap;

fn evolve(input: &str, generations: i64) -> isize{
	const CONVERGE:u32 = 19;
	let lines = input.lines().collect::<Vec<&str>>();
	let pots = lines[0]
					.split(':')
					.map(str::trim)
					.skip(1)
					.collect::<Vec<&str>>();

	let mut rules: HashMap<&str, char> = HashMap::new();
	let mut diffs: HashMap<isize, u32> = HashMap::new();
	lines[2..].iter().for_each(|s|{
		if s.ends_with('.') {
			rules.insert(&s[0..5], '.');
		}else{
			rules.insert(&s[0..5], '#');
		}
	});

	//println!("{:?}", pots);

	let mut pot = String::from("...");
   	pot.push_str(&pots[0]);
    pot.push_str("...");
	let mut last = 0;
	for gen in 0..generations{
		let mut result = String::from("...");
		
		for i in 2..pot.len()-2{
			let window = &pot[i-2..=i+2];

			match rules.get(window){
				Some('#') => {
					result.push('#');
				},

				_ => result.push('.')
			}
		}
		result.push_str("...");
		pot = result;

        // Our string grows by one '.' at both the beginning and end each generation
        let score = pot
            .chars()
            .enumerate()
            .filter(|(_, c)| c == &'#')
            .map(|(i, _)| i as isize - (3 + gen as isize))
            .sum::<isize>();
        let e = diffs.entry(score as isize - last as isize).or_insert(0);
        if *e > CONVERGE {
            return (generations - gen) as isize * (score - last) + score;
        } else {
            *e += 1;
        }
        last = score;

	}

	
	
    return last;
}

#[aoc(day12, part1)]
pub fn part1 (input: &str) -> isize {   
	
	return evolve(input, 20);

}


#[aoc(day12, part2)]
pub fn part2 (input: &str) -> isize {   
	return evolve(input,50000000000);

}
