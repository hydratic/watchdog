#![no_std]

extern crate watchdog_raw as raw;
extern crate ux;

use raw::cpu;

pub struct _Thread {
	id: i8,
	_cpu: i8,
	local_mem: i32,
	task: &str,
	priority: i8,
}

pub fn thread_spawn(id: i8, thread_local_mem: i8, Task: &str, prioirty: i8) -> Thread {
	let used_ids = get_used_ids(1);
	let used_id_num = get_used_ids(2);
	let mut err: i2 = 0;
	let mut item = used_ids[1];
	let mut pos = 1;
        
    for x in 0..used_id_num {
       	item = used_ids[pos]
		if item == $id { err = 1; }
    }
	
	if err == 0 {
		let thread: Thread {
			id: id,
			_cpu: get_cpu(id);
			local_mem: thread_local_mem,
			task: Task,
			priority: priority,
		};
		
		thread_stack__insert(thread);
		return thread;
	}
}

pub fn kill_thread(id: i8, sudo: i2) { thread_stack__remove(thread, sudo); }
