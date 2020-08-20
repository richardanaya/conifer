use std::fs::OpenOptions;
use std::io::Read;
fn main() {
    let device = OpenOptions::new()
        .read(true)
        .open("/dev/input/mouse2")
        .unwrap();

    loop {
        let mut b = (&device).take(3);

        let mut buffer = [0;3];
        b.read(&mut buffer);

        println!("{:?}", buffer);
    }
}
