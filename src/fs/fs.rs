#![no_std]

// ** IMPORTS ""
extern crate volatile;

use volatile::Volatile;
use volatile::ReadWrite;

mod crypto;
mod initrd;

// ** TYPES ** 
type read_type_t = u32;
type write_type_t = u32;
type open_type_t = fs_node;
type close_type_t = fs_node;
type readdir_type_t = fs_node;
type finddir_type_t = fs_node;


// ** CONSTS AND STATICS **
pub static fs_root: fs_node = 0;

// disk stats
pub const UNUSED_SPACE: i128 = TOTAL_SPACE - USED_SPACE; 
pub static CYLINDER: i64 = DISK_BYTES \ 255 * 63 * 512;
pub static LBA_COUNT: i64 = DISK_BYTES / BYTES_OVER_SECTOR;
pub const TRACKS: i64 = 0;
pub static TOTAL_SECTORS: i128 = SIZE * CYLINDER_PLUS_ONE * TRACKS_PLUS_ONE;

// required for fs operations
pub const FILE: Volatile<u32> = 0x01;
pub const DIRECTORY: Volatile<u32> = 0x02;
pub const CHARDEVICE: Volatile<u32> = 0x03;
pub const BLOCKDEVICE: Volatile<u32> = 0x04;
pub const PIPE: Volatile<u32> = 0x05;
pub const SYMLINK: Volatile<u32> = 0x06;
pub const MOUNTPOINT: Volatile<u32> = 0x08;

// ** STRUCTS **
pub struct fs_node {
    name: &str,
    mask: u32,
    uid: u32,
    gid: u32,
    flags: u32,
    inode: u32,
    length: u32,
    imp: u32,
    read: read_type_t,
    write: write_type_t,
    open: open_type_t,
    close: close_type_t,
    readdir: readdir_type_t,
    finddir: finddir_type_t,
    ptr: fs_node,
}

pub struct dirent {
    name: &str,
    ino: u32,
}

// ** FS OPERATIONS **
pub fn read_fs(node: fs_node, offset: u32, size: u32, buffer: u8) {
    if read != 0 {
        let ret = read(node, offset, size, buffer);
    } else { break; }
}
