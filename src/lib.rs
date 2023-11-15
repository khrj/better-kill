mod filter;
mod list;
mod print;
pub mod signal;
pub mod types;

use std::process;

use colored::Colorize;
use libc;
use log::debug;
use signal::Signal;
use sysinfo::{ProcessExt, RefreshKind, System, SystemExt};
use text_io::read;
use types::Options;

use crate::{
	filter::filter,
	list::get_processes,
	print::print_processes,
	types::{SearchMatch, UidType, Users},
};

pub fn app(options: Options) {
	debug!("{:#?}", options);

	let system = System::new_with_specifics(RefreshKind::new().with_processes());
	let system_processes =
		get_processes(system.get_processes(), Users::Some(vec![UidType::Current]));

	match options.process {
		Some(process_name) => {
			let matched_processes = filter(&system_processes, process_name);
			match matched_processes {
				SearchMatch::None => {
					eprintln!("No matching processes found");
					process::exit(1);
				}
				SearchMatch::Single(process) => {
					if process.name == process_name {
						println!("Exactly matched '{}', killing...", process.name.blue());
						kill_all(process.name, &options.signal);
					} else {
						println!("Partially matched '{}'. Kill [Y/n]?", process.name.blue());

						let choice: String = read!();

						if choice == "Y" || choice == "y" || choice == "" {
							kill_all(process.name, &options.signal);
						}
					}
				}
				SearchMatch::Multiple(processes) => {
					println!("Multiple matches");
					print_processes(processes);
					todo!()
				}
			}
		}
		None => todo!(),
	}
}

fn kill_all(name: &str, signal: &Signal) {
	let mut system = System::new_with_specifics(RefreshKind::new().with_processes());
	let mut all_killed = false;

	while !all_killed {
		system.refresh_processes();
		let system_processes =
			get_processes(system.get_processes(), Users::Some(vec![UidType::Current]));

		let matched_processes = filter(&system_processes, name);
		match matched_processes {
			SearchMatch::None => all_killed = true,
			SearchMatch::Multiple(_) => todo!(),
			SearchMatch::Single(process) => {
				c_kill(process.handle.pid(), signal);
				println!("Killing pid: {} with signal {}", process.handle.pid(), signal.0);
			}
		}
	}
}

fn c_kill(pid: i32, signal: &Signal) {
	unsafe {
		libc::kill(pid, signal.0);
	}
}
