#![no_std]

pub struct cr0 {
	protection_enable: u64,
	monitor_coprocessor: u64,
	emulate_fpu: u64,
	task_switched: u64,
	extension_type: u64,
	numeric_error: u64,
	reserved_1: u64,
	write_protect: u64,
	reserved_2: u64,
	alignment_mask: u64,
	reserved_3: u64,
	not_write_through: u64,
	cache_disable: u64,
	paging_enable: u64,
	flags: u64,
}

pub struct cr3 {
	reserved_1: u64,
	page_level_write_through: u64,
	page_level_cache_through: u64,
	reserved_2: u64,
	address_of_page_directory: u64,
	flags: u64,
}
