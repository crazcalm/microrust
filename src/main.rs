#![no_std]
#![no_main]

extern crate microbit;

use panic_halt as _;

use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    let _y = 10;
    let x = 42;
    let _y = x;
    loop {}
}
