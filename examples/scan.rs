use evdev::*;
use std::fs::File;
use std::os::unix::io::FromRawFd;


fn main() {
    let dev = evdev::enumerate();
    for d in dev.iter() {
        if d.events_supported().contains(ABSOLUTE) {
            for idx in 0..0x3f {
                let abs = 1 << idx;
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