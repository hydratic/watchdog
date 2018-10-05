#![no_std]

extern crate core as std;

extern crate average;
extern crate cpuio;
extern crate hashmap_core;
extern crate memadvise;
extern crate ux;

mod memory;
mod modules;

#[macro_use] mod lex;

pub const EXIT: i2 = 0;

pub struct ERRORS {
	// Broke VM ERR: 
	// Means that program written causes
	// a fatal error to a VM and is not safe to
	// interpret.
	//
	// BROKE_VM_ERR: i2,
	//
	// Insecure ERR:
	// Means that the program contains data that is not secure.
	//
	// INSECURE_ERR: i2,
	//
	// Memory Overflow ERR:
	//
	MEM_OVERFLOW_ERR: i2,
	STRAY_CHAR_ERR: i2,
	THREAD_ERR: i2,
	TYPE_ERR: i2,
}

pub fn init() { modules::load_modules(); }

macro_rules! parse {
	() => {{
		let mut commands = hashmap_core::New();
		let mut line_token;
		let mut x: i8 = 1;
		
		lex!();
		
		loop {
			line = file[x];
			let (token1, token2, token3, token4, token5, token6, token7) = line.split_at_mut(" ")
			x = x + 1;
			
			match token1 {
				"char" => {
					unsafe {
						ERR += 1;
					}
				}
				"int" => {

				}
				"syntax" => {
				
				}
				"var" => {
					
				}
			}
		}
		
		if ERR == "yes" {
			unsafe {
				
			}
		}
		
		if ERR == "yes" { break; }
	}};
}

pub fn interp() {
	init();
	parse!(FILE, PATH);
	
	if ERR == "none" {
		// execute the code as read from the cmd hashmap
	} else if  {
		panic!("ERR: {}, {}" ERR_CODE, ERR);
	}
}
