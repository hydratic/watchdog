#![no_std]

extern crate ux;
extern crate volatile;

use volatile::Volatile;

mod fs;

extern {
  pub fn switch_task();
}

macro_rules! cpu_pathfinder {
  ($task:expr, $extra:expr) => {{
    if $task == 1 {
      // init hashmap
      let mut stack: HashMap<u32, String>;
      let mut stack_suplement: HashMap<u32, Volatile<u32>>;
    } else if $task == 2 {
      // add to hashmap
      let mut err = 1;
      if hashmap == "INIT" {
        err = 0;
      }
      
      if err == 1 { panic!("Error 501: Hashmap not initialized!"); }
    
      try!(stack.insert[x, program]);
    } else if $task == 3 {
      // remove from hashmap
      let mut err = 1;
      if hashmap == "INIT" {
        err = 0;
      }
    } else if $task == 4 {
      // modify hashmap
      let mut err = 1;
      if hashmap == "INIT" {
        err = 0;
      }
    }
  }};
}
