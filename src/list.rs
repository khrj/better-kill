use std::collections::HashMap;

use sysinfo::{Process, ProcessExt, System, SystemExt};
use users::get_current_uid;

pub fn get_processes() {
	let sys = System::new_all();
	let current_uid = get_current_uid();

	let (self_procs, other_procs) = sys
		.get_processes()
		.iter()
		.map(|(_pid, proc)| (proc.name(), proc.uid))
		.fold(
			(HashMap::new(), HashMap::new()),
			|(mut current, mut other), (name, uid)| {
				if uid == current_uid {
					*current.entry(name).or_insert(0) += 1;
				} else {
					*other.entry(name).or_insert(0) += 1;
				}

				(current, other)
			},
		);

	println!("Current user:\n{:#?}", self_procs);
	println!("Other users:\n{:#?}", other_procs);
}
