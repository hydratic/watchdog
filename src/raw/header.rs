#![no_std]

extern crate watchdog_ralloc;
extern crate watchdog_fs;

use fs::ahci;
use fs::atapi;

// mod apic;
mod cpu;
mod hpet;
mod pci;
mod pic;
mod ps2_keyboard;
mod ps2_mouse;
mod vga;

pub fn header() i32 -> {
  // ahci
  let ahci = fs::probe_ports();
}
