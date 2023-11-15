use std::process;

use better_kill::{
	app,
	signal::Signal,
	types::{Options, UidType, Users},
};
use clap::clap_app;
use pretty_env_logger;

fn main() {
	pretty_env_logger::init();

	let matches = clap_app!(("better-kill") =>
		(version: "0.1.0")
		(author: "Khushraj Rathod <me@khushrajrathod.com>")
		(about: "A better kill command")
		(@arg nofuzzy: -f --("no-fuzzy") "Disable fuzzy matching")
		(@arg single: -S --single "Disable killing all processes with the same name")
		(@arg uninteractive: -i --uninteractive "Disable interactive mode if args are not completely matched")
		(@arg force: -F --force "Kill a process forcefully. Equivalent of --signal kill")
		(@arg SIGNAL: -s --signal +takes_value default_value("SIGTERM") "Signal to send to process")
		(@arg USER: -u --user +takes_value default_value("current") ...
			"UID/name of user(s) to filter processes by. \
				Use 'all' to list processes owned by any user. \
				Use 'noroot' for non-root processes")
		(@arg process: "Process to kill. Omitting this will enable interactive mode")
	)
	.get_matches();

	app(Options {
		fuzzy:       !matches.is_present("nofuzzy"),
		all:         !matches.is_present("single"),
		interactive: !matches.is_present("uninteractive"),
		signal:      if matches.is_present("force") {
			Signal(libc::SIGKILL)
		} else {
			let signal = matches.value_of_lossy("SIGNAL").unwrap();
			match signal.parse::<i32>() {
				Ok(n) => Signal::from(n),
				Err(_) => Signal::from(signal.as_ref()),
			}
		},
		users:       {
			let users = matches.values_of_lossy("USER").unwrap();
			let mut uids: Users = Users::Some(vec![]);

			for user_case in users {
				let user = user_case.to_uppercase();

				if user == "ALL" {
					uids = Users::All;
					break;
				}

				if user == "NOROOT" {
					uids = Users::NoRoot;
					break;
				}

				if let Users::Some(u) = &mut uids {
					if user == "CURRENT" {
						u.push(UidType::Current);
					} else if user == "ROOT" {
						u.push(UidType::Root);
					} else {
						match user.parse() {
							Ok(uid) => u.push(UidType::Other(uid)),
							Err(_) => {
								match users::get_user_by_name(&user_case) {
									Some(user) => u.push(UidType::Other(user.uid())),
									None => {
										eprintln!("Couldn't find user: {}", user);
										process::exit(1)
									}
								}
							}
						}
					}
				}
			}

			uids
		},
		process:     matches.value_of("process"),
	});
}
