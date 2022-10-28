#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

use panic_halt as _;

mod ribbon;
use ribbon::SEQUENCE;

mod matrix;
mod millis;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let mut matrix = matrix::Matrix::init(dp);
    unsafe { avr_device::interrupt::enable() };
    matrix.run_ribbon(&SEQUENCE)
}
