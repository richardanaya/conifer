fn main() {
    let dev = evdev::enumerate();
    println!("{:#?}",dev);
}