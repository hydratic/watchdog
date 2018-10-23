#![no_std]

extern crate x86_64;

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}
