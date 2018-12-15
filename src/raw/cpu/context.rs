// context.rs
// provides an interface to context switching
//
// Note on the Design:
//
// watchdog handles context switching via the programs.
// each working watchdog binary should call a function, main.
// when switching between tasks, main is the function that is called.

#![no_std]

extern crate ux;
extern crate watchdog_pkg_backend as pkg;
extern crate watchdog_ralloc as ralloc;
extern crate watchdog_utils as utils;
extern crate watchdog_mrustc_comp as mrustc_comp;
extern crate watchdog_telephone as telephone;

mod thread;
mod sleep;

pub fn context_switch(switch_to: &str, lang: &str, compiled: bool) {
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
			// NOTE: the 'lang' variable serves as a switch_to in this function
			watchdog_utils::context_handler::handle(lang);
		}
	}
}
