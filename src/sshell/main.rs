#![no_std]
 
extern crate spin;
extern crate watchdog_raw;
 
static PICS: Mutex<ChainedPics> =
    Mutex::new(unsafe { ChainedPics::new(0x20, 0x28) });

static KEYBOARD: Mutex<Port<u8>> = Mutex::new(unsafe {
    Port::new(0x60)
});
 
pub fn sshell() {
	PICS.lock().initialize()

  	let mut space: i2;
	let mut input: String = "";
	space = 0;
	let scancode = KEYBOARD.lock().read();
	let letter = io_driv::ps2_keyboard::match_code(scancode);
	if letter == "<ENTER>" {
		match cmd {
			"cd" => {
			
			}
			"mkdir" => {

			}
			"rmdir" => {
			
			}
			_ => {
				match cmd {
					"sudo" => {

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
		
	PICS.lock().notify_end_of_interrupt(interrupt_id);
}
