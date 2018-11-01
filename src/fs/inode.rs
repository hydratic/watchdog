#![no_std]

extern crate atomic_hashmap;
extern crate bytecount;
extern crate lz4;
extern crate ux;

extern crate watchdog_crypto;
extern crate watchdog_raw;

use watchdog_raw::cpu::interrupts;

mod ahci;
mod atapi;
mod bitmap;

pub struct Inode_S {
    name: &str,
    intype: i2, // TODO: Document
    permissions: i8,
    timestamp: &str,
    links: i32,
    pointers: Vec<u64>,
    indirect_blocks: Vec<u64>,
    double_indirect_blocks: Vec<u64>,
}

impl Inode_Utils for Inode_S {
    type Inode = Inode_S;

    pub fn make_inode_pointer(&self, inode: &Inode) {
        loop {
            let mut pos;
            let mut next_free_sec = get_next_sector!();
            let mut next_free_sec_size = get_next_sector_size!();
            let mut inode_contents = get_inode_contents!();
            let mut inode_warnings = get_inode_warnings!();
            let mut chars = get_chars!(inode_contents);
            
            for x in 0..inode_warnings {
                if inode_warnings[pos] == "multibyte" {
                    let mut size = chars * 4;
                }
                pos = pos + 1;
            }

            if size >= next_free_sec_size {
                break;
            } else { break; }
        }

        pointer_table!(9, 0, 0);
        pointer_table!(2, inode, pos);
    }

    pub fn get_inode_indirect_blocks(&self, inode: &Inode) -> &str {
        
    }

    pub fn get_inode_size(&self, inode: &Inode) -> i64 {
    
    }

    pub fn get_inode_contents(&self, inode: &Inode) -> Vec<&str> {
        
    }
}

#[macro_export]
macro_rules! inode_table {
    ($$itype:expr, $msg:expr) => {
        {
        if $$itype == 0 { 
            let inode_table = atomic_hashmap::new();
            const HASHMAP_CREATED: bool = true;
        } else if $$itype == 1 {
            // add a file to the inode table
            let y = HASHMAP_CREATED;
            if y == true {
                let inode_stats = get_inode();
                let inode_pointer_stats = get_inode_pointers();
                let inode_indirect_block_stats = get_inode_indirect_blocks();
                let inode_double_indirect_block_stats = get_inode_double_indirect_blocks();
                let inode = Inode {
                    name = lz4::compress_and_encrypt!(inode_stats[0], "serpent"),
                    intype = inode_stats[1],
                    permissions = inode_stats[2],
                    timestamp = lz4::compress_and_encrypt!(inode_stats[3], "serpent"),
                    links = inode_stats[4],
                    pointers = inode_pointer_stats;
                    indirect_blocks = inode_indirect_block_stats;
                    double_indirect_blocks = inode_double_indirect_block_stats;
                };

                // now we need to insert this into the hashmap
                inode_table.insert(pos, inode);
                pos = pos + 1;
            } else { println!("Error 701: HashMap not yet created!"); }
        } else if $itype == 2 {
            // remove a file from the inode table
            let mut cur_inode;
            let mut pos = 1;
            let mut nope: i2 = 0;

            for x in 0..INODE_COUNT {
                cur_inode = inode_table[pos];
                match cur_inode {
                    $msg => { break; }
                    _ => { nope = 1; }
                }
                if nope == 0 { break; }
            }

            inode_table.remove(pos, cur_inode);
        } else if $itype == 3 {
            // modify an inode in the inode table
            // first, we remove the inode from the table
            let mut cur_inode;
            let mut pos = 1;
            let mut nope: i2 = 0;

            for x in 0..INODE_COUNT {
                cur_inode = inode_table[pos];
                match cur_inode {
                    $msg => { break; }
                    _ => { nope = 1; }
                }
                if nope == 0 { break; }
            }
                
            inode.remove(pos, cur_inode);

            // now, we add it back with the different values
            let inode_stats = get_inode();
            let inode_pointer_stats = get_inode_pointers();
            let inode_indirect_block_stats = get_inode_indirect_blocks();
            let inode_double_indirect_block_stats = get_inode_double_indirect_blocks();
            let inode = Inode {
                name = lz4::compress_and_encrypt!(inode_stats[0], "serpent"),
                intype = inode_stats[1],
                permissions = inode_stats[2],
                timestamp = lz4::compress_and_encrypt!(inode_stats[3], "serpent"),
                links = inode_stats[4],
                pointers = inode_pointer_stats;
                indirect_blocks = inode_indirect_block_stats;
                double_indirect_blocks = inode_double_indirect_block_stats;
            };

            // now we need to insert this into the hashmap
            inode_table.insert(pos, inode);
            pos = pos + 1;
        }
    }};
}
