#![no_std]

mod memory;

pub struct PCI_DEVICE_STRUCTURE {
	device_id: u32,
	vendor_id: u32,
	status: u32,
	command: u32,
	class_code: u32,
	subclass: u32,
	prog_if: u32,
	revision_id: u32,
	bist: u32,
	header_type: u32,
	latency_timer: u32,
	cache_line_size: u32,
	
	cardbus_cis_pointer: u32,
	interrupt_line: u32,
	interrupt_pin: u32,
	max_latency: u32,
	min_grant: u32,
	capabilities_pointer: u32,
}

// -----------------------------------
// Configuration Space Access Mechanism #1
// -----------------------------------

// -----------------------------------
// TODO: Document IO port
// 31 - Enable Bit
// 30 - 24 - Reserved
// 23 - 16 - Bus Number
// 15 - 11 - Device Number
// 10 - 8 - Function Number
// 7 - 2 - Register Number
// 1 - 0 - 00
// -----------------------------------
 
 
// Configuration Space Access Mechanism #1

pub fn pciConfigReadWord(bus: u8, slot: u8, func: u8, offset: u8) {
	let mut address: u32;
	
	let mut lbus: u32 = bus;
	let mut lslot: u32 = slot;
	let mut lfunc: u32 = func;
	
	let mut tmp: i16 = 0;
	
	// TODO
}

pub fn pciCheckVendor(bus: u8, slot: u8) {
	let mut vendor: u16;
	let mut device: u16;
	
	// TODO
}

// -----------------------------------
// Configuration Space Access Mechanism #2
// DEPRECATED; included for legacy support
// limited to 80486 and early Pentium motherboards
// -----------------------------------

// -----------------------------------
// IO port(s): 0x0CF8
// 7 - 4 - Key (0 = access mechanism disabled, non-zero = access mechanism enabled)
// 3 -1 - Function number
// 0 - Special cycle enabled if set
// -----------------------------------

// -----------------------------------
// IO port(s): 0xC000 to 0xCFFF
// 15 - 12 - Must be 1100b
// 11 - 8 - Device number 
// 7 - 2 - Register index
// 1 - 0 - Must be zero
// -----------------------------------
