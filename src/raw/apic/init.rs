#![no_std]

extern crate rust-cpuid;

mod raw;

// TODO: Call C code to init
extern {

}

pub fn init() {
	cpuid!();
	
	if has_apic {
		
	}
}
