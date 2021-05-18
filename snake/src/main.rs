#![no_std]
#![feature(start)]
#![no_main]

use ferr_os_librust::io;
extern crate alloc;
extern crate rand;

use alloc::collections::vec_deque::VecDeque;
use alloc::string::String;
use rand::{distributions::{Distribution, Uniform},
           rngs::SmallRng,
           Rng,
           SeedableRng
            };

#[no_mangle]
pub extern "C" fn _start(heap_address: u64, heap_size: u64, args: u64, args_number: u64) {
    ferr_os_librust::allocator::init(heap_address, heap_size);
    main();
}

const WIDTH: u8 = 80;
const HEIGHT: u8 = 24;
const SIZE : usize = WIDTH as usize * HEIGHT as usize;

enum Dir {
    Left,
    Up,
    Right,
    Down
}

enum SnakeError {
    OutOfBounds,
    EatSelf,
    GenericError
}

#[derive(Clone, Copy)]
enum State {
    Empty,
    Head,
    Snake,
    Fruit
}

struct Snake {
    body: VecDeque<(u8,u8)>,
    fruited: bool,
    direction: Dir,
}

struct PointGenerator {
    rng: SmallRng,
    distr_x: Uniform<u8>,
    distr_y: Uniform<u8>,
}

struct Game {
    snake: Snake,
    fruit: (u8,u8),
    score: u8,
    ended: bool,
    rng: PointGenerator,
    buffer: [State; SIZE],
}

fn buffer_to_line(buffer:[State; SIZE], y: u8){
    let line = &buffer[((y*HEIGHT) as usize)..(((y+1)*HEIGHT) as usize)];
    
}

impl Snake {    
    pub fn init(origin: (u8,u8), direction:Dir, length:u8) -> Self {
        let mut body = VecDeque::new();
        let mut current_pos = origin;
        for i in 0..length {
            body.push_back(current_pos);
            match direction {
                Dir::Left => {
                    current_pos.0 -= 1;
                }
                Dir::Right => {
                    current_pos.0 += 1;
                }
                Dir::Up => {
                    current_pos.1 += 1;
                }
                Dir::Down => {
                    current_pos.1 -= 1;
                }
            }
        }
        Self {
            body: body,
            fruited: false,
            direction: direction
        }
    }
    pub fn get_head_pos(&self) -> &(u8,u8) {
        self.body.front().unwrap()
    }
    pub fn get_tail_pos(&self) -> &(u8, u8) {
        self.body.back().unwrap()
    }
}
impl PointGenerator {
    pub fn init() -> Self {
        Self{
            rng: SmallRng::seed_from_u64(0),
            distr_x: Uniform::new(0_u8, WIDTH), 
            distr_y: Uniform::new(0_u8, HEIGHT),
        }
    }
    pub fn new_point(&mut self) -> (u8,u8) {
        (self.distr_x.sample(&mut self.rng),
         self.distr_y.sample(&mut self.rng))
    }
    pub fn generate_fruit(&mut self, snake: &Snake) -> (u8,u8) {
        let fruit = self.new_point();
        if snake.body.contains(&fruit) {
            self.generate_fruit(snake)
        }
        else {
            fruit
        }
    }
        
}
impl Game {
    pub fn init() -> Self {
        let snake = Snake::init((WIDTH/2,HEIGHT/2), Dir::Right, 3);
        let mut rng = PointGenerator::init();
        let fruit = rng.generate_fruit(&snake);
        let mut buffer = [State::Empty; SIZE as usize];
        let mut snake_iter = snake.body.iter();
        let head = snake_iter.next().unwrap();
        buffer[(head.1*WIDTH + head.0) as usize] = State::Head;
        for (x,y) in snake_iter {
            buffer[(y*HEIGHT+x) as usize] = State::Snake;
        }
        buffer[(fruit.1*WIDTH + fruit.0) as usize]  = State::Fruit;
        Self {
            snake: snake,
            fruit: fruit,
            score: 0,
            ended: false,
            rng: rng,
            buffer: buffer
        }
    }
    
    fn generate_fruit(&mut self) -> (u8,u8) {
        self.rng.generate_fruit(&self.snake)
    }
    
    fn displace(&mut self) -> Result<(), SnakeError>{
        let (head_x,head_y) = self.snake.get_head_pos();
        let new_head = match &self.snake.direction {
            Dir::Left => {
                if head_x == &0_u8 {
                    return Err(SnakeError::OutOfBounds)
                } else {
                    (head_x-1, head_y+0)
                }
            },
            Dir::Right => {
                if head_x == &WIDTH {
                    return Err(SnakeError::OutOfBounds)
                } else {
                    (head_x+1, head_y+0)
                }
            },
            Dir::Up => {
                if head_y == &HEIGHT {
                    return Err(SnakeError::OutOfBounds)
                }
                else {
                    (head_x+0, head_y-1)
                }
            },
            Dir::Down => {
                if head_y == &0_u8 {
                    return Err(SnakeError::OutOfBounds)
                } else {
                    (head_x+0, head_y+1)
                }
            }
        };
        if !self.snake.fruited {
            self.snake.body.pop_back();
        }
        self.snake.body.push_front(new_head);
        self.snake.fruited = self.check_eat()?;
        Ok(())
    }

    pub fn check_eat(&self) -> Result<bool, SnakeError>{
        let head = self.snake.get_head_pos();
        if self.snake.body.contains(head) {
            Err(SnakeError::EatSelf)
        } else {
            Ok(head == &self.fruit)
        }
    }
        
    pub fn update(&mut self) {
        if let Err(x) = self.displace() {
            self.ended = true
        }
        if self.snake.fruited {
            self.fruit = self.generate_fruit();
            self.score += 1
        }
    }
    
    pub fn display(&self) {
        for j in 0..HEIGHT {
            for i in 0..WIDTH {
                
            }
        }
    }
}


#[inline(never)]
fn main() {
    io::_print(&String::from("Hello world\n"));
}

fn init() {
    todo!();
}

fn main_loop() {
    todo!();
}

