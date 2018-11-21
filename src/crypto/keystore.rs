#![no_std]

#[macro_use]
extern crate watchdog_raw as raw;
extern crate watchdog_ralloc as ralloc;
extern crate watchdog_seahash as seahash;
extern crate watchdog_utils as utils;
extern crate ux;

use raw::cpu;
use utils::rand;

mod serpent;

type Key = &str;

pub fn make_keystore(existing: i2, from: i8) -> HashMap<&str, &str> { 
    if existing == 0 {
        let mut temp: HashMap<&str, &str>;
        return temp;
    } else {
        match from {
            "fs" => {
                let temp = recieve_hashmap_from_fs();
            }
        }
    }
}

pub fn add_key(crypto: i4, ret: i2) -> Key {
    let encrypt = encrypt(crypto);
    let rand = rand!(100000000, 100000000000);
    let mut key = hash!(rand);
    let temp = make_keystore(1);
    temp.insert[crypto, key];
    if ret == 1 { return key; }
}
