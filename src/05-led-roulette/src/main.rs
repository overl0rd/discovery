#![deny(unsafe_code)]
#![no_main]
#![no_std]

use aux5::{entry, Delay, DelayMs, LedArray, OutputSwitch};

#[entry]
fn main() -> ! {
    let (mut delay, mut leds): (Delay, LedArray) = aux5::init();

    loop {
        for led in 0..8 {
            let next = (led + 1) % 8;

            leds[next].on().ok();
            delay.delay_ms(50_u16);
            leds[led].off().ok();
            delay.delay_ms(150_u16);
        }
    }
}
