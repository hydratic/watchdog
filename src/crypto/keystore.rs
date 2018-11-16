#![no_std]

#[macro_use]
extern crate watchdog_raw as raw;
extern crate watchdog_ralloc as ralloc;
extern crate watchdog_seahash as seahash;
extern crate ux;

use raw::cpu;

mod serpent;

pub fn keystore(path: &str, out?: i2) -> HashMap<&str, &str> { 
    let mut temp: HashMap<&str, &str>;
    return temp;
}
