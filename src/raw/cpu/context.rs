#![no_std]

extern crate ux;

// crates for compiling
extern crate cplusplus_comp;
extern crate mrustc_comp;

// required to run properly
mod mem;

mod thread;
mod sleep;

pub fn context_switch(addr: u32, switch_to: &str) {
	let mut err: i2 = 1;
	
	mem_addr_map!(2, addr);

	if res == "used" {
		err = 1;
	} else { err = 0; }
	
	// now we need to jump to this binary
	// from the binary we are in
	match switch_to {
		"path" => {
			if lang == "Rust" {
				mrustc::comp(path, 1);
				use mrustc_comp::*;
			}
			
			if lang == "C++" {
				cplusplus::comp(path);
				use mrustc_comp::*;
			}
		}
		"sys" => {
			if task == "net" {
				// TODO
			}
		}
	}
}
