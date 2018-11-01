// file.rs
//
// glue for standard libraries
// also provides a prettier interface to fs functions

#![no_std]

extern crate watchdog_ralloc;

mod bitmap;
mod inode;

pub fn new(name: &str, contents: Vec<&str>) {
    if filled == false {
        new_inode!(name);
    } else {
        new_inode!(name, contents);
    }
}
