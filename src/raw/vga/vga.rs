/*
 * vga.rs
 *
 * The VGA implementation for the watchdog kernel.
 */

#![no_std]

extern crate bytecount;
extern crate ux;

// VESA
//
// VESA should be on by default
//
// however, it is not completely implemented yet so the alternative
// will work for now
pub const VESA: i2 = 0;
pub const PIXEL_OFFSET: u32 = Y * PITCH + (X * (BPP/8)) + FRAMEBUFFER;
pub const PITCH: u32 = 0;
pub const BPP: i32 = 0;
pub const FRAMEBUFFER: u32 = 0;

// position
pub const X: i8 = 1;
pub const Y: i8 = 1;

// structs
pub struct vbe_info_structure {
    signature: &str, // should equal "VESA"
    version: u16,
    oem: u32,
    capabilities: u32,
    video_modes: u32,
    video_memory: u16,
    software_rev: u16,
    vendor: u32,
    product_name: u32,
    product_rev: u32,
    reserved: &str,
    oem_data: &str,
}

pub struct vbe_mode_info_structure {
    attributes: u16,   // deprecated
    window_a: u8,      // deprecated
    window_b: u8,      // deprecated
    granularity: u16,  // deprecated
    window_size: u16,
    segment_a: u16,
    segment_b: u16,
    win_func_ptr: u32, // deprecated
    pitch: u16,
    width: u16,
    height: u16,
    w_char: u8,        // unused
    y_char: u8,        // unused
    planes: u8,
    bpp: u8,
    banks: u8,         // deprecated
    memory_model: u8,
    bank_size: u8,     // deprecated
    image_pages: u8,
    reserved0: u8,

    red_mask: u8,
    red_position: u8,
    green_mask: u8,
    green_position: u8,
    blue_mask: u8,
    blue_position: u8,
    reserved_mask: u8,
    reserved_position: u8,
    direct_color_attributes: u8,

    framebuffer: u32,
    off_screen_mem_off: u32,
    off_screen_mem_size: u32,
    reserved1[206]: u8,
}

pub struct vbe2_pmi_table {
    set_window: u16,
    set_display_start: u16,
    set_pallette: u16,
}

// standard print
extern {
    pub fn WriteCharacter(c: u8, forecolour: u8, backcolour: u8, x: i16, y: i16);
}

// VESA
extern {
    pub fn findMode(x: i16, y: i16, d: i16);
}

macro_rules! print {
    ($msg:expr) => {{
        if VESA == 0 {
            let size = bytecount::num_chars(s$msg.as_bytes());
            for byte in 0..size {
                unsafe {
                    if Y < MAX_Y {
                        if X < MAX_X {
                            WriteCharacter(character, FORECOLOR, BACKCOLOR, X, Y);
                            X += X + 1;
                        } else if X == MAX_X {
                            WriteCharacter("  ", 0, 0, 1, Y + 1)
                            Y = Y + 1;
                            WORD_WRAP.insert[Y, character];
                            WriteCharacter(character, FORECOLOR, BACKCOLOR, X, Y);
                            X = X + 1;
                        }
                    } else if Y == MAX_Y {
                        if X + 1 < MAX_X {
                            WriteCharacter(character, FORECOLOR, BACKCOLOR, X, Y);
                            X += X + 1;
                        } else if X == MAX_X {
                            scroll();
                        }
                    }
                }		
            }
        } else {
            break;
        }
    }};
}

macro_rules! println {
    ($msg:expr) => {{
        if VESA == 0 {
            let size = bytecount::num_chars(s$msg.as_bytes());
            for byte in 0..size {
                unsafe {
                    if Y < MAX_Y {
                        if X < MAX_X {
                            WriteCharacter(character, FORECOLOR, BACKCOLOR, X, Y);
                            X += X + 1;
                        } else if X == MAX_X {
                            WriteCharacter("  ", 0, 0, 1, Y + 1)
                            Y = Y + 1;
                            WORD_WRAP.insert[Y, character];
                            WriteCharacter(character, FORECOLOR, BACKCOLOR, X, Y);
                            X = X + 1;
                        }
                    } else if Y == MAX_Y {
                        if X + 1 < MAX_X {
                            WriteCharacter(character, FORECOLOR, BACKCOLOR, X, Y);
                            X += X + 1;
                        } else if X == MAX_X {
                            scroll();
                        }
                    }
                }		
            }
        } else {
            break;
        }
    }};
}
