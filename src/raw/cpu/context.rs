// context.rs
// provides an interface to context switching
//
// Note on the Design:
//
// watchdog handles context switching via the programs.
// each working watchdog binary should have a function, main.
// when switching between tasks, main is the function that is called.
//
// each binary should also have a function called answer, reserved for use
// by the kernel. this function is called by telephone and halts the other
// processes. then, main is called in the binary that we wish to switch to.

#![no_std]

extern crate ux;

// crates for compiling
extern crate watchdog_mrustc_comp as mrustc_comp;
extern crate watchdog_telephone as telephone;

use mrustc_comp::*;

// required to run properly
mod mem;

// call the outside world
// mod pkg;

mod thread;
mod sleep;

pub fn context_switch(addr: u32, switch_to: &str, lang: &str, compiled: bool) {
	let mut err: i2 = 1;
	
	mem_addr_map!(2, addr);

	if res == "used" {
		err = 1;
	} else { err = 0; }
	
	// now we need to jump to this binary
	// from the binary we are in
	match switch_to {
		"path" => {
            if compiled == false {
			    if lang == "Rust" {
                    // compile the rust code into a crate
				    rustc_comp::comp(path, 1);

                    // call the crate
					telephone::call(path);

                    // execute the crate
                    rustc_comp::call("dev/gen/rust/crate.rs");
                }
            } else { panic!("Compiled binaries are not supported yet!"); }
		}
		"sys" => {
			if task == "net" {
				// TODO
			}
            if task == "security" {
                // TODO
            }
            if task == "alloc" {
                // TODO
            }
            if task == "tfs" {
                // TODO
            }
		}
	}
}
