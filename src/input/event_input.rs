use crate::input::InputEvent;
use crate::point::Timeval;
use evdev::{Device, ABSOLUTE};
use std::path::Path;

const EV_KEY: u16 = 1;
const EV_ABS: u16 = 3;
const ABS_X: u16 = 0;
const ABS_Y: u16 = 1;
const BTN_TOUCH: u16 = 330;

#[derive(Debug)]
pub struct EventInput {
    input_device: Device,
    pub input_min_width: f32,
    pub input_min_height: f32,
    pub input_max_width: f32,
    pub input_max_height: f32,
}

impl EventInput {
    pub fn new<P: AsRef<Path>>(
        path_to_input_device: P,
        input_min_width: f32,
        input_min_height: f32,
        input_max_width: f32,
        input_max_height: f32,
    ) -> Self {
        let input_device = Device::open(&path_to_input_device).unwrap();

        EventInput {
            input_device,
            input_min_width,
            input_min_height,
            input_max_width,
            input_max_height,
        }
    }

    pub fn auto() -> Result<Self, &'static str> {
        let dev = evdev::enumerate();
        // look through all the devices
        for d in dev.into_iter() {
            // if it supports absolute events
            if d.events_supported().contains(ABSOLUTE) {
                // if it supports x and y axis
                let first_axis = 1 << 0;
                if (d.absolute_axes_supported().bits() & first_axis) == 1 {
                    let (x_abs_val, y_abs_val) = {
                        let d_ref = &d;
                        (
                            d_ref.state().abs_vals[0 as usize],
                            d_ref.state().abs_vals[1 as usize],
                        )
                    };

                    return Ok(EventInput {
                        input_device: d,
                        input_min_width: x_abs_val.minimum as f32,
                        input_min_height: y_abs_val.minimum as f32,
                        input_max_width: x_abs_val.maximum as f32,
                        input_max_height: y_abs_val.maximum as f32,
                    });
                }
            }
        }
        Err("Could not find a valid input device")
    }

    pub fn on_event(&mut self, mut f: impl FnMut(InputEvent)) {
        loop {
            for ev in self.input_device.events_no_sync().unwrap() {
                let e = match (ev._type, ev.code, ev.value, ev.time) {
                    (EV_ABS, ABS_X, x, time) => {
                        InputEvent::PartialX(x as isize, Timeval::from_timeval(time))
                    }
                    (EV_ABS, ABS_Y, y, time) => {
                        InputEvent::PartialY(y as isize, Timeval::from_timeval(time))
                    }
                    (EV_KEY, BTN_TOUCH, 0, _) => InputEvent::ButtonDown(0),
                    _ => InputEvent::Unknown,
                };
                f(e);
            }
        }
    }
}
