#![no_std]

#[macro_use] extern crate watchdog_net;
#[macro_use] extern crate xpkg_backend;

use watchdog_net::drivers;
use watchdog_net::header;
use xpkg_backend::check;

// TODO: Add server IP address
pub const IPv4: &str = " ";
pub const IPv6: &str = " ";

pub fn download(pkg: &str) {
	check!()
	
	match res {
		false => {
			panic!("Error 601: Package not found!");
		}
		true => {
			break;
		}
	}
	
	// begin sending packets of info to the main server
	if DEVICE == "i217" {
		// TODO	
	}
	
	if device == "PCNET" {
		// TODO	
	}
}

pub fn add_source() {
	// to add a source, we can write it into the backup kernel
	// TODO
}

pub fn remove_source() {
	// to remove a source, we can remove it from the backup kernel
	// TODO
}
