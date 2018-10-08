#![no_std]

extern crate hashmap_core;
extern crate libm;
extern crate smoltcp;
extern crate ux;
extern crate volatile;

mod data;
mod raw;

macro_rules! start_server {
	($net:expr, $ret:expr, $msg:expr) => {{
		if $ret == 0 {
			// init server
			let mut og_map = HashMap::New();
			let mut temp_map = HashMap::New();
	
			loop {
				if gen == 1 {
					for x in 0..TOTAL_SECTORS {
    					data = probe_sector!(y);
						read_line(1, data);
						og_map.insert(y, data);
						last_checked = raw::time::Get();
						y = y + 1;
						if y >= TOTAL_SECTORS { break; }
					}
				} else {
					for x in 0..TOTAL_SECTORS {
    					data = probe_sector!(y);
						read_line(1, data);
						txt = uncompress!(first_line);
						if txt >> last_checked { change = 1; }
						temp_map.insert(y, data);
						last_checked = raw::time::Get();
						y = y + 1;
						if y >= TOTAL_SECTORS { break; }
						if change == 1 { og_map.insert(y, data);
					}
				}
			}
		} else if $ret == 1 {
			// if server is recieving a message
			pub const NUM_SYM_RET: i32 = $msg;
		} else if $ret == 2 {
			// if server is being accessed by an outside force
		}
	}};
}

macro_rules! result {
	($ret:expr) => {{
		start_server!(NET, 1, $ret);
	}};
}
