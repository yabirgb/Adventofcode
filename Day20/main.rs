use std::io::{self, BufRead};
use std::collections::HashMap;
use std::ops::{Add, AddAssign};
use util::num;

#[derive(Debug, Clone, Copy, Hash, PartialOrd, Ord, Eq, PartialEq, )]
struct Vector3D(i64,i64,i64);

impl Add for Vector3D{

    type Output = Vector3D;

    fn add(self, other:Self) -> Self::Output{
        Vector3D(self.0 + other.0, self.1+other.1, self.2 + other.2)
    }
    
}

impl AddAssign for Vector3D{

    fn add_assign(&mut self, other: Self){
        self.0 += other.0;
        self.1 += other.1;
        self.2 += other.2;
    }

}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Particle {

    pos: Vector3D,
    vel: Vector3D,
    acc: Vector3D

}

impl Particle {

    fn update(&mut self){
        self.vel += self.acc;
        self.pos += self.vel;
    }

}

named!(point3d(&str) -> Point3D, do_parse!(

    tag_s!("<") >> x: call!(num) >>
    tag_s!(",") >> y: call!(num) >>
    tag_s!(",") >> z: call!(num) >>
    tag_s!(">") >> (Vector3D(x as i64, y as i64, z as i64))
        
));

pub fn play() {

    let stdin = io::stdin();
    let mut particles: Vec<(usize, Particle)> = vec![];

    for(i, line) in stdin.lock().lines().enumerate(){

        let line = line.unwrap();

        let particle = do_parse!(line.as_str(),
                                 tag_s!("p=")>>
                                 p: point3d >>
                                 tag_s!(", v=") >>
                                 v: point3d >>
                                 tag_s!(", a=") >>
                                 a: point3d >>
                                 (Particle{pos:p, vel:v, acc:a})).unwrap().1;

        particles.push((i, particle));

    }

    let mut animated = particles.clone();

    for _ in 0..10000{
        for(_, p) in animated.iter_mut(){
            p.update();
        }
    }

    let nearest = particles.iter().min_by_key(|(_,p)| p.pos.0.abs() + p.pos.1.abs() + p.pos.2.abs()).unwrap();

    println!("The closest particle is {}", nearest.0);
    println!("{}", nearest)
}
