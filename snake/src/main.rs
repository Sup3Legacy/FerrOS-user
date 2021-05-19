#![no_std]
#![feature(start)]
#![no_main]

use ferr_os_librust::{io,
                      syscall,
                      interfaces::keyboard};
extern crate alloc;
extern crate rand;

use alloc::{collections::vec_deque::VecDeque,
            string::String,
            format,
            vec::Vec
            };
use rand::{distributions::{Distribution, Uniform},
           rngs::SmallRng,
           RngCore,
           SeedableRng
            };

#[no_mangle]
pub extern "C" fn _start(heap_address: u64, heap_size: u64, args: u64, args_number: u64) {
    ferr_os_librust::allocator::init(heap_address, heap_size);
    let arguments = ferr_os_librust::env::retrieve_arguments(args_number, args);
    main(arguments);
}
//80*20
const WIDTH: u16 = 59;
const HEIGHT: u16 = 20;
const SIZE : usize = WIDTH as usize * HEIGHT as usize;

#[derive(Clone,Copy, PartialEq, Eq)]
enum Dir {
    Left,
    Up,
    Right,
    Down
}

enum SnakeError {
    OutOfBounds,
    EatSelf,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum State {
    Empty,
    Head(Dir),
    Snake,
    Fruit
}
impl State {
    #[allow(clippy::inherent_to_string)]
    pub fn to_string(self) -> String {
        String::from(match self {
            Self::Empty => ".",
            Self::Head(Dir::Left) => "\x1B[\x04m<\x1B[\x10m",
            Self::Head(Dir::Up) => "\x1B[\x04m^\x1B[\x10m",
            Self::Head(Dir::Down) => "\x1B[\x04mv\x1B[\x10m",
            Self::Head(Dir::Right) => "\x1B[\x04m>\x1B[\x10m",
            Self::Snake => "\x1B[\x03m0\x1B[\x10m",
            Self::Fruit => "\x1B[\x05m@\x1B[\x10m"
        })
    }
}

enum Action {
    Nop,
    Turn(Dir)
}

#[derive(Clone)]
struct Snake {
    body: VecDeque<(u16,u16)>,
    fruited: bool,
    direction: Dir,
}

struct PointGenerator {
    rng: SmallRng,
    distr_x: Uniform<u16>,
    distr_y: Uniform<u16>,
}

struct Game {
    snake: Snake,
    fruit: (u16,u16),
    score: u16,
    ended: bool,
    rng: PointGenerator,
    buffer: [State; SIZE],
}

fn buffer_to_line(buffer:[State; SIZE], y: u16) -> String {
    let beg = y as usize * WIDTH as usize;
    let end = (y+1) as usize * WIDTH as usize;
    let line = &buffer[beg..end];
    let mut res = String::new();
    for pixel in line {
        res.push_str(&pixel.to_string()[..]);
    }
    res.push('\n');
    res
}

impl Snake {    
    pub fn init(origin: (u16,u16), direction:Dir, length:u16) -> Self {
        let mut body = VecDeque::new();
        let mut current_pos = origin;
        for _ in 0..length {
            body.push_back(current_pos);
            match direction {
                Dir::Left => {
                    current_pos.0 += 1;
                }
                Dir::Right => {
                    current_pos.0 -= 1;
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
            body,
            fruited: false,
            direction
        }
    }
    pub fn get_head_pos(&self) -> &(u16,u16) {
        self.body.front().unwrap()
    }
}
impl PointGenerator {
    pub fn init() -> Self {
        Self{
            rng: SmallRng::seed_from_u64(0),
            distr_x: Uniform::new(0_u16, WIDTH), 
            distr_y: Uniform::new(0_u16, HEIGHT),
        }
    }
    pub fn new_point(&mut self) -> (u16,u16) {
        (self.distr_x.sample(&mut self.rng),
         self.distr_y.sample(&mut self.rng))
    }
    pub fn generate_fruit(&mut self, snake: &Snake) -> (u16,u16) {
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
        let snake = Snake::init((WIDTH/2,HEIGHT/2), Dir::Right, 4);
        let mut rng = PointGenerator::init();
        let fruit = rng.generate_fruit(&snake);
        let mut buffer = [State::Empty; SIZE as usize];
        let mut snake_iter = snake.body.iter();
        let head = snake_iter.next().unwrap();
        buffer[(head.1*WIDTH + head.0) as usize] = State::Head(Dir::Right);
        for (x,y) in snake_iter {
            buffer[(y*WIDTH+x) as usize] = State::Snake;
        }
        buffer[(fruit.1*WIDTH + fruit.0) as usize]  = State::Fruit;
        Self {
            snake,
            fruit,
            score: 0,
            ended: false,
            rng,
            buffer
        }
    }
    
    fn generate_fruit(&mut self) -> (u16,u16) {
        self.rng.generate_fruit(&self.snake)
    }
    
    fn displace(&mut self) -> Result<(), SnakeError>{
        let (head_x,head_y) = *self.snake.get_head_pos();
        let new_head = match &self.snake.direction {
            Dir::Left => {
                if head_x == 0_u16 {
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
        self.buffer[(head_y*WIDTH + head_x) as usize] = State::Snake;
        self.snake.body.push_front(new_head);
        self.snake.fruited = self.check_eat()?;
        if !self.snake.fruited {
            let tail = self.snake.body.pop_back().unwrap();
            self.buffer[(tail.1*WIDTH + tail.0) as usize] = State::Empty;
        }
        Ok(())
    }

    fn turn(&mut self, dir:Dir) {
        self.snake.direction = dir;
        let head = self.snake.get_head_pos();
        self.buffer[(head.1*WIDTH + head.0) as usize] = State::Head(dir);
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
            get_point(self);
        }
    }
    
    pub fn display(&self) {
        io::_print(&String::from("\n"));
        for y in 0..HEIGHT {
            io::_print(&buffer_to_line(self.buffer,y));
        }
        io::_print(&String::from("\n"));
    }
        
    pub fn do_action(&mut self, a: Action) {
        match a {
            Action::Nop => (),
            Action::Turn(dir) => self.turn(dir)
        }
    }
}

fn get_point(_g: &mut Game) {
    unsafe{ io::push_sound(SOUND_FD, 350, 2, 0);
    io::push_sound(SOUND_FD, 500, 2, 2)};
}

fn annoying(g: &mut Game) {
    if unsafe {!MUTE} {
        let pitch = ((g.rng.rng.next_u64()) % 4)*50+ 50;
        if g.rng.rng.next_u64() %4 < 2 {
            unsafe{io::push_sound(SOUND_FD, pitch, 2, 0)}
        }
    }
}

fn loose(_g: &mut Game) {
    unsafe{io::push_sound(SOUND_FD, 500, 3, 0)};
    unsafe{io::push_sound(SOUND_FD, 400, 3, 3)};
    unsafe{io::push_sound(SOUND_FD, 300, 3, 6)};
    unsafe{io::push_sound(SOUND_FD, 200, 8, 9)};
}

static mut SOUND_FD : u64 = 0_u64;
static mut MUTE: bool = false;


#[inline(never)]
fn main(args: Vec<String>) {
    if args.len() > 3 {
        io::_print(&String::from("Got too many arguments, try -h for help\n"));
        return;
    } else if args.len() == 3 {
        if &args[1] == "-h" {
            io::_print(&String::from("-S Mute background music\n"));
            io::_print(&String::from("-h Show this\n"));
            return;
        } else if &args[1] == "-S" {
            unsafe { MUTE = true };
        } else {
            io::_print(&String::from("Incorrect argument, try -h for help\n"));
            return;
        }
    }
    
    unsafe {
        let fd = syscall::open(&String::from("/hard/screen"), io::OpenFlags::OWR);
        syscall::set_layer(10);
        syscall::dup2(io::STD_OUT, fd);
        syscall::close(fd);
        syscall::set_screen_size((HEIGHT+2) as usize,WIDTH as usize);
        syscall::set_screen_pos(1,0);
        SOUND_FD = syscall::open(&String::from("/hard/sound"), io::OpenFlags::OWR) as u64;
    }
    let mut game = Game::init(); 
    game.display();
    main_loop(&mut game);
    end_screen(&mut game);
}

fn get_inputs() -> String {
    let v = io::read_input(io::STD_IN, 512);
    let mut begin = String::new();
    let mut _end = String::new();
    keyboard::translate(v, &mut begin, &mut _end);
    begin
}

fn char_to_action(c:char) -> Action {
    match c.to_ascii_lowercase() {
        'q' => Action::Turn(Dir::Left),
        's' => Action::Turn(Dir::Down),
        'd' => Action::Turn(Dir::Right),
        'z' => Action::Turn(Dir::Up),
        _ => Action::Nop,
    }
}

fn sleep(n: usize) {
    for _ in 0..n {
        unsafe { syscall::sleep() }
    }
}

fn main_loop(g:&mut Game) {
    while !g.ended {
        sleep(75);
        for c in get_inputs().chars() {
            g.do_action(char_to_action(c));
        }
        g.update();
        annoying(g);
        g.display();
    }
}

fn end_screen(g: &mut Game) {
    loose(g);
    for _ in 0..HEIGHT{
        io::_print(&String::from("\n"));
    }
    io::_print(&String::from(
        "     ----------==========GAME OVER==========----------     \n"));
    io::_print(&format!("Collected {} fruit",g.score));
    loop {
        if !get_inputs().is_empty(){
            return
        }
    }
}

