fn main() {
    let mut t = 0;
    conifer::run(|frame, pointer, delta_time| {
        for y in 0..frame.height {
            for x in 0..frame.width {
                t += delta_time;
                frame.set_pixel(
                    x,
                    y,
                    ((x as f32 / 255.0) * 255.0) as u8,
 0,//                   ((y as f32 / 255.0) * 255.0) as u8,
                    0 as u8,
                );
            }
        }
        true
    })
}
