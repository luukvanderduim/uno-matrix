mod font8x8_basic;
mod tests;

use std::ops::Index;

pub use font8x8_basic::FONT8X8_BASIC;

/// The 8x8 font Data Type consists of 8 bytes, 8 bits each.
/// The type has a number of methods to manipulate its data.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Character([u8; 8]);

impl Character {
    /// Translates a given `char` into its 8x8 font type counterpart.
    ///
    /// # Panics
    /// Will panic on any `char` with UTF8 encodings outside of the range `U+0000`..=`U+007F`.
    /// This function is limited to chars with single byte UTF8 encodings.
    pub fn get(c: char) -> Self {
        let mut utf8_byte = [0];
        c.encode_utf8(&mut utf8_byte);
        let array: [u8; 8] = *FONT8X8_BASIC.index(utf8_byte[0] as usize);
        Character(array)
    }

    pub fn inner(&self) -> &[u8; 8] {
        &self.0
    }

    /// Flip the byte over horizontally.
    /// Such that the Lsb becomes the Msb.
    /// Example: 1100 1000 => 0001 0011
    pub fn flip_horizontal(mut self) -> Self {
        for u in self.0.iter_mut() {
            *u = u.reverse_bits();
        }
        self
    }

    /// Manipulates the font data to go from rowwise to columnwise ordering.
    ///
    /// One Could also say the represented character is rotated by 90 degrees.
    pub fn as_columns(&mut self) -> Character {
        let mul = [128, 64, 32, 16, 8, 4, 2, 1];
        let mut cols = [0u8; 8];
        for (i, col) in cols.iter_mut().enumerate() {
            self.0
                .iter()
                .zip(mul)
                .for_each(|(v, m)| *col += ((v >> (7 - i)) & 0x01) * m);
        }
        cols.reverse();
        self.0 = cols;
        self.clone()
    }

    /// On all bytes, each bit is evaluated left-to-right.
    ///
    /// If set, print something fat, otherwise, print something thin.
    ///
    /// ``` Bash
    ///     @@@@@@.
    ///     @@.....
    ///     @@@@@..
    ///     ....@@.
    ///     ....@@.
    ///     @@..@@.
    ///     .@@@@..
    ///     .......
    ///     .......
    /// ```
    pub fn render(&self) {
        self.0.iter().for_each(|c| {
            print!("\t\t");
            for i in 0..7 {
                print!("{}", if (c >> (7 - i)) & 0x01 == 1 { "@" } else { "." });
            }
            println!();
        });
    }
}
