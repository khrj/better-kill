mod kill;
mod list;
mod print;
mod types;

use sysinfo::{System, SystemExt};

use crate::{
	list::get_processes,
	print::print_processes,
	types::{UidType, Users},
};

pub fn app() {
	let sys = System::new_all();
	let procs = get_processes(&sys, Users::Some(vec![UidType::Current]));
	print_processes(&procs);
}
