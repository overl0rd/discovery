#![deny(unsafe_code)]
#![no_main]
#![no_std]

#[allow(unused_imports)]
use aux11::{entry, iprint, iprintln};
use heapless::Vec;

const BUFFER_FULL_ERR_MSG: &[u8] = b"error: buffer full\n\r";
#[entry]
fn main() -> ! {
    let (usart1, _mono_timer, _itm) = aux11::init();

    // A buffer with 32 bytes of capacity
    let mut buffer: Vec<u8, 32> = Vec::new();

    loop {
        buffer.clear();

        loop {
            while usart1.isr.read().rxne().bit_is_clear() {}
            let input_byte = usart1.rdr.read().rdr().bits() as u8;

            if buffer.push(input_byte).is_err() {
                for byte in BUFFER_FULL_ERR_MSG {
                    while usart1.isr.read().txe().bit_is_clear() {}
                    usart1.tdr.write(|w| w.tdr().bits(u16::from(*byte)));
                }
                break;
            }
            if input_byte == 13 {
                for byte in buffer.iter().rev().chain(&[b'\n', b'\r']) {
                    while usart1.isr.read().txe().bit_is_clear() {}
                    usart1.tdr.write(|w| w.tdr().bits(u16::from(*byte)));
                }
                break;
            }
        }
    }
}
