#![no_std]
 
extern crate spin;
extern crate watchdog_raw;
extern crate tfs_core;
// extern crate watchdog_ralloc; 

static PICS: Mutex<ChainedPics> =
    Mutex::new(unsafe { ChainedPics::new(0x20, 0x28) });

static KEYBOARD: Mutex<Port<u8>> = Mutex::new(unsafe {
    Port::new(0x60)
});
 
pub fn sshell() {
	if PIC_DEVICE == "8529_PIC" {
		PICS.lock().initialize()
	}
		
  	let mut space: i2;
	let mut input: String = "";
	space = 0;
	let scancode = KEYBOARD.lock().read();
	let letter = io_driv::ps2_keyboard::match_code(scancode);
	if letter == "<ENTER>" {
		match cmd {
			_ => {
				match cmd {
					// ** FS FUNCTIONS **
					"cd" => {
						if multi_cmd == 0 { print!("CD is not a standalone command.");
						if multi_cmd == 0 { break; }
					}
					"mkdir" => {
						// call TFS to make a directory
					}
					"rmdir" => {
						// call TFS to remove a directory
					}
					
					"sudo" => {
						if SUDO == 1 {
							if multi_cmd == 0 { print!("SUDO is not a standalone command.");
							if multi_cmd == 0 { break; }
						} else {
							print!("Superuser access not available!"); 
						}
					}
					_ => {
						print!("Unrecognized command.");
					}
				}
			}
		}
	} else if letter == "<SPACE>" {
		cmd.push_str(input);
	} else if stringable == true {
		in.push_str(letter);
	}
	
	if PIC_DEVICE == "8529_PIC" {
		PICS.lock().notify_end_of_interrupt(interrupt_id);
	}
}
