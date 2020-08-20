fn main() {
    let mut t = 0;
    conifer::run(|frame, pointer, delta_time| {
        for y in 0..frame.height {
            for x in 0..frame.width {
                t += delta_time;
                if t > 10000 {
                    return true;
                }
                frame.set_pixel(
                    x,
                    y,
                    ((x as f32 / frame.width as f32) * 255.0) as u8,
                    ((t as f32 / 1000.0) * 255.0) as u8, //                   ((y as f32 / 255.0) * 255.0) as u8,
                    0 as u8,
                );
            }
        }
        false
    })
}
