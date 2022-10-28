//!
//! Lets the sequence of bytes be generated from the sentence at compile time.
//! Creates src/ribbon.rs, which represents a ribbon of chained characters,
//! as encoded by the 8x8 font.
//!

use font::Character;

use std::fs::File;
use std::io::prelude::*;
use std::io::BufWriter;
use std::path::Path;

const SENTENCE: &str = "Ferris Rocks!";
const LEN: usize = SENTENCE.len() * 8;

fn main() -> std::io::Result<()> {
    let mut byte_seq = String::new();

    for c in SENTENCE.chars() {
        let byte_block: String = Character::get(c)
            .rotate90()
            .inner()
            .iter()
            .map(|b| format!("\t\t{:#010b},\n", b))
            .collect();
        byte_seq = byte_seq + &byte_block + "\n";
    }

    let path = Path::new("src/").join("ribbon.rs");
    let matrix_path = Path::new("../matrix/src/").join("ribbon.rs");
    let mut f = BufWriter::new(File::create(&path)?);
    let mut h = BufWriter::new(File::create(&matrix_path)?);
    write!(
        f,
        "pub const SEQUENCE: [u8; {}] = [\n{}\n];\n",
        LEN, byte_seq
    )?;

    write!(
        h,
        "pub const SEQUENCE: [u8; {}] = [\n{}\n];\n",
        LEN, byte_seq
    )
}
