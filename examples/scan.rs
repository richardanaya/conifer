use evdev::*;
use std::fs::File;
use std::os::unix::io::FromRawFd;


fn main() {
    let dev = evdev::enumerate();
    // look through all the devices
    for d in dev.iter() {
        // if it supports absolute events
        if d.events_supported().contains(ABSOLUTE) {
            // look through all its bits
            for idx in 0..0x3f {
                let abs = 1 << idx;
                // if bit for index is true, get the absolute state values for that idx
                if (d.absolute_axes_supported().bits() & abs) == 1 {
                    println!("name: {}",d.name().clone().into_string().unwrap());
                    let file = unsafe { File::from_raw_fd(d.fd()) };
                    println!("file: {:#?}",file);
                   println!("abs: {:#?}", &d.state().abs_vals[idx as usize]);
                }
            }
        }
    }
}
