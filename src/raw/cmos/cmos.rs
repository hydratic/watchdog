#![no_std]

extern crate ux;

extern {
	pub fn get_update_in_process_flag_c();
	pub fn get_RTC_register_c();
}

pub static CURRENT_YEAR: i4 = 2018;

macro_rules! read_rtc {
	() => {{
		// init
		let mut century: u32;
		let mut last_second: u32;
		let mut last_minute: u32;
		let mut last_hour: u32;
		let mut last_day: u32;
		let mut last_year: u32;
		let mut last_century: u32;
		let mut registerB: u32;
		
		// TODO: Rest of CMOS
		// LINK: http://techdocs.altium.com/display/FPGA/PS2+Keyboard+Scan+Codes
		
      	// if (!(registerB & 0x02) && (hour & 0x80)) {
       	//     hour = ((hour & 0x7F) + 12) % 24;
      	// }
		
		// Calculate the full (4-digit) year
    	if(century_register != 0) {
    		year += century * 100;
    	} else {
        	year += (CURRENT_YEAR / 100) * 100;
        	if(year < CURRENT_YEAR) year += 100;
		}
	}};
}
