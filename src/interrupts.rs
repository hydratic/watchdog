extern crate watchdog_raw as raw;
extern crate spin;

use raw::pic;
use spin::*;

pub const TIMER_INTERRUPT_ID: u8 = PIC_1_OFFSET; // new

pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

pub static PICS: spin::Mutex<ChainedPics> =
    spin::Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        […]
        idt[usize::from(TIMER_INTERRUPT_ID)]
            .set_handler_fn(timer_interrupt_handler); // new

        idt
    };
}
