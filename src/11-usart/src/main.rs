#![no_main]
#![no_std]

#[allow(unused_imports)]
use aux11::{entry, iprint, iprintln};

#[entry]
fn main() -> ! {
    let (usart1, mono_timer, itm) = aux11::init();

    // Send a single character

    for c in "The quick brown fox jumps over the lazy dog.".chars() {
        usart1
            .tdr
            .write(|w| unsafe { w.tdr().bits(u16::from(c as u16)) });
    }

    loop {}
}
