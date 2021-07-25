use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};
use log::trace;

use crate::types::{Process, SearchMatch};

pub fn filter<'a>(processes: &'a Vec<Process>, name: &'a str) -> SearchMatch<'a> {
	let matcher = SkimMatcherV2::default();

	let matches: Vec<_> = processes
		.into_iter()
		.filter(|process| {
			let is_match = matcher.fuzzy_match(process.name, name).is_some();
			trace!("Testing {} with {}: {}", process.name, name, is_match);
			is_match
		})
		.collect();

	if matches.len() == 0 {
		SearchMatch::None
	} else if matches.len() == 1 {
		SearchMatch::Single(matches.first().unwrap())
	} else {
		SearchMatch::Multiple(matches)
	}
}
