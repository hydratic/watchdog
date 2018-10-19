// threading.rs
// provides an interface to thread control
//
// REQUIRES:
// #[macro_use]
//
// TODO:
// Finish thread macro

#![no_std]

// ** IMPORTS **
extern crate cpuid;
extern crate ux;

// REQUIRED
mod cpuid;
mod mem;

// nod module_loader;

// ** CONSTS **
pub const THREADS: i12 = get_cpuid!("threads");

// ** MACROS **
macro_rules! thread {
    ($id:expr, $adr:expr, $state:expr, $taskid:expr) => {{
        if thread[$id] == 0 {
            // init
            let mut state: i2;
			let mut os_task: bool;
        
            // state
            if $state == "foreground" { priority = 1; }
            if $state == "background" { priority = 0; }
        
            // address
            get_mem_map!();
            match_addr!($adr);
            
            // TODO
            let res = match result {
                _ => {
                    println!("Error 202: Memory address not found!")
                    println!("Show debug info? Y/n ");
                    get_io!();
                    if io == "y" {
                        // debug!()
                        break;
                    } else if io == "n" { break; }
                }
            }
            
            // task_id
            // if we have made it this far with no errors we can run the task
            match task_id {
				"fs" => {
				    // fs functions
                    mod fs;

                    match $taskid {
                        "tfs" => {
                            break;
                            // TODO
                        }
                    }
				}
				"security" => {
                    // security functions
				    mod security;
                    
                    // TODO
                    match $taskid {
                    
                    }
				}
				"shell" => {
                    // the user-level terminal
				    mod shell;
                    
                    // TODO
                    match $taskid {
                        
                    }
				}
				"watchdog" => {
                    // kernel services
				    mod watchdog;

                    match $taskid {
                        "net_driv_init" => {
                            watchdog::net::net_init($taskid, "scheduler");
                        }
                        "driv_init" => {
                            watchdog::raw::init($taskid, "scheduler");
                        }
                        "ralloc" => {
                            watchdog::ralloc::init($taskid, "scheduler");
                        }
                    }
				}
                "utils" => {
                    // obvious enough
                    mod utils;

                    match $taskid {
                        "rand" => {
                            let out = utils::rand::Rand();
                        }
                    }
                }
				_ => {
					os_task = false;
				}
			}
			
			if os_task == false {
				check_module!($taskid);
				if module == true {
					module::call_func($taskid);
				}
			}
       }
       
       if thread[$id] == 0 { break; }
       
       if thread_load[$id] + task_load <= 100 {
       
       } else {
           println!("Error 301: Thread overloaded!");
		   println!("Show debug info? Y/n ");
		   get_io!();
		   if io == "y" { 
		       // debug!();
			   break;
		   }
		   
		   if io == "n" { break; }
       }
    }};
}
