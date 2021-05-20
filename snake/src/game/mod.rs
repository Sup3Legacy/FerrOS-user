use crate::{state::State,
            snake::Snake,
            point_generator::PointGenerator,
            errors::SnakeError,
            action::Action,
            direction::Dir,
            WIDTH,
            HEIGHT,
            SIZE};
use ferr_os_librust::io;
use alloc::string::String;

pub struct Game {
    pub snake: Snake,
    pub fruit: (usize,usize),
    pub score: usize,
    pub ended: bool,
    pub rng: PointGenerator,
    pub buffer: [State; SIZE],
}

//80*20



impl Game {
    pub fn init() -> Self {
        let snake = Snake::init((WIDTH/2,HEIGHT/2), Dir::Right, 4);
        let mut rng = PointGenerator::init();
        let fruit = rng.generate_fruit(&snake);
        let mut buffer = [State::Empty; SIZE];
        let mut snake_iter = snake.body.iter();
        let head = snake_iter.next().unwrap();
        buffer[head.1*WIDTH + head.0] = State::Head(Dir::Right);
        for (x,y) in snake_iter {
            buffer[y*WIDTH+x] = State::Snake;
        }
        buffer[fruit.1*WIDTH + fruit.0]  = State::Fruit;
        Self {
            snake,
            fruit,
            score: 0,
            ended: false,
            rng,
            buffer
        }
    }
    
    fn generate_fruit(&mut self) -> (usize,usize) {
        self.rng.generate_fruit(&self.snake)
    }
    
    fn displace(&mut self) -> Result<(), SnakeError>{
        let (head_x,head_y) = *self.snake.get_head_pos();
        let new_head = match &self.snake.direction {
            Dir::Left => {
                if head_x == 0 {
                    return Err(SnakeError::OutOfBounds)
                } else {
                    (head_x-1, head_y)
                }
            },
            Dir::Right => {
                if head_x >= WIDTH-1 {
                    return Err(SnakeError::OutOfBounds)
                } else {
                    (head_x+1, head_y)
                }
            },
            Dir::Up => {
                if head_y == 0 {
                    return Err(SnakeError::OutOfBounds)
                }
                else {
                    (head_x, head_y-1)
                }
            },
            Dir::Down => {
                if head_y >= HEIGHT - 1 {
                    return Err(SnakeError::OutOfBounds)
                } else {
                    (head_x, head_y+1)
                }
            }
        };
        self.buffer[(new_head.1*WIDTH +
                    new_head.0) as usize] = State::Head(self.snake.direction);
        self.buffer[head_y*WIDTH + head_x] = State::Snake;
        self.snake.body.push_front(new_head);
        self.snake.fruited = self.check_eat()?;
        if !self.snake.fruited {
            let tail = self.snake.body.pop_back().unwrap();
            self.buffer[tail.1*WIDTH + tail.0] = State::Empty;
        }
        Ok(())
    }

    fn turn(&mut self, dir:Dir) {
        self.snake.direction = dir;
        let head = self.snake.get_head_pos();
        self.buffer[head.1*WIDTH + head.0] = State::Head(dir);
    }

    fn check_eat(&self) -> Result<bool, SnakeError>{
        let mut copy = self.snake.body.clone();
        let head = copy.pop_front().unwrap();
        if copy.contains(&head) {
            Err(SnakeError::EatSelf)
        } else {
            Ok(head == self.fruit)
        }
    }
        
    pub fn update(&mut self) {
        self.ended = self.displace().is_err();
        if self.snake.fruited {
            self.fruit = self.generate_fruit();
            self.buffer[(self.fruit.1*WIDTH + self.fruit.0) as usize] = State::Fruit;
            self.score += 1;
            crate::get_point(self);
        }
    }
    
    pub fn display(&self) {
        for y in 0..HEIGHT {
            io::_print(&crate::buffer_to_line(self.buffer,y));
        }
        crate::move_cursor(Dir::Left, WIDTH);
        crate::move_cursor(Dir::Up, HEIGHT);
    }
        
    pub fn do_action(&mut self, a: Action) {
        match a {
            Action::Nop => (),
            Action::Turn(dir) => self.turn(dir)
        }
    }
}
