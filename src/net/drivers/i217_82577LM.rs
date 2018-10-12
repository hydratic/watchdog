#![no_std]

extern crate ux;
extern crate volatile;

use volatile::Volatile;

mod e1000;

// TODO:
// Test
// Add C-style types in arguments
extern {
    pub fn MMIOUtils::read8(p_address: u64) -> Volatile<u8>;
    pub fn MMIOUtils::read16(p_address: u64) -> Volatile<u16>;
    pub fn MMIOUtils::read32(p_address: u64) -> Volatile<u32>;
    pub fn MMIOUtils::read64(p_address: u64) -> Volatile<u64>;
    
    pub fn MMIOUtils::write8(p_address: u64) -> Volatile<u8>;
    pub fn MMIOUtils::wrtie16(p_address: u64) -> Volatile<u16>;
    pub fn MMIOUtils::write32(p_address: u64) -> Volatile<u32>;
    pub fn MMIOUtils::write64(p_address: u64) -> Volatile<u64>;
    
    pub fn Ports::outportb(p_port: u16, p_data: u8);
    pub fn Ports::outportw(p_port: u16, p_data: u16);
    pub fn Ports::outportl(p_port: u16, p_data: u32);
    pub fn Ports::inportb(p_port: u16);
    pub fn Ports::inportw(p_port: u16);
    pub fn Ports::inportl(p_port: u16);
    
    pub fn E1000::readCommand(p_address: u16);
    pub fn E1000::readMACAddress() -> bool;
}

pub const INTEL_VEND: Volatile<u32> = 0x8086  // Vendor ID for Intel 
pub const E1000_DEV: Volatile<u32> = 0x100E;  // Device ID for the e1000 Qemu, Bochs, and VirtualBox emmulated NICs
pub const E1000_I217: Volatile<u32> = 0x153A;  // Device ID for Intel I217
pub const E1000_82577LM: Volatile<u32> = 0x10EA;  // Device ID for Intel 82577LM
 
// I have gathered those from different Hobby online operating systems instead of getting them one by one from the manual
 
pub const REG_CTRL: Volatile<u32> = 0x0000;
pub const REG_STATUS: Volatile<u32> = 0x0008;
pub const REG_EEPROM: Volatile<u32> = 0x0014;
pub const REG_CTRL_EXT: Volatile<u32> = 0x0018;
pub const REG_IMASK: Volatile<u32> = 0x00D0;
pub const REG_RCTRL: Volatile<u32> = 0x0100;
pub const REG_RXDESCLO: Volatile<u32> = 0x2800;
pub const REG_RXDESCHI: Volatile<u32> = 0x2804;
pub const REG_RXDESCLEN: Volatile<u32> = 0x2808;
pub const REG_RXDESCHEAD: Volatile<u32> = 0x2810;
pub const REG_RXDESCTAIL: Volatile<u32> = 0x2818;
 
pub const REG_TCTRL: Volatile<u32> = 0x0400;
pub const REG_TXDESCLO: Volatile<u32> = 0x3800;
pub const REG_TXDESCHI: Volatile<u32> = 0x3804;
pub const REG_TXDESCLEN: Volatile<u32> = 0x3808;
pub const REG_TXDESCHEAD: Volatile<u32> = 0x3810;
pub const REG_TXDESCTAIL: Volatile<u32> = 0x3818;
 
pub const REG_RDTR: Volatile<u32> = 0x2820; // RX Delay Timer Register
pub const REG_RXDCTL: Volatile<u32> = 0x3828; // RX Descriptor Control
pub const REG_RADV: Volatile<u32> = 0x282C; // RX Int. Absolute Delay Timer
pub const REG_RSRPD: Volatile<u32> = 0x2C00; // RX Small Packet Detect Interrupt
 
pub const REG_TIPG: Volatile<u32> = 0x0410;     // Transmit Inter Packet Gap
pub const ECTRL_SLU: Volatile<u32> = 0x40;        //set link up
 
pub const RCTL_EN: u32 = (1 << 1);    // Receiver Enable
pub const RCTL_SBP: u32 = (1 << 2);   // Store Bad Packets
pub const RCTL_UPE: u32 = (1 << 3);    // Unicast Promiscuous Enabled
pub const RCTL_MPE: u32 = (1 << 4);    // Multicast Promiscuous Enabled
pub const RCTL_LPE: u32 = (1 << 5);   // Long Packet Reception Enable
pub const RCTL_LBM_NONE: u32 = (0 << 6);    // No Loopback
pub const RCTL_LBM_PHY: u32 = (3 << 6);    // PHY or external SerDesc loopback
pub const RTCL_RDMTS_HALF: u32 = (0 << 8);    // Free Buffer Threshold is 1/2 of RDLEN
pub const RTCL_RDMTS_QUARTER: u32 = (1 << 8);    // Free Buffer Threshold is 1/4 of RDLEN
pub const RTCL_RDMTS_EIGHTH: u32 = (2 << 8);    // Free Buffer Threshold is 1/8 of RDLEN
pub const RCTL_MO_36: u32 = (0 << 12);   // Multicast Offset - bits 47:36
pub const RCTL_MO_35: u32 = (1 << 12);   // Multicast Offset - bits 46:35
pub const RCTL_MO_34: u32 = (2 << 12);   // Multicast Offset - bits 45:34
pub const RCTL_MO_32: u32 = (3 << 12);   // Multicast Offset - bits 43:32
pub const RCTL_BAM: u32 = (1 << 15);   // Broadcast Accept Mode
pub const RCTL_VFE: u32 = (1 << 18);   // VLAN Filter Enable
pub const RCTL_CFIEN: u32 = (1 << 19);   // Canonical Form Indicator Enable
pub const RCTL_CFI: u32 = (1 << 20);   // Canonical Form Indicator Bit Value
pub const RCTL_DPF: u32 = (1 << 22);   // Discard Pause Frames
pub const RCTL_PMCF: u32 = (1 << 23);   // Pass MAC Control Frames
pub const RCTL_SECRC: u32 = (1 << 26);   // Strip Ethernet CRC
 
// Buffer Sizes
pub const RCTL_BSIZE_256: u32 = (3 << 16);
pub const RCTL_BSIZE_512: u32 = (2 << 16);
pub const RCTL_BSIZE_1024: u32 = (1 << 16);
pub const RCTL_BSIZE_2048: u32 = (0 << 16);
pub const RCTL_BSIZE_4096: u32 = ((3 << 16) | (1 << 25));
pub const RCTL_BSIZE_8192: u32 = ((2 << 16) | (1 << 25));
pub const RCTL_BSIZE_16384: u32 = ((1 << 16) | (1 << 25));
 
// Transmit Command
 
pub const CMD_EOP: u32 = (1 << 0);    // End of Packet
pub const CMD_IFCS: u32 = (1 << 1);    // Insert FCS
pub const CMD_IC: u32 = (1 << 2);    // Insert Checksum
pub const CMD_RS: u32 = (1 << 3);    // Report Status
pub const CMD_RPS: u32 = (1 << 4);    // Report Packet Sent
pub const CMD_VLE: u32 = (1 << 6);    // VLAN Packet Enable
pub const CMD_IDE: u32 = (1 << 7);    // Interrupt Delay Enable
 
// TCTL Register
 
pub const TCTL_EN: u32 = (1 << 1);    // Transmit Enable
pub const TCTL_PSP: u32 = (1 << 3);    // Pad Short Packets
pub const TCTL_CT_SHIFT: u32 = 4;           // Collision Threshold
pub const TCTL_COLD_SHIFT: u32 = 12;          // Collision Distance
pub const TCTL_SWXOFF: u32 = (1 << 22);   // Software XOFF Transmission
pub const TCTL_RTLCL: u32 = (1 << 24);   // Re-transmit on Late Collision
 
pub const TSTA_DD: u32 = (1 << 0);    // Descriptor Done
pub const TSTA_EC: u32 = (1 << 1);    // Excess Collisions
pub const TSTA_LC: u32 = (1 << 2);    // Late Collision
pub const LSTA_TU: u32 = (1 << 3);    // Transmit Underrun

pub const E1000_NUM_RX_DESC: i8 = 32;
pub const E1000_NUM_TX_DESC: i3 = 8;

pub struct e1000_rx_desc {
    addr: Volatile<u64>,
    length: Volatile<u16>,
    checksum: Volatile<u16>,
    status: Volatile<u8>,
    errors: Volatile<u8>,
    special: Volatile<u16>,
}

pub struct e1000_tx_desc {
    addr: Volatile<u64>,
    length: Volatile<u16>,
    cso: Volatile<u8>,
    cmd: Volatile<u8>,
    status: Volatile<u8>,
    css: Volatile<u8>,
    special: Volatile<u8>,
}
