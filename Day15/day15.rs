fn main(){

    struct Generator{
        current: i64,
        factor: i64,
    }

    impl Iterator for Generator {

        type Item = i64;

        fn next(&mut self) -> Option<Self::Item>{
            self.current = self.current*self.factor % 2147483647;
            Some(self.current)
        }
    }

    fn part1(a:i64, b:i64) -> usize{
        let gen_a = Generator{current:a, factor:16807};
        let gen_b = Generator{current:b, factor:48271};

        gen_a.zip(gen_b).take(40000000).filter(|&(a,b)| a&0xffff == b&0xffff).count()
    }

    fn part2(a:i64, b:i64) -> usize{
        let gen_a = Generator{current:a, factor:16807};
        let gen_b = Generator{current:b, factor:48271};

        gen_a.filter(|x| x % 4 == 0).zip(gen_b.filter(|x| x % 8 == 0)).take(5000000).filter(|&(a,b)| a&0xffff == b&0xffff).count()
    }


    println!("Solution to part 1: {}", part1(783, 325));
    println!("Solution to part 2: {}", part2(783, 325));
}
