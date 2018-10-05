#![no_std]

extern crate core as std;

extern crate cpuio;
extern crate hashmap_core;
extern crate ux;

mod memory;
mod modules;

#[macro_use] mod lex;

pub const ERR: &str = "none";
pub const EXIT: i2 = 0;

// TODO: Document Each Error
pub struct ERRORS {
	// BROKE_VM_ERR: i2,
	STRAY_CHAR_ERR: i2,
	THREAD_ERR: i2,
	TYPE_ERR: i2,
}

pub fn init() { modules::load_modules(); }

macro_rules! parse {
	() => {{
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
				"syntax" => {
				
				}
			}
		}
		
		if ERR == "yes" {
			unsafe {
				EXIT += 1;
				
				
			}
		}
		
		if ERR == "yes" { break; }
	}};
}

pub fn interp() {
	init();
	
	parse!(FILE, PATH);
	
	if EXIT == 0 {
	
	} else if  {
		
	}
}
