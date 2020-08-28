use evdev::*;
use std::fs::File;
use std::os::unix::io::FromRawFd;

fn main() {
    let dev = evdev::enumerate();
    // look through all the devices
    for d in dev.iter() {
        // if it supports absolute events
        if d.events_supported().contains(ABSOLUTE) {
            // if it supports x and y axis
            let first_axis = 1 << 0;
            if (d.absolute_axes_supported().bits() & first_axis) == 1 {
                let x_abs_val = &d.state().abs_vals[0 as usize];
                let y_abs_val = &d.state().abs_vals[1 as usize];
                println!("{:#?}", &d.state().abs_vals);
                println!("name: {}", d.name().clone().into_string().unwrap());
                let file = unsafe { File::from_raw_fd(d.fd()) };
                println!("file: {:#?}", file);
                println!("abs: {:#?}", x_abs_val);
                println!("abs: {:#?}", y_abs_val);
            }
        }
    }
}
