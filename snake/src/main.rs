#![no_std]
#![feature(start)]
#![no_main]

use ferr_os_librust::{io,
                      syscall,
                      interfaces::keyboard};


mod action;
mod direction;
mod errors;
mod state;
mod snake;
mod point_generator;
mod game;

use action::Action;
use direction::Dir;
use errors::SnakeError;
use state::State;
use snake::Snake;
use point_generator::PointGenerator;
use game::Game;

extern crate alloc;
extern crate rand;

use alloc::{collections::vec_deque::VecDeque,
            string::String,
            format,
            vec::Vec
            };
use rand::RngCore;

pub const WIDTH: usize = 59;
pub const HEIGHT: usize = 20;
pub const SIZE : usize = WIDTH as usize * HEIGHT as usize;

#[no_mangle]
pub extern "C" fn _start(heap_address: u64, heap_size: u64, args: u64, args_number: u64) {
    ferr_os_librust::allocator::init(heap_address, heap_size);
    let arguments = ferr_os_librust::env::retrieve_arguments(args_number, args);
    main(arguments);
}

fn buffer_to_line(buffer:[State; SIZE], y: usize) -> String {
    let beg = y * WIDTH;
    let end = (y+1) * WIDTH;
    let line = &buffer[beg..end];
    let mut res = String::new();
    for pixel in line {
        res.push_str(&pixel.to_string()[..]);
    }
    res.push('\n');
    res
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
        syscall::set_screen_size(HEIGHT+2,WIDTH+2);
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

fn move_cursor(d:Dir, n: usize) {
    let end = match d {
        Dir::Left => 'D',
        Dir::Right => 'C',
        Dir::Down => 'B',
        Dir::Up => 'A'
    };
    io::_print(&format!("\x1B[{}{}",n,end))
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

