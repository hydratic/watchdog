extern {
	pub fn probe_port(abar: HBA_MEM);
	pub fn check_type(port: HBA_PORT);
	pub fn port_rebase(port: HBA_PORT, portno: i32);
	pub fn start_cmd(port: HBA_PORT);
	pub fn stop_cmd(port: HBA_PORT);
    pub fn read(port: HBA_PORT, startl: u32, starth: u32, count: u32, buf: u16);
}

pub struct HBA_PORT_ {
	
}

pub struct HBA_MEM_ {

}

impl HBA_port for HBA_PORT_ {
    type HBA_PORT = HBA_PORT_;
}

impl HBA_mem for HBA_MEM_ {
	type HBA_MEM = HBA_MEM_;
}
