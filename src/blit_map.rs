use crate::canvas::Canvas;

pub struct BlitMap {
    pub map: Vec<bool>,
}

impl BlitMap {
    pub fn from_canvas(canvas: &Canvas) -> BlitMap {
        return BlitMap {
            map: vec![true; canvas.width * canvas.height],
        };
    }

    pub fn from_canvas_with_alpha(canvas: &Canvas) -> BlitMap {
        let mut b = vec![false; canvas.width * canvas.height];
        for x in 0..canvas.width {
            for y in 0..canvas.height {
                let b_index = (y * canvas.width) + x;
                let cur_index = b_index;
                if canvas.pixels[cur_index + 3] > 0 {
                    b[b_index] = true;
                }
            }
        }
        BlitMap { map: b }
    }
}
