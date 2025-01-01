// This code connects to Microbit v1 and controls the LED lights
// It uses a different system to address the 5x5 LED display than
// Microbit v2. It uses a 9x3 matrix system that needs to be
// mapped to the 5x5 LED display.
#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _;
use microbit::Peripherals;
use cortex_m::asm::nop;

#[entry]
fn main() -> ! {
    // Connect to Microbit peripherals like LED, pins, etc.
    let p = Peripherals::take().unwrap();
    
    // Configure all pins for columns as outputs
    p.GPIO.pin_cnf[10].write(|w| w.dir().output());
    p.GPIO.pin_cnf[11].write(|w| w.dir().output());
    p.GPIO.pin_cnf[12].write(|w| w.dir().output());
    p.GPIO.pin_cnf[4].write(|w| w.dir().output());
    p.GPIO.pin_cnf[5].write(|w| w.dir().output());
    p.GPIO.pin_cnf[6].write(|w| w.dir().output());
    p.GPIO.pin_cnf[7].write(|w| w.dir().output());
    p.GPIO.pin_cnf[8].write(|w| w.dir().output());
    p.GPIO.pin_cnf[9].write(|w| w.dir().output());

    // Configure all pins for rows as outputs    
    p.GPIO.pin_cnf[13].write(|w| w.dir().output());
    p.GPIO.pin_cnf[14].write(|w| w.dir().output());
    p.GPIO.pin_cnf[15].write(|w| w.dir().output());

    // Function to set a specific LED
    fn set_led(p: &Peripherals, row: usize, col: usize) {
        // Array with column pins
        let cols = [10, 11, 12, 4, 5, 6, 7, 8, 9];
        // Array with row pins
        let rows = [13, 14, 15];
        
        // Mapping from 5x5 to 9x3
        let (mapped_row, mapped_col) = match (row, col) {
            (0, 0) => (0, 3),
            (0, 1) => (1, 6),
            (0, 2) => (0, 4),
            (0, 3) => (1, 7),
            (0, 4) => (0, 5),
            (1, 0) => (2, 6),
            (1, 1) => (2, 7),
            (1, 2) => (2, 8),
            (1, 3) => (2, 0),
            (1, 4) => (2, 1),
            (2, 0) => (1, 4),
            (2, 1) => (0, 2),
            (2, 2) => (1, 5),
            (2, 3) => (2, 2),
            (2, 4) => (1, 3),
            (3, 0) => (0, 1),
            (3, 1) => (0, 0),
            (3, 2) => (0, 8),
            (3, 3) => (0, 7),
            (3, 4) => (0, 6),
            (4, 0) => (2, 5),
            (4, 1) => (1, 0),
            (4, 2) => (2, 3),
            (4, 3) => (1, 8),
            (4, 4) => (2, 4),
            _ => (0, 0), // Default case, should not happen
        };
        
        // Reset all column pins by setting them high
        p.GPIO.outset.write(|w| unsafe { w.bits((1 << 10) | (1 << 11) | (1 << 12) | (1 << 4) | (1 << 5) | (1 << 6) | (1 << 7) | (1 << 8) | (1 << 9)) });
        
        // Reset all row pins by setting them low
        p.GPIO.outclr.write(|w| unsafe { w.bits((1 << 13) | (1 << 14) | (1 << 15)) });
        
        // Set the specified column pin low
        p.GPIO.outclr.write(|w| unsafe { w.bits(1 << cols[mapped_col]) });
        // Set the specified row pin high
        p.GPIO.outset.write(|w| unsafe { w.bits(1 << rows[mapped_row]) });
    }
    
    // Set the LED in row 3 (index 2) and column 5 (index 4)
    //set_led(&p, 3, 2);
    
    loop {
        // Keep the program in an infinite loop
       
        // Nested loop to traverse the LED lights row by row
        for row in 0..5 {
            for col in 0..5 {
                set_led(&p, row, col);
                for _ in 0..100_000 {
                    nop();
                }
            }
        }
    }
}