#![no_std]

extern crate watchdog_fs as fs;
extern crate watchdog_utils as utils;
extern crate watchdog_mrustc as mrustc;
extern crate watchdog_ralloc as ralloc;
// TODO: Does this need to be imported?
extern crate watchdog_telephone as telephone;
extern crate ux;

pub fn execute(path: &str, compiled: i2) { 
    if compiled == 1 {
      let test = fs::test_path(path);
      if test == 1 {
        utils::elf::exec(path}
      } else {
        println!("Executable not found!"};
      }
    }
}
