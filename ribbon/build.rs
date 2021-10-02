//!
//! Lets the sequence of bytes be generated from the sentence at compile time.
//! Creates src/ribbon.rs, which represents a ribbon of chained characters,
//! as encoded by the 8x8 font.
//!

use font::Character;

use std::io::prelude::*;
use std::path::Path;
use std::{fs::File, io::BufWriter};

const SENTENCE: &str = "Luuk is hip!";
const LEN: usize = SENTENCE.len() * 8;

fn main() -> std::io::Result<()> {
    let mut byte_seq = String::new();

    for c in SENTENCE.chars() {
        let byte_block: String = Character::get(c)
            .as_columns()
            .flip_horizontal()
            .inner()
            .iter()
            .map(|b| format!("\t\t{:#010b},\n", b))
            .collect();
        byte_seq = byte_seq + &byte_block + "\n";
    }

    let path = Path::new("src/").join("ribbon.rs");
    let nostd_path = Path::new("../ribbon_nostd/src/").join("ribbon.rs");
    let mut f = BufWriter::new(File::create(&path)?);
    let mut g = BufWriter::new(File::create(&nostd_path)?);
    write!(
        f,
        "pub const SEQUENCE: [u8; {}] = [\n{}\n];\n",
        LEN, byte_seq
    )?;
    write!(
        g,
        "pub const SEQUENCE: [u8; {}] = [\n{}\n];\n",
        LEN, byte_seq
    )
}
