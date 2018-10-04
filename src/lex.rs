#![no_std]

extern crate cpuio;
extern crate hashmap_core;
extern crate ux;

pub const ERR: i2 = 0;
pub const FILE: &str = "empty";
pub const TARGET: &str = ":c//";

pub fn lex() {
	let mut token_length;
	
	loop {
		let token = match char {
			// insert code fold
			" " => "space",
			
			"1" => "int",
			"2" => "int",
			"3" => "int",
			"4" => "int",
			"5" => "int",
			"6" => "int",
			"7" => "int",
			"8" => "int",
			"9" => "int",
			"0" => "int",
			
			"a" => "char",
			"b" => "char",
			"c" => "char",
			"d" => "char",
			"e" => "char",
			"f" => "char",
			"g" => "char",
			"h" => "char",
			"i" => "char",
			"j" => "char",
			
			// end code fold
		}
		
		token_length = token_length + 1;
		
		// cry havoc and let loose the ifs of war
		if token_length > 3 {
			if tokens == "strings" {
				match long_token {
					_ => "var",
				}
			}
			
			if tokens == "ints" { break; }
		}
		
		if token == "int" {
				
		}
		
		if token == "char" {
			
		}
	}
}
