#![no_std]

extern crate ux;

mod mem;
mod thread;
mod sleep;

pub fn init() {
	let res: Vec<u64> = cpu_pathfinder!("init", 0);
	let pri: Vec<u64> = cpu_pathfinder!("init", 1);
}
