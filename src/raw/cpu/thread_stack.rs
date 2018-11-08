#![no_std]

extern crate atomic_hashmap;
extern crate ux;

mod thread;

#[macro_rules]
macro_rules! thread_stack {
	($init:expr, $id:expr, $permissions:expr) => {{
		if $init == 0 {
			// create initial hashmap
		} else if $init == 1 {
			// create local hashmap
		}
	}}; 
}

pub fn thread_stack__insert(thread: Thread, permissions: i8) {
	thread_stack!(100);
	__threads__.insert[thread, permissions];
}

pub fn thread_stack__remove(thread: Thread, sudo: i2) {
	thread_stack!(100);
	if sudo == 1 {
		__threads__.remove[thread, permissions];
	} else {
		print!("SUDO required to spawn thread!");
	}
}
