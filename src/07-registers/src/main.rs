#![no_main]
#![no_std]

use aux7::stm32::gpioa::brr::BR11_AW;
#[allow(unused_imports)]
use aux7::{entry, iprintln, RegisterBlock, ITM};

#[entry]
fn main() -> ! {
    let gpioe = aux7::init().1;

    // Turn on the North LED
    gpioe.bsrr.write(|w| w.bs9().set_bit());

    // Turn on the East LED
    gpioe.bsrr.write(|w| w.bs11().set_bit());

    // Turn off the North LED
    gpioe.bsrr.write(|w| w.br9().set_bit());

    // Turn off the East LED
    gpioe.bsrr.write(|w| w.br11().set_bit());

    gpioe.bsrr.write(|w| w.br12().set_bit());
    loop {}
}
