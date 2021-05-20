use alloc::collections::VecDeque;
use crate::direction::Dir;

#[derive(Clone)]
pub struct Snake {
    pub body: VecDeque<(usize,usize)>,
    pub fruited: bool,
    pub direction: Dir,
}
impl Snake {    
    pub fn init(origin: (usize,usize), direction:Dir, length:usize) -> Self {
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
    pub fn get_head_pos(&self) -> &(usize,usize) {
        self.body.front().unwrap()
    }
}
