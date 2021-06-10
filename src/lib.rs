mod kill;
mod list;

use crate::list::get_processes;

pub fn app() { get_processes(); }
