// handler.rs
// handles special characters such as DEL, INS, Ctrl...
// TODO: Also should handle characters in different languages.
//
// calls keybindings as well.

#![no_std]

extern crate watchdog_keys as keys;
extern crate watchdog_ralloc as ralloc;
extern crate ux;

#[macro_use]
mod io;

pub fn handle(in: &str) {
	let mut done: i2 = 0;
	match in {
		"ALT" => {
			done = 1;
		}
		"CTRL" => {
			done = 1;
		}
	}
	
	if done == 0 {
		match in {
			"DEL" => {
			
			}
			"INS" => {

			}
		}
	}
}
