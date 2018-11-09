#![no_std]

pub struct Task {
	id: &str,
	pri: i8,
	thread: i8,
	_cpu: i8,
}

impl __Task for _Task {
	type Task = _Task;
}
