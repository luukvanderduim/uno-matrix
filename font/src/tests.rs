#[cfg(test)]
use super::*;

#[test]
fn font_utf8_nul() {
    let arr = FONT8X8_BASIC.index(0);
    assert_eq!(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], arr);
}

#[test]
fn font_utf8_exclamation() {
    let ch = Character::get('!');
    assert_eq!(
        &[0x18, 0x3C, 0x3C, 0x18, 0x18, 0x00, 0x18, 0x00],
        ch.inner()
    );
}

#[test]
fn font_utf8_x() {
    let ch = Character::get('x');
    assert_eq!(
        &[0x00, 0x00, 0x63, 0x36, 0x1C, 0x36, 0x63, 0x00],
        ch.inner()
    );
}

#[test]
fn font_utf8_y() {
    let ch = Character::get('y');
    assert_eq!(
        &[0x00, 0x00, 0x33, 0x33, 0x33, 0x3E, 0x30, 0x1F],
        ch.inner()
    );
}

#[test]
fn font_utf8_tilde() {
    let ch = Character::get('~');
    assert_eq!(
        &[0x6E, 0x3B, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
        ch.inner()
    );
}

#[test]
fn font_as_columns() {
    let ch = Character::get('5');

    // `.as_columns()` rotates the character by 90 degrees.
    // Lets see if four invocations yield the original!

    let mut rot = ch;
    assert_eq!(rot, ch); // 0 deg.
    assert_ne!(rot.as_columns(), ch); // 90 deg.
    assert_ne!(rot.as_columns(), ch); // 180 deg.
    assert_ne!(rot.as_columns(), ch); // 270 deg.
    assert_eq!(rot.as_columns(), ch); // 360 deg.
}

// cargo test render_test -- --ignored --nocapture
#[ignore]
#[test]
fn render_test() {
    let mut ch = Character::get('5').flip_horizontal();
    ch.render();
    ch.as_columns().render();
}
