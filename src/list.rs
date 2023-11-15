use std::collections::HashMap;

use sysinfo::{Process, ProcessExt};
use users::get_current_uid;

use crate::types::{Process as BkillProcess, UidType, Users};

pub fn get_processes<'a>(
	system_processes: &'a HashMap<i32, Process>,
	users: Users,
) -> Vec<BkillProcess<'a>> {
	let current_uid = get_current_uid();

	let processes = filter_processes(
		system_processes
			.iter()
			.map(|(_pid, process)| {
				BkillProcess {
					name:   process.name(),
					count:  1,
					uid:    {
						if process.uid == 0 {
							UidType::Root
						} else if process.uid == current_uid {
							UidType::Current
						} else {
							UidType::Other(process.uid)
						}
					},
					handle: process,
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

fn increment_entry<'a>(processes: &mut Vec<BkillProcess<'a>>, current_process: BkillProcess<'a>) {
	for process in &mut *processes {
		if process.name == current_process.name {
			process.count += 1;
			return;
		}
	}

	processes.push(current_process);
}

fn filter_processes(processes: Vec<BkillProcess>, users: Users) -> Vec<BkillProcess> {
	match users {
		Users::All => processes,
		Users::NoRoot => {
			processes
				.into_iter()
				.filter(|process| {
					if let UidType::Root = &process.uid {
						false
					} else {
						true
					}
				})
				.collect()
		}
		Users::Some(users) => {
			processes
				.into_iter()
				.filter(|process| users.contains(&process.uid))
				.collect()
		}
	}
}
