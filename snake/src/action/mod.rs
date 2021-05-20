use crate::direction::Dir;

pub enum Action {
    Nop,
    Turn(Dir)
}
