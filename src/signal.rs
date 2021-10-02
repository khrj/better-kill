use std::process;

use sysinfo::Signal::{self, *};

#[derive(Debug)]
pub struct ProcessSignal(pub Signal);

impl From<u8> for ProcessSignal {
	fn from(code: u8) -> Self {
		ProcessSignal(match code {
			1 => Hangup,
			2 => Interrupt,
			3 => Quit,
			4 => Illegal,
			5 => Trap,
			6 => Abort,
			8 => FloatingPointException,
			9 => Kill,
			11 => Segv,
			13 => Pipe,
			14 => Alarm,
			15 => Term,
			n => {
				eprintln!("Unknown signal '{}'. Try bkill --help", n);
				process::exit(1);
			}
		})
	}
}

impl From<&str> for ProcessSignal {
	#[rustfmt::skip]
	fn from(signal: &str) -> Self {
		ProcessSignal(match signal.to_uppercase().as_str() {
			"HUP"     | "SIGHUP"   | "HANGUP"    => Hangup,
			"INT"     | "SIGINT"   | "INTERRUPT" => Interrupt,
			"QUIT"    | "SIGQUIT"                => Quit,
			"ILL"     | "SIGILL"   | "ILLEGAL"   => Illegal,
			"TRAP"    | "SIGTRAP"                => Trap,
			"ABRT"    | "SIGABRT"  | "ABORT"     => Abort,
			"IOT"     | "SIGIOT"                 => IOT,
			"BUS"     | "SIGBUS"                 => Bus,
			"FPE"     | "SIGFPE"                 => FloatingPointException,
			"KILL"    | "SIGKILL"                => Kill,
			"USR1"    | "SIGUSR1"  | "USER1"     => User1,
			"SEGV"    | "SIGSGEV"                => Segv,
			"USR2"    | "SIGUSR2"  | "USER2"     => User2,
			"PIPE"    | "SIGPIPE"                => Pipe, 
			"ALRM"    | "SIGALRM"  | "ALARM"     => Alarm,
			"TERM"    | "SIGTERM"  | "TERMINATE" => Term,
			"CHLD"    | "SIGCHILD" | "CHILD"     => Child,
			"CONT"    | "SIGCONT"  | "CONTINUE"  => Continue,
			"STOP"    | "SIGSTOP"                => Stop,
			"TSTP"    | "SIGTSTP"                => TSTP,
			"TTIN"    | "SIGTTIN"                => TTIN,
			"TTOU"    | "SIGTTOU"                => TTOU,
			"URG"     | "SIGURG"   | "URGENT"    => Urgent,
			"XCPU"    | "SIGXCPU"                => XCPU,
			"XFSZ"    | "SIGXFSZ"                => XFSZ,
			"VTALRM"  | "SIGVTALRM"              => VirtualAlarm,
			"PROF"    | "SIGPROF"  | "PROFILING" => Profiling,
			"WINCH"   | "SIGWINCH"               => Winch,
			"IO"      | "SIGIO"                  => IO,
			"POLL"    | "SIGPOLL"                => Poll,
			"PWR"     | "SIGPWR"   | "POWER"     => Power,
			"SYS"     | "SIGSYS"                 => Sys,
			other => {
				eprintln!("Unknown signal '{}'. Try bkill --help", other);
				process::exit(1);
			}
		})
	}
}
