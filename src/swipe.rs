use crate::point::*;
use crate::streamed_data::*;

#[derive(Clone, Debug)]
pub struct Swipe {
    pub points: Vec<Point>,
    pub finished: bool,
}

// self.points ought not to be empty.
impl Swipe {
    pub fn new(origin: Point) -> Swipe {
        Swipe {
            points: vec![origin],
            finished: false,
        }
    }

    pub fn end(&mut self) {
        self.finished = true;
    }

    pub fn push(&mut self, point: Point) {
        // drop late comers (is this supposed to happen?)
        if self.last().time < point.time {
            self.points.push(point);
        }
    }

    pub fn last(&self) -> &Point {
        &self.points[self.points.len() - 1]
    }

    pub fn vector(&self) -> (usize, usize) {
        let (first, last) = (&self.points[0], self.last());
        (last.x - first.x, last.y - first.y)
    }

    // overkill? use floats for Timeval?
    pub fn duration(&self) -> Timeval {
        let (first, last) = (self.points[0].time, self.last().time);
        let micros = last.1 - first.1;
        Timeval(
            last.0 - first.0 - if micros < 0 { 1 } else { 0 },
            micros.rem_euclid(1000000),
        )
    }
}

pub enum SwipeFragment {
    PointFragment(PointFragment),
    End,
}

#[derive(Clone, Debug)]
pub struct StreamedSwipe {
    pub swipe: Option<Swipe>,
    pub streamed_point: StreamedPoint,
}

impl Default for StreamedSwipe {
    fn default() -> Self {
        StreamedSwipe {
            swipe: None,
            streamed_point: StreamedPoint::Nothing,
        }
    }
}

impl StreamedData<Swipe> for StreamedSwipe {
    type Fragment = SwipeFragment;

    fn update(self, fragment: Self::Fragment) -> StreamedState<Self, Swipe> {
        match fragment {
            SwipeFragment::PointFragment(ptfrag) => {
                match self.streamed_point.clone().update(ptfrag) {
                    StreamedState::Complete(pt) | StreamedState::Standalone(pt) => {
                        if let Some(swipe) = self.swipe {
                            let mut updated_swipe = swipe.clone();
                            updated_swipe.push(pt);
                            StreamedState::Standalone(updated_swipe)
                        } else {
                            StreamedState::Standalone(Swipe::new(pt))
                        }
                    }
                    StreamedState::Incomplete(ipt) => StreamedState::Incomplete(StreamedSwipe {
                        swipe: self.swipe.clone(),
                        streamed_point: ipt,
                    }),
                }
            }
            SwipeFragment::End => {
                if let Some(swipe) = self.swipe {
                    let mut updated_swipe = swipe.clone();
                    updated_swipe.end();
                    StreamedState::Complete(updated_swipe)
                } else {
                    StreamedState::Incomplete(StreamedSwipe::default())
                }
            }
        }
    }
}
