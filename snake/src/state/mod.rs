use crate::direction::Dir;
use alloc::string::String;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum State {
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
