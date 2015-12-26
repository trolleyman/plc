use std::fmt::{self, Write, Display, Formatter};

pub enum Token {
	Char(char),
	Not,
	And,
	Or,
	Implies,
	Iff,
}
impl Token {
	/// Create a token vector from a string, normalizing the different forms of operators.
	pub fn from_str(mut s: &str) -> Vec<Token> {
		use Token::*;
		use consts::*;
		
		let mut res = Vec::with_capacity(s.len());
		while s.len() > 0 {
			if s.starts_with(STR_NOT) {
				s = &s[STR_NOT.len()..];
				res.push(Not);
			} else if s.starts_with(STR_PRETTY_NOT) {
				s = &s[STR_PRETTY_NOT.len()..];
				res.push(Not);
			} else if s.starts_with(STR_AND) {
				s = &s[STR_AND.len()..];
				res.push(And);
			} else if s.starts_with(STR_PRETTY_AND) {
				s = &s[STR_PRETTY_AND.len()..];
				res.push(And);
			} else if s.starts_with(STR_OR) {
				s = &s[STR_OR.len()..];
				res.push(Or);
			} else if s.starts_with(STR_PRETTY_OR) {
				s = &s[STR_PRETTY_OR.len()..];
				res.push(Or);
			} else if s.starts_with(STR_IF) {
				s = &s[STR_IF.len()..];
				res.push(Implies);
			} else if s.starts_with(STR_PRETTY_IF) {
				s = &s[STR_PRETTY_IF.len()..];
				res.push(Implies);
			} else if s.starts_with(STR_IFF) {
				s = &s[STR_IFF.len()..];
				res.push(Iff);
			} else if s.starts_with(STR_PRETTY_IFF) {
				s = &s[STR_PRETTY_IFF.len()..];
				res.push(Iff);
			} else {
				let c = s.chars().next().unwrap();
				s = &s[c.len_utf8()..];
				res.push(Char(c));
			}
		}
		
		res
	}
}

impl Display for Token {
	fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
		use Token::*;
		use consts::*;
		
		let (not, and, or, implies, iff) = if !f.alternate() {
			(STR_NOT, STR_AND, STR_OR, STR_IF, STR_IFF)
		} else {
			(STR_PRETTY_NOT, STR_PRETTY_AND, STR_PRETTY_OR, STR_PRETTY_IF, STR_PRETTY_IFF)
		};
		
		match self {
			&Char(ref c) => c.fmt(f),
			&Not         => f.write_str(not),
			&And         => f.write_str(and),
			&Or          => f.write_str(or),
			&Implies     => f.write_str(implies),
			&Iff         => f.write_str(iff),
		}
	}
}
