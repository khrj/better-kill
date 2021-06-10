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

#[derive(Debug)]
pub struct Process<'a> {
	pub name:  &'a str,
	pub uid:   UidType,
	pub count: u32,
}
