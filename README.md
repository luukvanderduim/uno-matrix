# uno-matrix

Embedded Rust toy-project to display text, scrolling, on an 8x8 LED matrix display with an Arduino Uno (compatible) board.

## Components

The following components are used:

* an 8x8 LED matrix (Oasis) 1088AS
* an Arduino Uno (compatible)
* a shift register 74HC595
* eight ~220 Ohm resistors

Other requirements include a bread board and a bunch of jumper wires.

## Rust

Uno-matrix is written in Rust. If you wish to try `uno-matrix`, you will need to install Rust.
See [the fine rust-lang web page](https://www.rust-lang.org/) for more information on how to install Rust.

## HAL and prerequisites

Uno-matrix uses the [avr-hal](https://github.com/Rahix/avr-hal) by Rahix, a Hardware Abstraction Layer for AVR microcontroller devices (for example Arduino).

As `uno-matrix` builds on `avr-hal` one should follow its [quickstart](https://github.com/Rahix/avr-hal#quickstart) guide to get going.

## Installation

Clone this repository:

```Bash
git clone https://github.com/luukvanderduim/uno-matrix.git
```

Enter the directory where the binary resides and build and load the binary to the `Atmega 328p`:

```
cd uno-matrix/matrix/
RAVEDUDE_PORT=/dev/ttyUSB0 cargo r
```

Note: Your Arduino (compatible) may be mapped to a different `/dev/ttyXXX`.

## Usage

The `ribbon` crate provides a `build.rs` script which generates the data to be scrolled.
You can alter the text message there too. Just look for:

``` Rust
const SENTENCE: &str = "Ferris Rocks!";
```

For now the `ribbon` crate is kept separate from the `matrix` crate because it uses features from `std` in order to run tests.

## License

Licensed under either of

* Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)

* MIT license
   ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your discretion.
