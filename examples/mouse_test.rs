use std::fs::OpenOptions;
use std::io::Read;
const EV_KEY: u32 = 1;
const EV_ABS: u32 = 3;
const EV_MSC: u32 = 4;
const ABS_X: u32 = 0;
const ABS_Y: u32 = 1;
const ABS_MT_SLOT: u32 = 47;
const ABS_MT_POSITION_X: u32 = 53;
const ABS_MT_POSITION_Y: u32 = 54;
const ABS_MT_TRACKING_ID: u32 = 57;
const SYN: u32 = 0;
fn main() {
    let device = OpenOptions::new()
        .read(true)
        .open("/dev/input/event3")
        .unwrap();
    let mut x = 0 as i32;
    let mut y = 0 as i32;
    let mut down = false;
    let mut first_down = false;
    loop {
        let mut b = (&device).take(24);

        let mut buffer = [0; 24];
        b.read(&mut buffer);
        /*let c = buffer[0];
        let left = (buffer[0] & 0x1) != 0;
        let right = (buffer[0] & 0x2) != 0;
        let middle = (buffer[0] & 0x4) != 0;
        if left {
            println!("left");
        }
        if right {
            println!("right");
        }
        if middle {
            println!("middle");
        }*/
        let code_a = (buffer[17] as u32) << 8 | buffer[16] as u32;
        let code_b = (buffer[19] as u32) << 8 | buffer[18] as u32;
        let value = (buffer[23] as u32) << 24
            | (buffer[22] as u32) << 16
            | (buffer[21] as u32) << 8
            | (buffer[2] as u32);
        if code_a == EV_KEY {
            //println!("EV_KEY {} {}", code_b, value);
        } else if code_a == EV_MSC {
            //  println!("EV_MISC {} {}", code_b, value);
        } else if code_a == EV_ABS {
            if code_b == ABS_X {
                // println!("EV_ABS ABS_X {}", value);
            } else if code_b == ABS_Y {
                // println!("EV_ABS ABS_Y {}", value);
            } else if code_b == ABS_MT_SLOT {
                // println!("EV_ABS ABS_MT_SLOT {}", value);
            } else if code_b == ABS_MT_TRACKING_ID {
                //println!("EV_ABS ABS_MT_TRACKING_ID {}", value);
            } else if code_b == ABS_MT_POSITION_X {
                println!("EV_ABS ABS_MT_POSITION_X {}", value);
            } else if code_b == ABS_MT_POSITION_Y {
                println!("EV_ABS ABS_MT_POSITION_Y {}", value);
            } else {
                //println!("EV_ABS unknown code_b {}", code_b);
            }
        } else if code_a == SYN {
            //  println!("SYN");
        } else {
            //   println!("unknown code_a {}", code_a);
        }
        /*if c == 8 {
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
        }*/
    }
}
