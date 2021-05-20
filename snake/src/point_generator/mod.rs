use rand::{distributions::{Distribution, Uniform},
           rngs::SmallRng,
           SeedableRng
            };
use crate::snake::Snake;

pub struct PointGenerator {
    pub rng: SmallRng,
    pub distr_x: Uniform<usize>,
    pub distr_y: Uniform<usize>,
}

impl PointGenerator {
    pub fn init() -> Self {
        Self{
            rng: SmallRng::seed_from_u64(0),
            distr_x: Uniform::new(0_usize, crate::WIDTH), 
            distr_y: Uniform::new(0_usize, crate::HEIGHT),
        }
    }
    pub fn new_point(&mut self) -> (usize,usize) {
        (self.distr_x.sample(&mut self.rng),
         self.distr_y.sample(&mut self.rng))
    }
    pub fn generate_fruit(&mut self, snake: &Snake) -> (usize,usize) {
        let fruit = self.new_point();
        if snake.body.contains(&fruit) {
            self.generate_fruit(snake)
        }
        else {
            fruit
        }
    }
        
}
