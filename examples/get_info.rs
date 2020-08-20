fn main() {
    let mut t = 0;
    conifer::run(|frame, pointer, delta_time| {
        println!("{} {}",frame.width,frame.height);
        true
    })
}
