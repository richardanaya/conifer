use std::time::Instant;

fn main() {
    let t = Instant::now();
    conifer::run(|frame, pointer, delta_time| {
        println!("{} {}", pointer.x, pointer.y);
        for y in 0..frame.height {
            for x in 0..frame.width {
                if t.elapsed().as_millis() > 10000 as u128 {
                    return true;
                }
                frame.set_pixel(
                    x,
                    y,
                    ((x as f32 / frame.width as f32) * 255.0) as u8,
                    ((t.elapsed().as_millis() as f32 / 10000 as f32) * 255.0) as u8, //                   ((y as f32 / 255.0) * 255.0) as u8,
                    0 as u8,
                );
            }
        }
        false
    })
}
