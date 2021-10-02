#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

use ribbon_nostd::SEQUENCE;

mod matrix;
mod millis;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let mut matrix = matrix::Matrix::init(dp);
    matrix.run_ribbon(&SEQUENCE)
}
