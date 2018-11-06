#![no_std]

extern crate cpuio;
extern crate watchdog_raw as raw;
extern crate ux;

use raw::header;
use raw::ps2_keyboard;

pub fn init_io() -> char {
	get_header();
	
	let mut pic: i2 = 1;
	if pic == 1 {
	
	let character == get_io!();
	return character;
	
	if pic == 1 {
}
