// add_to_binary.rs
// WHEN WRITING BINARIES FOR WATCHDOG, YOU MUST ADD THIS
//
// if no_std:
// #![no_std]
// else

extern crate watchdog_libc;
extern crate watchdog_binary_support;

// if std:
use watchdog_libc::*;

use watchdog_binary_support::*;
