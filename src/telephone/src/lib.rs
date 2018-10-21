#![no_std]

extern crate ux;

mod mem;
mod fs;

pub fn call(path: &str) {
	// first check to ensure that this crate exists
	// TODO
	
	if err == 0 {
		extern crate ccomp_local;
	}
	
	try!(let res = ccomp_local::test(1));
	// if res == 2 { log.out("Code called successfully.")
}
