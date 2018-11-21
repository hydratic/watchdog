// smp.rs
// a WIP implementation of multi-processor support

#![no_std]

pub struct mp_floating_point_structure {
	signature: &str,
	configuration_table: u32,
	length: u8,
	mp_specification_revision: u8,
	checksum: u8,
	default_configuration: u8,
	features: u32,
}

pub struct mp_configuration_table {
	signature: &str,
	length: u16,
	mp_specification_revision: u8,
	checksum: u8,
	oem_id: &str,
	product_id: &str,
	oem_table: u32,
	oem_table_size: u16,
}
