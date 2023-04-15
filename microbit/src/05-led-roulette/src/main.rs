#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use rtt_target::rtt_init_print;
use panic_rtt_target as _;
use microbit::{
    board::Board,
    display::blocking::Display,
    hal::timer::Timer,
    hal::prelude::*,
};

const DURATION_MS: u32 = 30;

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);
    let mut led_grid = [
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
    ];

    let mut row = 0;
    let mut col = 0;

    loop {
        // 1. Print the current configuration of LED grid at (row, col)
        display.show(&mut timer, led_grid, DURATION_MS);

        // 2. Turn off the pin at the current (row, col) –- set the position to 0
        led_grid[row][col] = 0;

        // 3. Clear the display and delay for 1000ms
        display.clear();
        timer.delay_ms(DURATION_MS);

        // 4. Calculate the next pin to light up by modifying `row` and `col` variables
        match (row, col) {
            (0, c) if c < 4 => col += 1,
            (0, c) if c == 4 => row = 1,
            (r, 4) if r < 4 => row += 1,
            (4, c) if c > 0 => col -= 1,
            (r, 0) if r > 0 => row -= 1,
            (_, _) => {
                row = 0;
                col = 0;
            }
        }

        // 5. Update the LED grid at (row, col) to be 1
        led_grid[row][col] = 1;

        // 6. Continue back up to the top of the loop
    }
}
