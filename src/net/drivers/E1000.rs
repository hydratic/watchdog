#![no_std]

extern crate ux;
extern crate volatile;

// mod other_stuff;

macro_rules! eepromRead {
	($addr:expr) => {{
		let mut data: u16 = 0;
		let mut tmp: u32 = 0;
		if eerprom_exists == true {
			writeCommand( REG_EEPROM, (1) | addr << 8);
			loop {
				if tmp = readCommand(REG_EEPROM) {
					if tmp == (1 << 1) {
						break;
					}
				} else {
				
				}
			}
		}		
	}};
}
