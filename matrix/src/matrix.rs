//! `Matrix` and `ShiftRegister` form the meat and bones of this program.
//! All logic is conveniently tied to these structs' impls.

use arduino_hal::{
    hal::port::{self, Dynamic},
    port::mode::Output,
};
use arduino_hal::{port::Pin, Peripherals};
use bit_field::BitField;
use panic_halt as _;

use crate::millis::millis as now;
use crate::millis::millis_init;

/// Represents the 74HC595 shift register.
struct ShiftRegister {
    clk: Pin<Output, Dynamic>,
    data: Pin<Output, Dynamic>,
    latch: Pin<Output, Dynamic>,
}

impl ShiftRegister {
    /// 'Feeds' a row - represented by a single `u8` - into the 74HC595 shift register.
    fn feed(&mut self, r: u8) {
        self.latch.set_low();
        for i in 0..=7 {
            self.clk.set_low();
            match r.get_bit(7 - i) {
                true => self.data.set_high(),
                false => self.data.set_low(),
            };
            self.clk.set_high();
        }
        self.latch.set_high();
    }
}

/// The 8x8 Matrix component with all relevant behavior for this project.
pub struct Matrix {
    rows: ShiftRegister,
    columns: [port::Pin<Output, Dynamic>; 8],
}

/// The functionality needed to operate the 8x8 LED display.
/// in the context of this toy program.
impl Matrix {
    pub fn init(dp: Peripherals) -> Self {
        let pins = arduino_hal::pins!(dp);

        let rows = ShiftRegister {
            clk: pins.d10.into_output().downgrade(),
            data: pins.d11.into_output().downgrade(),
            latch: pins.d12.into_output().downgrade(),
        };

        let columns = [
            pins.d2.into_output().downgrade(),
            pins.d3.into_output().downgrade(),
            pins.d4.into_output().downgrade(),
            pins.d5.into_output().downgrade(),
            pins.d6.into_output().downgrade(),
            pins.d7.into_output().downgrade(),
            pins.d8.into_output().downgrade(),
            pins.d9.into_output().downgrade(),
        ];

        millis_init(dp.TC0);

        Matrix { rows, columns }
    }

    pub fn select_row_and_column(&mut self, row: usize, col: usize) {
        // Select appropriate column:
        (0_usize..=7)
            .into_iter()
            .filter(|idx| *idx != col)
            .for_each(|idx| self.columns.get_mut(idx).unwrap().set_high());
        self.columns.get_mut(col).unwrap().set_low();

        // Select approriate row:
        // NOTE:
        // NOT efficient as this is 8 times slower than feeding the whole row at once.+
        let mut value = 0_u8;
        value.set_bit(row, true);

        self.rows.feed(value);
    }

    /// Run the ribbon forever
    pub fn run_ribbon(&mut self, ribbon: &[u8]) -> ! {
        let panorama = ribbon.windows(8).cycle();

        // The duration (in ms) a frame is to be lit.
        //  The iterator will progress to the next view in the 'panorama'.
        static FRAMEDUR: u32 = 120;

        for frame in panorama {
            let later = now() + FRAMEDUR;

            while later > now() {
                //  0:  0 0 1 0 0 0 0 0
                // 1:   0 0 1 1 0 0 0 0
                // 2:   0 0 1 1 0 0 0 0
                // 3:   0 0 1 1 0 0 0 0
                // 4:   0 0 1 1 1 0 0 0
                // 5:   0 0 1 1 1 0 0 0
                // 6:   0 0 1 1 0 0 0 0
                // 7:   0 0 1 1 1 1 1 1
                for row in 0..8 {
                    //  0:  0 0 1 0 0 0 0 0

                    let row_data = *frame.get(row).unwrap();
                    for col in 0..8 {
                        if row_data.get_bit(col) {
                            self.select_row_and_column(row, col);
                        }
                    }
                }
            }
        }

        // The return type is `!`, if we should end up here we `panic!()`.
        panic!();
    }
}
