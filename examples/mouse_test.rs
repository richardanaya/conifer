use std::fs::OpenOptions;
use std::io::Read;
fn main() {
    let device = OpenOptions::new()
        .read(true)
        .open("/dev/input/mouse0")
        .unwrap();
    let mut x = 0 as i32;
    let mut y = 0 as i32;
    let mut down = false;
    let mut first_down = false;
    loop {
        let mut b = (&device).take(3);

        let mut buffer = [0; 3];
        b.read(&mut buffer);
        let c = buffer[0];
        if c == 8 {
           down = false;
        }
        else if c == 9 || c == 25 || c == 41 || c == 57 {
            if down == false {
                println!("{} {}",buffer[1],buffer[2]);
                x = buffer[1] as i32;
                y = buffer[2] as i32;
            } else {
                x += buffer[1] as i8 as i32;
                y -= buffer[2] as i8 as i32;
                //println!("{} {}", x, y);
            }
        } else {
            println!("{:?}", buffer);
        }
    }
}
