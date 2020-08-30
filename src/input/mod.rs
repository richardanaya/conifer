use crate::point::Timeval;

pub mod event_input;

pub enum InputEvent {
    PartialX(isize, Timeval),
    PartialY(isize, Timeval),
    ButtonDown(usize),
    Unknown,
}
