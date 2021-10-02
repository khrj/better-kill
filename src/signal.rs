use std::process;

use libc;

#[derive(Debug)]
pub struct Signal(pub i32);

impl From<i32> for Signal {
	fn from(code: i32) -> Self { Signal(code) }
}

impl From<&str> for Signal {
	#[rustfmt::skip]
	fn from(signal: &str) -> Self {
		Signal(match signal.to_uppercase().as_str() {
			"HUP"     | "SIGHUP"   | "HANGUP"    => libc::SIGHUP,
			"INT"     | "SIGINT"   | "INTERRUPT" => libc::SIGINT,
			"QUIT"    | "SIGQUIT"                => libc::SIGQUIT,
			"ILL"     | "SIGILL"   | "ILLEGAL"   => libc::SIGILL,
			"TRAP"    | "SIGTRAP"                => libc::SIGTRAP,
			"ABRT"    | "SIGABRT"  | "ABORT"     => libc::SIGABRT,
			"IOT"     | "SIGIOT"                 => libc::SIGIOT,
			"BUS"     | "SIGBUS"                 => libc::SIGBUS,
			"FPE"     | "SIGFPE"                 => libc::SIGFPE,
			"KILL"    | "SIGKILL"                => libc::SIGKILL,
			"USR1"    | "SIGUSR1"  | "USER1"     => libc::SIGUSR1,
			"SEGV"    | "SIGSGEV"                => libc::SIGSEGV,
			"USR2"    | "SIGUSR2"  | "USER2"     => libc::SIGUSR2,
			"PIPE"    | "SIGPIPE"                => libc::SIGPIPE, 
			"ALRM"    | "SIGALRM"  | "ALARM"     => libc::SIGALRM,
			"TERM"    | "SIGTERM"  | "TERMINATE" => libc::SIGTERM,
			"CHLD"    | "SIGCHILD" | "CHILD"     => libc::SIGCHLD,
			"CONT"    | "SIGCONT"  | "CONTINUE"  => libc::SIGCONT,
			"STOP"    | "SIGSTOP"                => libc::SIGSTOP,
			"TSTP"    | "SIGTSTP"                => libc::SIGTSTP,
			"TTIN"    | "SIGTTIN"                => libc::SIGTTIN,
			"TTOU"    | "SIGTTOU"                => libc::SIGTTOU,
			"URG"     | "SIGURG"   | "URGENT"    => libc::SIGURG,
			"XCPU"    | "SIGXCPU"                => libc::SIGXCPU,
			"XFSZ"    | "SIGXFSZ"                => libc::SIGXFSZ,
			"VTALRM"  | "SIGVTALRM"              => libc::SIGVTALRM,
			"PROF"    | "SIGPROF"  | "PROFILING" => libc::SIGPROF,
			"WINCH"   | "SIGWINCH"               => libc::SIGWINCH,
			"IO"      | "SIGIO"                  => libc::SIGIO,
			"POLL"    | "SIGPOLL"                => libc::SIGPOLL,
			"PWR"     | "SIGPWR"   | "POWER"     => libc::SIGPWR,
			"SYS"     | "SIGSYS"                 => libc::SIGSYS,
			other => {
				eprintln!("Unknown signal '{}'. Try bkill --help", other);
				process::exit(1);
			}
		})
	}
}
