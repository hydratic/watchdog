#![no_std]

extern crate core as std;

extern crate memadvise;
extern crate ux;

pub mod ahci;
pub mod err;
pub mod raw;
pub mod stats;

pub const NOT: i2 = 0;
pub const SIZE: i64 = get_size!() 
pub const SECTORS: i32 = SIZE * 1073741824 / 512;

pub fn probe_sector(sector: i64) {

}

// TODO:
// assign to thread
macro_rules! contents {
    () => {{
        let mut m = HashMap::New();

        loop {
            let change = update!();
        }
    }};
}

macro_rules! update {
    () => {{
        let mut err;
        for x in 0..SECTORS {
            if NOT == 1 {
                probe_sector(x);

                x = x + 1;
                
                if x == SECTORS {
                
                } else x > SECTORS {
                    let mut code = err::match_err!() 
                    println!("Error code {}, error reading sector.", code);
                }
            } else { )
                raw::cpu::sleep(30000); 
            }
        }
    }};
}
