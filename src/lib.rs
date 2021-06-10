mod kill;
mod list;
mod types;

use sysinfo::{System, SystemExt};

use crate::{
	list::get_processes,
	types::{UidType, Users},
};

pub fn app() {
	let sys = System::new_all();
	let procs = get_processes(&sys, Users::Some(vec![UidType::Current]));
	println!("{:#?}", procs);
}
