// mod.rs
// provides calls into the network drivers
// 
// TODO:
// all

#![no_std]

// ** Intel i217 **
extern {
  // ** MMIOUTILS **
  pub fn MMIOUtils::read8(p_address: u64) -> Volatile<u8>;
  pub fn MMIOUtils::read16(p_address: u64) -> Volatile<u16>;
  pub fn MMIOUtils::read32(p_address: u64) -> Volatile<u32>;
  pub fn MMIOUtils::read64(p_address: u64) -> Volatile<u64>;
  
  pub fn MMIOUtils::write8(p_address: u64, p_value: u8);
  pub fn MMIOUtils::write16(p_address: u64, p_value: u16);
  pub fn MMIOUtils::write32(p_address: u64, p_value: u32);
  pub fn MMIOUtils::write64(p_address: u64, p_value: u64);

  // ** PORTS **
  pub fn Portd::outportb(p_port: u16, p_data: u8);
  // TODO
}

// ** AMD PCNET **
// TODO
extern {
  
}
