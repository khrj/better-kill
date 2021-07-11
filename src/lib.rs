mod filter;
mod kill;
mod list;
mod print;
pub mod types;

use std::process;

use sysinfo::{System, SystemExt};
use types::Options;

use crate::{
	filter::filter,
	list::get_processes,
	print::print_processes,
	types::{SearchMatch, UidType, Users},
};

pub fn app(options: Options) {
	let sys = System::new_all();
	let procs = get_processes(&sys, Users::Some(vec![UidType::Current]));

	match options.process {
		Some(process) => {
			let matched_procs = filter(&procs, process);
			match matched_procs {
				SearchMatch::None => {
					eprintln!("No matching processes found");
					process::exit(1);
				}
				SearchMatch::Single(proc) => println!("Matched: {}", proc.name),
				SearchMatch::Multiple(procs) => {
					println!("Multiple matches");
					print_processes(procs);
				}
			}
		}
		None => todo!(),
	}
}
