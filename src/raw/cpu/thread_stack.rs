#![no_std]

extern crate atomic_hashmap;
extern crate ux;

mod thread;
mod consts;

#[macro_export]
macro_rules! thread_stack {
	($init:expr, $run:expr) => {{
		if $init == 0 {
			// create initial hashmap
			let mut thread_stack: PERSISTENT_HashMap<Thread, Task>;
			if $run == 0 { break; }
			thread_stack__insert(SYS_THREAD, SYS_PERMISSIONS);
			
			// assign thread stack to a thread
			task!("thread_stack", SYS_THREAD);
		} else if $init == 1 {
			// create local hashmap
			if 
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
