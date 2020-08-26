use crate::streamed_data::StreamedData;
use crate::streamed_data::StreamedState;
use crate::streamed_data::*;

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug, Copy, Clone)]
pub struct Timeval(i32, i32);

impl Timeval {
    pub fn from_timeval(t: ::libc::timeval) -> Timeval {
        Timeval(t.tv_sec, t.tv_usec)
    }
}

#[derive(Debug)]
pub struct Point {
    time: Timeval,
    pub x: usize,
    pub y: usize,
}

#[derive(Debug)]
pub enum StreamedPoint {
    X(Timeval, usize),
    Y(Timeval, usize),
    Nothing,
}

impl Default for StreamedPoint {
    fn default() -> Self {
        StreamedPoint::Nothing
    }
}

impl StreamedData<Point> for StreamedPoint {
    type Fragment = StreamedPoint;

    fn new() -> Self {
        StreamedPoint::default()
    }

    fn update(self, fragment: Self::Fragment) -> StreamedState<Self, Point> {
        match (&self, &fragment) {
            (&StreamedPoint::X(timex, x), &StreamedPoint::Y(timey, y))
            | (&StreamedPoint::Y(timey, y), &StreamedPoint::X(timex, x)) => {
                if timex == timey {
                    StreamedState::Complete(Point { time: timex, x, y })
                } else {
                    StreamedState::Incomplete(fragment)
                }
            }
            (_, &StreamedPoint::Nothing) => StreamedState::Incomplete(self),
            _ => StreamedState::Incomplete(fragment),
        }
    }
}
