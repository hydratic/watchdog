#![no_std]

mod mem;

pub const UCODE_MAGIC: Volatle<u32> = 0x0a4c5749;

pub struct tfd_queue2 {
	tx_cmd_byte_count: u16,
	station_idx: u16,
}

pub struct tx_scheduler_entry {
	tfd_queue: tfd_queue2,
}

pub struct tx_ring_entry {
	lo_addr: u32,
	hi_addr: u16,
	tx_buf_len: u16,
}

pub struct tx_cmd{
	hdr: cmd_hdr,
	data: &str,
}

pub struct tx_cmd_wide {
	hdr: cmd_hdr_wide,
	data: &str,
}

pub struct cmd_hdr {
	opcode: u8,
	flags: u8,
	index: u8,
	queue_id: u8,
}

pub struct cmd_hdr_wide {
	opcode: u8,
	group_id: u8,
	index: u8,
	queue_id: u8,
	length: u16,
	rsv: u8,
	ver: u8,
}

pub struct rx_status {
	closed_rb_num: u16,
	closed_frame_num: u16,
	finished_rb_num: u16,
	finished_frame_num: u16,
	rsv: u32,
}

pub struct ucode_hdr {
	zr0: u32,
	magic: u32,
	name: u8, // TODO: Fix this
	version: u32,
	build: u32,
	rsv: u64,
	data: u8,
}

pub struct ucode_tlv {
	type: u32,
	length: u32,
	data: u8,
}

#[repr(C)]
pub enum ucode_tlv_type {
   UCODE_TLV_INVALID = 0,
   UCODE_TLV_INST = 1,
   UCODE_TLV_DATA = 2,
   UCODE_TLV_INIT = 3,
   UCODE_TLV_INIT_DATA = 4,
   UCODE_TLV_BOOT = 5,
   UCODE_TLV_PROBE_MAX_LEN = 6,
   UCODE_TLV_PAN = 7,
   UCODE_TLV_RUNT_EVTLOG_PTR = 8,
   UCODE_TLV_RUNT_EVTLOG_SIZE = 9,
   UCODE_TLV_RUNT_ERRLOG_PTR = 10,
   UCODE_TLV_INIT_EVTLOG_PTR = 11,
   UCODE_TLV_INIT_EVTLOG_SIZE = 12,
   UCODE_TLV_INIT_ERRLOG_PTR = 13,
   UCODE_TLV_ENHANCE_SENS_TBL = 14,
   UCODE_TLV_PHY_CALIBRATION_SIZE = 15,
   UCODE_TLV_WOWLAN_INST = 16,
   UCODE_TLV_WOWLAN_DATA = 17,
   UCODE_TLV_FLAGS = 18,
   UCODE_TLV_SEC_RT = 19,
   UCODE_TLV_SEC_INIT = 20,
   UCODE_TLV_SEC_WOWLAN = 21,
   UCODE_TLV_DEF_CALIB = 22,
   UCODE_TLV_PHY_SKU = 23,
   UCODE_TLV_SECURE_SEC_RT = 24,
   UCODE_TLV_SECURE_SEC_INIT = 25,
   UCODE_TLV_SECURE_SEC_WOWLAN = 26,
   UCODE_TLV_NUM_OF_CPU = 27,
   UCODE_TLV_CSCHEME = 28,
   UCODE_TLV_API_CHANGES_SET = 29,
   UCODE_TLV_ENABLED_CAPABILITIES = 30,
   UCODE_TLV_N_SCAN_CHANNELS = 31,
   UCODE_TLV_PAGING = 32,
   UCODE_TLV_SEC_RT_USNIFFER = 34,
   UCODE_TLV_SDIO_ADMA_ADDR = 35,
   UCODE_TLV_FW_VERSION = 36,
   UCODE_TLV_FW_DBG_DEST = 38,
   UCODE_TLV_FW_DBG_CONF = 39,
   UCODE_TLV_FW_DBG_TRIGGER = 40,
   UCODE_TLV_FW_GSCAN_CAPA = 50,
   UCODE_TLV_FW_MEM_SEG = 51,
}
