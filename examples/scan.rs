use evdev::*;

fn main() {
    let dev = evdev::enumerate();
    for d in dev.iter() {
        if d.events_supported().contains(ABSOLUTE) {
            for idx in 0..0x3f {
                let abs = 1 << idx;
                if (d.absolute_axes_supported().bits() & abs) == 1 {
                    println!("name: {}",d.name().clone().into_string().unwrap());
                   println!("abs: {:#?}", &d.state().abs_vals[idx as usize]);
                }
            }
        }
    }
}