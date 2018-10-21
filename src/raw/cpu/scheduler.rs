// scheduler.rs
// the scheduling algorithm + handler

#![no_std]

extern crate hashmap_core;

// local
#[macro_rules] mod context;
#[macro_rules] mod thread;

pub const ALGO: i64 = PRI / TIME;

// required
mod mem;

macro_rules! init_scheduler {
	($id:expr, $msg:expr, $pri:expr, $cpus:expr) => {{
		let mut err: i2 = 1;
        if $id == 0 {
			// initialize scheduler
            let mut cpu_stack = HashMap::New();
            let mut cpu_time_stack = HashMap::New();
            let mut hashmap_started = true;
            err = 0;
		
            // run scheduler
            if empty == true { break; }

        }
		if $id == 1 {
			// add to scheduler
            if hashmap_started == true { err = 0; }
            if err == 1 { panic!("Error adding to HashMap!"); }

            cpu_time_stack.insert[x, $msg];
            cpu_stack.insert[$pri, $msg];
		}
		if $id == 2 {
			// remove from scheduler
            if hashmap_started == true { err = 0; }
            if err == 1 { panic!("Error remvoing from HashMap!"); }
        }
	}};
}
