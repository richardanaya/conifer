use crate::streamed_data::StreamedData;
use crate::streamed_data::StreamedState;

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug, Copy, Clone)]
pub struct Timeval(pub isize, pub isize);

impl Timeval {
    pub fn from_timeval(t: ::libc::timeval) -> Timeval {
        Timeval(t.tv_sec as isize, t.tv_usec as isize)
    }
}

#[derive(Clone, Debug)]
pub struct Point {
    pub time: Timeval,
    pub x: isize,
    pub y: isize,
}

#[derive(Debug, Copy, Clone)]
pub enum StreamedPoint {
    X(Timeval, isize),
    Y(Timeval, isize),
    Nothing,
}

pub type PointFragment = StreamedPoint;

impl Default for StreamedPoint {
    fn default() -> Self {
        StreamedPoint::Nothing
    }
}

impl StreamedData<Point> for StreamedPoint {
    type Fragment = PointFragment;

    fn reset(&mut self) {
        *self = StreamedPoint::Nothing;
    }

    fn update(&mut self, fragment: Self::Fragment) -> StreamedState<Point> {
        match (&self, &fragment) {
            (&&mut StreamedPoint::X(timex, x), &StreamedPoint::Y(timey, y))
            | (&&mut StreamedPoint::Y(timey, y), &StreamedPoint::X(timex, x)) => {
                if timex == timey {
                    self.reset();
                    StreamedState::Complete(Point { time: timex, x, y })
                } else {
                    *self = fragment;
                    StreamedState::Incomplete
                }
            }
            (_, StreamedPoint::Nothing) => StreamedState::Incomplete,
            _ => {
                *self = fragment;
                StreamedState::Incomplete
            }
        }
    }
}
