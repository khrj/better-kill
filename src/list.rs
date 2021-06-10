use sysinfo::{ProcessExt, System, SystemExt};
use users::get_current_uid;

use crate::types::{Process, UidType, Users};

pub fn get_processes<'a>(sys: &'a System, users: Users) -> Vec<Process<'a>> {
	let current_uid = get_current_uid();

	let processes = filter_processes(
		sys.get_processes()
			.iter()
			.map(|(_pid, process)| {
				Process {
					name:  process.name(),
					count: 1,
					uid:   {
						if process.uid == 0 {
							UidType::Root
						} else if process.uid == current_uid {
							UidType::Current
						} else {
							UidType::Other(process.uid)
						}
					},
				}
			})
			.fold(Vec::new(), |mut acc, process| {
				increment_entry(&mut acc, process);
				acc
			}),
		users,
	);

	processes
}

fn increment_entry<'a>(processes: &mut Vec<Process<'a>>, current_process: Process<'a>) {
	for process in &mut *processes {
		if process.name == current_process.name {
			process.count += 1;
			return;
		}
	}

	processes.push(current_process);
}

fn filter_processes(processes: Vec<Process>, users: Users) -> Vec<Process> {
	match users {
		Users::All => processes,
		Users::Some(users) => {
			processes
				.into_iter()
				.filter(|process| {
					for user in &users {
						if process.uid == *user {
							return true;
						}
					}

					false
				})
				.collect()
		}
	}
}
