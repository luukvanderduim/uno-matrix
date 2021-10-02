//! `Matrix` and `ShiftRegister` form the meat and bones of this program.
//! All logic is conveniently tied to these structs' impls.

use arduino_hal::{
    delay_ms,
    hal::port::{self, Dynamic},
    port::mode::Output,
};
use arduino_hal::{port::Pin, Peripherals};
use bit_field::BitField;
use panic_halt as _;

// TODO:
// General question: Wouldn't we rather have a 'global' clock and have the whole thing
// be orchestrated by that? Downside, it creates overhead, will see if its needed.

struct ShiftRegister {
    clk: Pin<Output, Dynamic>,
    data: Pin<Output, Dynamic>,
    latch: Pin<Output, Dynamic>,
}

impl ShiftRegister {
    /// Gets a `Window` matrix column,
    /// Takes each byte and feeds it to the shift register.
    pub fn feed(&mut self, col: &u8) {
        self.latch.set_low();
        for i in 0..=7 {
            self.clk.set_low();
            match col.get_bit(7 - i) {
                false => self.data.set_low(),
                true => self.data.set_high(),
            }
            self.clk.set_high();
        }
        self.latch.set_high();
    }
}

/// The 8x8 Matrix component with all relevant behavior for this project.
pub struct Matrix {
    columns: ShiftRegister,
    rows: [port::Pin<Output, Dynamic>; 8],
}

/// The functionality needed to operate the 8x8 LED display.
/// in the context of this toy program.
impl Matrix {
    pub fn init(dp: Peripherals) -> Self {
        let pins = arduino_hal::pins!(dp);
        let shift_register = ShiftRegister {
            clk: pins.d10.into_output().downgrade(),
            data: pins.d11.into_output().downgrade(),
            latch: pins.d12.into_output().downgrade(),
        };
        let rows = [
            pins.d2.into_output().downgrade(),
            pins.d3.into_output().downgrade(),
            pins.d4.into_output().downgrade(),
            pins.d5.into_output().downgrade(),
            pins.d6.into_output().downgrade(),
            pins.d7.into_output().downgrade(),
            pins.d8.into_output().downgrade(),
            pins.d9.into_output().downgrade(),
        ];
        crate::millis::millis_init(dp.TC0);
        Matrix {
            columns: shift_register,
            rows,
        }
    }

    /// Lights each row for a certain duration in ms.
    fn light_each_row(&mut self, dur: u16) {
        self.rows.iter_mut().for_each(|row| {
            row.set_high();
            delay_ms(dur);
            row.set_low();
        });
    }

    /// Run the ribbon forever
    pub fn run_ribbon(&mut self, ribbon: &[u8]) -> ! {
        let windows = ribbon.windows(8).cycle();

        // Because `windows` is a cyclic - thus endless - iterator,
        // I assume this `for` loop, to loop forever.
        for window in windows {
            let now = crate::millis::millis();

            // TODO: Wrapping safety!?
            while crate::millis::millis() < (now + 25) {
                for column in window {
                    self.columns.feed(column);
                    self.light_each_row(1);
                }
            }
        }

        // We promised not to return, so if we end up here:
        panic!();
    }
}
