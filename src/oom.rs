// oom.rs
// the handler for Out Of Memory errors used by ralloc.

#![no_std]

extern crate watchdog_raw as raw;

use raw::vga;

pub fn handler() -> ! {
  println!("OUT OF MEMORY!");
}
