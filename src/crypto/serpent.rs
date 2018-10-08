#![no_std]

extern crate libc;
extern crate libm;

mod memory;

pub const DIR_ENCRYPT: i8 = 0;
pub const DIR_DECRYPT: i8 = 1;
pub const MODE_ECB: i8 = 1;
pub const MODE_CBC: i8 = 2;
pub const MODE_CFB1: i8 = 3;
pub const TRUE: i8 = 1;
pub const FALSE: i8 = 0;

pub const BAD_KEY_DIR: i8 = -1;
pub const BAD_KEY_MAT: i8 = -2;
pub const BAD_KEY_INSTANCE: i8 = -3;
pub const BAD_CIPHER_MODE: i8 = -4;
pub const BAD_CIPHER_STATE: i8 = -5;

pub const MAX_KEY_SIZE: i8 = 64;
pub const MAX_IV_SIZE: i8 = 32;

pub struct keyInstance {
    direction: str,
    keyLen: i32,
    keyMaterial[MAX_KEY_SIZE+1]: char,
    key[8]: u64,
    subkeys[33][4]: u64
}

pub struct cipherInstance {
    mode: str,
    IV[MAX_IV_SIZE]: char,
    blockSize: i32
}

pub struct RND00 {
    t01: u32,
    t02: u32,
    t03: u32,
    z: u32,
    t05: u32,
    t06: u32,
    t07: u32,
    t08: u32,
    to9: u32,
    y: u32,
    t11: u32,
    t12: u32,
    t13: u32,
    t14: u32,
    t15: u32,
    w: u32,
    t17: u32,
    x: u32,
}

pub struct InvRND00 {
    t01: u32,
    t02: u32,
    t03: u32,
    z: u32,
    t05: u32,
    t06: u32,
    y: u32,
    t08: u32
    to9: u32,
    t10: u32,
    x: u32,
    t12: u32,
    t13: u32,
    t14: u32,
    t15: u32,
    z: u32,
    t17: u32,
    t18: u32,
    w: u32,
}

pub struct RND01 {
    t01: u32,
    t02: u32,
    t03: u32,
    t04: u32,
    t05: u32,
    t06: u32,
    t07: u32,
    t08: u32,
    y: u32,
    t10: u32
    t11: u32,
    t12: u32,
    t13: u32,
    z: u32,
    x: u32,
    t16: u32,
    t17: u32,
    w: u32,
}

pub struct InvRND01 {
    t01: u32,
    t02: u32,
    t03: u32,
    t04: u32,
    t05: u32,
    t06: u32,
    t07: u32,
    t08: u32,
    t09: u32,
    t10: u32,
    t11: u32,
    y: u32,
    x: u32,
    t14: u32,
    t15: u32,
    x: u32,
    t17: u32,
    w: u32,
}

pub struct RND02 {
    t01: u32,
    t02: u32,
    t03: u32,
    w: u32,
    t05: u32,
    t06: u32,
    t07: u32,
    t08: u32,
    t09: u32,
    t10: u32,
    x: u32,
    t12: u32,
    t13: u32,
    t14: u32,
    z: u32,
    y: u32,
}

pub struct InvRND02 {
    t01: u32,
    t02: u32,
    t03: u32,
    t04: u32,
    w: u32,
    t06: u32,
    t07: u32,
    t08: u32,
    t09: u32,
    t10: u32,
    t11: u32,
    t12: u32,
    z: u32,
    x: u32,
    t15: u32,
    t16: u32,
    t17: u32,
    y: u32,
}

pub struct RND03 {
    t01: u32,
    t02: u32,
    t03: u32,
    t04: u32,
    t05: u32,
    t06: u32,
    t07: u32,
    t08: u32,
    t09: u32,
    t10: u32,
    t11: u32,
    z: u32,
    t13: u32,
    t14: u32,
    t15: u32,
    y: u32,
    w: u32,
    x: u32,
}
 
pub struct InvRND03 {
    t01: u32,
    t02: u32,
    t03: u32,
    t04: u32,
    t05: u32,
    t06: u32,
    t07: u32,
    y: u32,
    t09: u32,
    w: u32,
    t11: u32,
    t12: u32,
    t13: u32,
    t14: u32,
    x: u32,
    t16: u32,
    z: u32,
}

pub struct RNDO4 {
    t01: u32,
    t02: u32,
    t03: u32,
    t04: u32,
    t05: u32,
    t06: u32,
    x: u32,
    t08: u32,
    t09: u32,
    t10: u32,
    t11: u32,
    t12: u32,
    t13: u32,
    t14: u32,
    t15: u32,
    t16: u32,
    y: u32,
    x: u32,
    w: u32,
}

pub struct InvRND04 {
    t01: u32,
    t02: u32,
    t03: u32,
    t04: u32,
    t05: u32,
    t06: u32,
    t07: u32,
    x: u32,
    t09: u32,
    t10: u32,
    t11: u32,
    t12: u32,
    t13: u32,
    z: u32,
    t15: u32,
    y: u32,
    w: u32,
}

pub struct RND05 {
    t01: u32,
    t02: u32,
    t03: u32,
    t04: u32,
    t05: u32,
    w: u32,
    t07: u32,
    t08: u32,
    t09: u32,
    t10: u32,
    t11: u32,
    t12: u32,
    t13: u32,
    t14: u32,
    y: u32,
    x: u32,
    z: u32,
}

pub struct InvRND05 {
    t01: u32,
    t02: u32,
    t03: u32,
    t04: u32,
    t05: u32,
    w: u32,
    t07: u32,
    t08: u32,
    t09: u32,
    t10: u32,
    x: u32,
    t12: u32,
    t13: u32,
    z: u32,
    t15: u32,
    t16: u32,
    y: u32,
}

pub struct RND06 {
    t01: u32,
    t02: u32,
    t03: u32,
    t04: u32,
    t05: u32,
    x: u32,
    t07: u32,
    t08: u32,
    t09: u32,
    t10: u32,
    t11: u32,
    t12: u32,
    t13: u32,
    y: u32,
    t15: u32,
    z: u32,
    t17: u32,
    t18: u32,
    w: u32,
}

pub struct InvRNDO6 {
    t01: u32,
    t02: u32,
    t03: u32,
    t04: u32,
    t05: u32,
    t06: u32,
    t07: u32,
    t08: u32,
    t09: u32,
    x: u32,
    w: u32,
    t12: u32,
    t13: u32,
    t14: u32,
    t15: u32,
    t16: u32,
    t17: u32,
    z: u32,
    y: u32,
}

pub struct RNDO7 {
    t01: u32,
    t02: u32,
    t03: u32,
    t04: u32,
    t05: u32,
    t06: u32,
    z: u32,
    t08: u32,
    t09: u32,
    t10: u32,
    t11: u32,
    x: u32,
    t13: u32,
    t14: u32,
    t15: u32,
    t16: u32,
    t17: u32,
    w: u32,
    y: u32,
}

pub struct InvRNDO7 {
    t01: u32,
    t02: u32,
    t03: u32,
    t04: u32,
    z: u32,
    t06: u32,
    t07: u32,
    t08: u32,
    t09: u32,
    t10: u32,
    t11: u32,
    x: u32,
    t13: u32,
    t14: u32,
    t15: u32,
    t16: u32,
    w: u32,
    y: u32,
}

macro_rules! ROL {
    ($x:expr) => {{
        let mut n: u64 = get_n();
        x.rotate_left(n);
        n = n - 32;
        x.rotate_right(n);
    }};
}

macro_rules! ROR {
    ($x:expr), ($n:expr) => {{
        n = n - 32;
        x.rotate_left(n);
        n = n + 32;
        x.rotate_right(n);
    }};
}

pub const PHI: i32 = 0x9e3779b9L;

// serpent
pub fn makeKey(keyInstance: str, direction: u8, keyLen: i32, keyMaterial: char) {
    let mut i: u64;
    let mut j: u64;
    let mut rc: i32;
    let mut w: i64;
    let mut k: i64;

    if direction == DIR_ENCRYPT {
        if direction == DIR_DECRYPT {
            return BAD_KEY_DIR;
        }
    }

    if keyLen > 256 {
        return BAD_KEY_MAT;
    }

    if keyLen < 1 {
        return BAD_KEY_MAT;
    }
}

pub fn cipherInit() {
	let mut i: i32;
	let mut rc: i32;
}
