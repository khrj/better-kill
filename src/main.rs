use std::process;

use better_kill::{app, types::Options};
use clap::clap_app;

fn main() {
	let matches = clap_app!(("better-kill") =>
		(version: "0.1.0")
		(author: "Khushraj Rathod <me@khushrajrathod.com>")
		(about: "A better kill command")
		(@arg nofuzzy: -f --("no-fuzzy") "Disable fuzzy matching")
		(@arg single: -S --single "Disable killing all processes with the same name")
		(@arg uninteractive: -i --uninteractive "Disable interactive mode if args are not completely matched")
		(@arg force: -F --force "Kill a process forcefully. Equivalent of --signal kill")
		(@arg SIGNAL: -s --signal +takes_value default_value("15") "Signal to send to process. Defaults to SIGTERM")
		(@arg USER: -u --user +takes_value ...
			"UID/name of user(s) to filter processes by. Defaults to current user. \
				Use 'all' to list processes owned by any user")
		(@arg process: "Process to kill. Omitting this will enable interactive mode")
	)
	.get_matches();

	if matches.is_present("USER") {
		todo!()
	}

	app(Options {
		fuzzy:       !matches.is_present("nofuzzy"),
		all:         !matches.is_present("single"),
		interactive: !matches.is_present("uninteractive"),
		signal:      if matches.is_present("force") {
			9
		} else {
			let signal = matches.value_of("SIGNAL").unwrap();
			match signal.parse() {
				Ok(n) => n,
				Err(_) => {
					match signal.to_uppercase().as_str() {
						"HUP" | "SIGHUP" | "HANGUP" => 1,
						"INT" | "SIGINT" | "INTERRUPT" => 2,
						"QUIT" | "SIGQUIT" => 3,
						"ABRT" | "SIGABRT" | "ABORT" => 6,
						"KILL" | "SIGKILL" => 9,
						"ALRM" | "SIGALRM" | "ALARM" => 14,
						"TERM" | "SIGTERM" | "TERMINATE" => 15,
						_ => {
							eprintln!("Unknown signal. Try bkill --help");
							process::exit(1);
						}
					}
				}
			}
		},
		user:        &vec![],
		process:     matches.value_of("process"),
	});
}
