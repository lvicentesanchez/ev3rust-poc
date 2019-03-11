extern crate ev3dev_lang_rust;

use std::io::Result;
use ev3dev_lang_rust::led::{ COLOR_AMBER, COLOR_GREEN, Led };
use ev3dev_lang_rust::touch_sensor::TouchSensor;

fn main() -> Result<()> {

    let mut led = Led::new().unwrap();
    let mut touch_sensor = TouchSensor::find().unwrap();
    
    loop {
        if touch_sensor.get_pressed_state()? {
            led.set_color(COLOR_AMBER)?;
        } else {
            led.set_color(COLOR_GREEN)?;
        }
    }
}

