use std::fmt::Display;

pub enum Users {
	All,
	Some(Vec<UidType>),
}

#[derive(Debug, PartialEq, Eq)]
pub enum UidType {
	Current,
	Root,
	Other(u32),
}

impl Display for UidType {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Current => write!(f, "You"),
			Self::Root => write!(f, "Root"),
			Self::Other(u) => write!(f, "{}", u), // TODO GET NAME OF UID OWNER
		}
	}
}

#[derive(Debug)]
pub struct Process<'a> {
	pub name:  &'a str,
	pub uid:   UidType,
	pub count: u32,
}
