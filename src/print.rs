use colored::*;

use crate::types::Process;

pub fn print_processes(processes: Vec<&Process>) {
	processes.into_iter().for_each(|process| {
		println!(
			"{}({}): {}",
			process.name.green().bold(),
			process.count.to_string().yellow(),
			process.uid
		);
	})
}
