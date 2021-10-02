mod ribbon;
pub use crate::ribbon::SEQUENCE;

#[cfg(test)]
mod tests {
    use super::*;

    fn render(c: u8) {
        print!("\t\t");
        for i in 0..7 {
            print!("{}", if (c >> (7 - i)) & 0x01 == 1 { "@" } else { "." });
        }
        print!("\n");
    }

    #[test]
    fn render_sequence() {
        for c in SEQUENCE {
            render(c);
        }
    }
}
