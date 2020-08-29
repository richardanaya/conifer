use crate::point::*;
use crate::swipe::*;
use std::ops::Div;

#[derive(Clone, Debug)]
pub enum Gesture {
    Tap(Point),
    Drag(Point, Point),
}

impl Swipe {
    pub fn narrow_enough(&self, spread: usize) -> Option<(isize, isize)> {
        if let (Some(minx), Some(maxx), Some(miny), Some(maxy)) = (
            self.points.iter().map(|p| p.x).min(),
            self.points.iter().map(|p| p.x).max(),
            self.points.iter().map(|p| p.y).min(),
            self.points.iter().map(|p| p.y).max(),
        ) {
            if (maxx as isize - minx as isize <= spread as isize
                && maxy as isize - miny as isize <= spread as isize)
            {
                return Some((
                    (maxx as isize + minx as isize).div(2),
                    (maxy as isize + miny as isize).div(2),
                ));
            }
        }
        None
    }

    pub fn tap(&self, spread: usize) -> Option<Gesture> {
        if self.finished {
            self.narrow_enough(20).and_then(|(x, y)| {
                Some(Gesture::Tap(Point {
                    x,
                    y,
                    time: self.duration(),
                }))
            })
        } else {
            None
        }
    }

    pub fn drag(&self) -> Option<Gesture> {
        Some(Gesture::Drag(
            self.points[0].clone(),
            self.points[self.points.len() - 1].clone(),
        ))
    }
}
