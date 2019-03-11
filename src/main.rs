extern crate ev3dev_lang_rust;

use std::io::Result;
use ev3dev_lang_rust::led::{ COLOR_AMBER, COLOR_GREEN, Led };
use ev3dev_lang_rust::touch_sensor::TouchSensor;

fn main() -> Result<()> {

    let mut led = Led::new().unwrap();
    let mut touch_sensor = TouchSensor::find().unwrap();
    
    loop {
        let is_pressed = touch_sensor.get_pressed_state()?;
        let colour = if is_pressed { COLOR_AMBER } else { COLOR_GREEN };
        led.set_color(colour)?;
    }
}

