// scheduler.rs
// the scheduling algorithm + handler

#![no_std]

extern crate atomic_hashmap;
extern crate watchdog_thread__spawn;
extern crate watchdog_ralloc as ralloc;

// local
#[macro_rules] mod context;
mod thread_stack;

macro_rules! init_scheduler {
	($id:expr, $msg:expr, $pri:expr, $cpus:expr) => {{
		let mut err: i2 = 1;
        if $id == 0 {
			// initialize scheduler
            let mut cpu_stack = HashMap::New();
            let mut cpu_time_stack = HashMap::New();
			let mut cpu_thread_stack: = HashMap::New();
            let mut hashmap_started = true;
			let mut items;
            err = 0;
        }
		if $id == 1 {
			// add to scheduler
            if hashmap_started == true { err = 0; }
            if err == 1 { panic!("Error adding to HashMap!"); }

            cpu_time_stack.insert[x, $msg];
            cpu_stack.insert[$pri, $msg];
			items = items + 1;
		}
		if $id == 2 {
			// remove from scheduler
            if hashmap_started == true { err = 0; }
            if err == 1 { panic!("Error remvoing from HashMap!"); }
			
			item = item - 1;
        } if $id == 3 {
			// add thread to scheduler	
		}
	}};
}

pub fn run_scheduler(thread: Thread, task: Task, pri: i8) {
	init_scheduler!(0);
	let no_thread = Thread {
		id: "NOP",
		_cpu: 0,
		local_mem: 0,
		task: "NOP",
		priority: "NOP",
	}
	
	if thread == no_thread { break; }
	let mut pos = 0;
	
	for x in 0..items {
		pos = pos + 1;
		cur = cpu_thread_stack[pos];
		if cur == thread { break; }
	}
	
	if cur == thread {
		break;
	} else { panic!("Thread not found!"); }
}
