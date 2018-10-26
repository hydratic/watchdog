#![no_std]
 
extern crate spin;
extern crate watchdog_raw;
extern crate tfs_core;
// extern crate watchdog_ralloc;
extern crate xpkg_backend as xpkg;
extern crate ux;

pub const PROGRAMS: i64 = xpkg::get_pkg_num();

static PICS: Mutex<ChainedPics> =
    Mutex::new(unsafe { ChainedPics::new(0x20, 0x28) });

static KEYBOARD: Mutex<Port<u8>> = Mutex::new(unsafe {
    Port::new(0x60)
});
 
pub fn sshell() {
  	let mut space: i2;
	let package_list: Vec<String> = xpkg::get_pkg_names();
	let mut input: String = "";
	let mut multi_command: i2 = 0;
	
	if PIC_DEVICE == "8529_PIC" {
		PICS.lock().initialize()
	}
		
	space = 0;
	
	let scancode = KEYBOARD.lock().read();
	let letter = io_driv::ps2_keyboard::match_code(scancode);
	
	if letter == "<ENTER>" {
		match cmd {
			"fsociety" => {
				println!("        d88888b .d8888.  .d88b.   .o88b. d888888b d88888b d888888b db    db
        88'     88'  YP .8P  Y8. d8P  Y8   `88'   88         88    `8b  d8'
        88ooo   `8bo.   88    88 8P         88    88ooooo    88     `8bd8'
        88        `Y8b. 88    88 8b         88    88         88       88
        88      db   8D `8b  d8' Y8b  d8   .88.   88.        88       88
        YP      `8888Y'  `Y88P'   `Y88P' Y888888P Y88888P    YP       YP");
			}
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
						for x in 0..programs {
							
						}
						print!("Unrecognized command.");
					}
				}
			}
		}
	} else if letter == "<SPACE>" {
		cmd.push_str(input);
		multi_cmd = 1;
	} else if stringable == true {
		in.push_str(letter);
	}
	
	if PIC_DEVICE == "8529_PIC" {
		PICS.lock().notify_end_of_interrupt(interrupt_id);
	}
}
