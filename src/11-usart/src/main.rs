#![no_main]
#![no_std]

use core::convert::TryInto;

#[allow(unused_imports)]
use aux11::{entry, iprint, iprintln};

#[entry]
fn main() -> ! {
    let (usart1, _mono_timer, _itm) = aux11::init();

    // Send a single character
    usart1
        .tdr
        .write(|w| w.tdr().bits(u16::from(b'X')) );

    for c in "The quick brown fox jumps over the lazy dog.".chars() {
        while usart1.isr.read().txe().bit_is_clear() {}
        usart1
            .tdr
            .write(|w| unsafe { w.tdr().bits(u16::from(c as u16)) });
    }

    let elapsed = instant.elapsed();

    iprintln!(
        &mut itm.stim[0],
        "`for` loop took {} ticks ({} us)",
        elapsed,
        elapsed as f32 / mono_timer.frequency().0 as f32 * 1e6
    );
    loop {}
}
