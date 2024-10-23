#![no_main]
#![no_std]

use rp_pico::entry;
use panic_halt as _;
use embedded_hal::digital::{InputPin, OutputPin};
use rp_pico::hal::pac;
use rp_pico::hal;
use rp_pico::hal::adc::AdcPin;
///use embedded_hal::adc::OneShot;

#[entry]
fn does_not_have_to_be_main() -> ! {
    let mut pac = pac::Peripherals::take().unwrap();
    let sio = hal::Sio::new(pac.SIO);
    let pins = rp_pico::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );
    let mut led_pin = pins.led.into_push_pull_output();
    led_pin.set_high().unwrap();

    let mut adc = hal::Adc::new(pac.ADC, &mut pac.RESETS);
    let mut adc_pin_0 = pins.gpio26.into_floating_input();

    loop {
        let adc_pin = adc_pin_0.unwrape(); // Unwrap the Result to get the AdcPin
       // adc.wait_ready();
       
        let light_level: u16 = adc.read(&mut adc_pin).unwrap();
        
        // You can use the light_level value here
        // For example, you could print it if you had a way to output data
        // or use it to control the LED brightness
        
        if light_level > 2000 {
            led_pin.set_high().unwrap();
        } else {
            led_pin.set_low().unwrap();
        }
    }
}
