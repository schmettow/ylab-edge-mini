#![no_std]
#![no_main]

use arduino_hal::prelude::*;
use panic_halt as _;

use arduino_hal::adc;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial 
        = arduino_hal::default_serial!(dp, pins, 115_200);

    let mut adc = arduino_hal::Adc::new(dp.ADC, Default::default());

    let (_vbg, _gnd, _tmp) = (
        adc.read_blocking(&adc::channel::Vbg),
        adc.read_blocking(&adc::channel::Gnd),
        adc.read_blocking(&adc::channel::Temperature),
    );
    /* ufmt::uwriteln!(&mut serial, "Vbandgap: {}", vbg).void_unwrap();
    ufmt::uwriteln!(&mut serial, "Ground: {}", gnd).void_unwrap();
    ufmt::uwriteln!(&mut serial, "Temperature: {}", tmp).void_unwrap();*/

    let a0 = pins.a0.into_analog_input(&mut adc);
    let a1 = pins.a1.into_analog_input(&mut adc);
    let a2 = pins.a2.into_analog_input(&mut adc);
    let a3 = pins.a3.into_analog_input(&mut adc);
    let a4 = pins.a4.into_analog_input(&mut adc);
    let a5 = pins.a5.into_analog_input(&mut adc);

    let mut tick: i16 = 0;

    loop {
        // Arduino Nano has two more ADC pins A6 and A7.  Accessing them works a bit different from
        // the other pins as they are not normal IO pins.  The code below shows how it works.
        let (v6, v7) = (
            adc.read_blocking(&adc::channel::ADC6),
            adc.read_blocking(&adc::channel::ADC7),
        );
        let values = [
            a0.analog_read(&mut adc),
            a1.analog_read(&mut adc),
            a2.analog_read(&mut adc),
            a3.analog_read(&mut adc),
            a4.analog_read(&mut adc),
            a5.analog_read(&mut adc),
            v6, v7
        ];
        tick += 1;
        ufmt::uwriteln!(&mut serial, 
            "{},0,{},{},{},{},{},{},{},{}", 
            tick, values[0], values[1], values[2], values[3], 
            values[4], values[5], values[6], values[7],).void_unwrap();
        serial.flush();

        /* for (_, value) in values.iter().enumerate() {
            ufmt::uwrite!(&mut serial, "{},", value).void_unwrap();
        }

        ufmt::uwrite!(&mut serial, "{},{}", a6, a7).void_unwrap();
        ufmt::uwriteln!(&mut serial, "").void_unwrap();*/
        arduino_hal::delay_ms(5);
    }
}