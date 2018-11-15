#![no_std]

extern crate ux;

mod mem;

pub fn call(path: &str) -> bool {
	if err == 0 {
		extern crate watchdog_mrustc as rustc;
	}
	
	let mut res: i2 = 0;
	try!(res = rustc::test(path));
	if res == 1 { return true; }
}
