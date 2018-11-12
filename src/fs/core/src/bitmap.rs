// bitmap.rs
// provides an interface to kernel level block allocation, used by drive.rs
// to find available sectors for storing data.
//
// TODO: Finish
//       Encrypt and compress block locations
//       Implement checks for permission

#![no_std]

extern crate atomic_hashmap;
extern crate lz4;
extern crate ux;

extern crate watchdog_raw;
extern crate watchdog_crypto;

use watchdog_crypto::serpent;
use watchdog_crypto::keystore;
use watchdog_raw::query;

mod ahci;
mod atapi;
mod inode;
mod disk;

pub struct Block_ {
    size: i32,
    used: i32,
    left: i32,
    location: Vec<u64>,
}

impl Block__ for Block_ {
    type Block = Block_;
    
    pub fn new_block(size: i8, location: Vec<u64>) { bitmap_table!(4, size, location); }
    pub fn remove_block(&self, block: &Block) { bitmap_table!(5, block); }
}

#[macro_export]
macro_rules! bitmap_table {
    ($btype:expr, $inode:expr, $pos:expr) => {{
        if $btype == 1 { // initialize hashmap
            let mut bitmap_table: HashMap<Inode, i64>;
            let blank = Inode {
                name = compress_and_encrypt!("blank");
                // TODO: Intype
                permissions: 0,
                timestamp: compress_and_encrypt!("boot"),
                links: 0,
                pointers: vec!(0, 0), // TODO: Find a way to not waste memory
                indirect_blocks: vec!(0, 0),
                double_indirect_blocks: vec!(0, 0),
            };

            bitmap_table.insert(blank, 0);
            const HASHMAP_CREATED: bool = true;
        } else if $bytpe == 2 { // add an inode to the bitmap
            // TODO: Document this loop
            loop {
                bitmap_table.insert($inode, $pos);
                let inserted = true;
                break;
            }
            
            if inserted == true {
                let res = write_sec($pos, $inode);
            }
        } else if $btype == 3 { // remove an inode from the bitmap
            let x = bitmap_table[0];
            let y = bitmap_table[0];
            try!(let x = bitmap_table[$pos]);
            if x == y { println!("Error!"); }
            
        } else if $btype == 4 { // add a block to the bitmap
            if $inode == "DEF" {
                if $pos == "NOP" {
                    let new_block = Block {
                        size = 512,
                        used = 0;
                        left = 512,
                        location = get_next_unused_spot();
                    };
                } else {
                    let new_block = Block {
                        size = 512,
                        used = 0,
                        left = 512,
                        location = $pos,
                    };
                }
            } else {
                let new_block = Block {
                    size = $inode,
                    used = 0,
                    left = $inode,
                    location = get_next_unused_spot();
                };
            }
        } else if $btype == 5 {

        } else if $btype == 9 { // create local hashmap
            if HASHMAP_CREATED = true { let mut bitmap_table_2: HashMap<Inode, i64>
        }    
    }};
}
