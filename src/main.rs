use better_kill::app;
use clap::{App, Arg};

fn main() {
	let matches = App::new("better-kill")
		.version("0.1.0")
		.author("Khushraj Rathod <me@khushrajrathod.com>")
		.about("A better kill command")
		.arg(
			Arg::with_name("signal")
				.short("s")
				.long("signal")
				.takes_value(true)
				.value_name("SIGNAL")
				.help("Signal to send to process"),
		)
		.arg(
			Arg::with_name("fuzzy")
				.short("f")
				.long("fuzzy")
				.takes_value(false)
				.help("Use fuzzy matching"),
		)
		.arg(
			Arg::with_name("all")
				.short("a")
				.long("all")
				.help("Kill all processes with the same name"),
		)
		.arg(
			Arg::with_name("interactive")
				.short("i")
				.long("interactive")
				.takes_value(false)
				.help(
					"Enable interactive mode if args are not completely matched. Defaults to true",
				),
		)
		.arg(
			Arg::with_name("force")
				.short("F")
				.long("force")
				.takes_value(false)
				.help("Kill a process forcefully. Equivalent of --signal kill"),
		)
		.arg(
			Arg::with_name("user")
				.short("u")
				.long("user")
				.takes_value(true)
				.help(
					"User to list processes for. Defaults to current user. Use 'all' to list \
					 processes owned by any user",
				),
		)
		.get_matches();

	app();
}
