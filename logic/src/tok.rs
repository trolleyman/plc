use std::fmt::{self, Write, Display, Formatter};
use std::ops::{Deref, DerefMut};

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Token {
	Char(char),
	Not,
	And,
	Or,
	Implies,
	Iff,
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

#[derive(Clone, PartialEq, Eq)]
pub struct Tokens {
	inner: Vec<Token>
}
impl Tokens {
	pub fn new() -> Tokens {
		Tokens::from_vec(Vec::new())
	}
	
	pub fn from_vec(v: Vec<Token>) -> Tokens {
		Tokens {
			inner: v
		}
	}
	
	/// Create a token vector from a string, normalizing the different forms of operators.
	pub fn from_str(mut s: &str) -> Tokens {
		use Token::*;
		use consts::*;
		
		let mut res = Tokens::from_vec(Vec::with_capacity(s.len()));
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
	
	/// Simplify the token vector. E.g. convert `[Token::Char('-'), Token::Char('>')]` into `[Token::Implies]`
	/// Takes O(n) currently.
	/// Returns the number of tokens removed.
	pub fn simplify(&mut self) -> usize {
		use Token::*;
		use consts::*;
		
		let old_len = self.len();
		let mut res = Vec::with_capacity(self.len());
		{
			let mut ts: &[Token] = self.as_ref();
			while ts.len() > 0 {
				if ts.starts_with(TOK_STR_NOT) {
					ts = &ts[TOK_STR_NOT.len()..];
					res.push(Not);
				} else if ts.starts_with(TOK_STR_PRETTY_NOT) {
					ts = &ts[TOK_STR_PRETTY_NOT.len()..];
					res.push(Not);
				} else if ts.starts_with(TOK_STR_AND) {
					ts = &ts[TOK_STR_AND.len()..];
					res.push(And);
				} else if ts.starts_with(TOK_STR_PRETTY_AND) {
					ts = &ts[TOK_STR_PRETTY_AND.len()..];
					res.push(And);
				} else if ts.starts_with(TOK_STR_OR) {
					ts = &ts[TOK_STR_OR.len()..];
					res.push(Or);
				} else if ts.starts_with(TOK_STR_PRETTY_OR) {
					ts = &ts[TOK_STR_PRETTY_OR.len()..];
					res.push(Or);
				} else if ts.starts_with(TOK_STR_IF) {
					ts = &ts[TOK_STR_IF.len()..];
					res.push(Implies);
				} else if ts.starts_with(TOK_STR_PRETTY_IF) {
					ts = &ts[TOK_STR_PRETTY_IF.len()..];
					res.push(Implies);
				} else if ts.starts_with(TOK_STR_IFF) {
					ts = &ts[TOK_STR_IFF.len()..];
					res.push(Iff);
				} else if ts.starts_with(TOK_STR_IFF2) {
					ts = &ts[TOK_STR_IFF2.len()..];
					res.push(Iff);
				} else if ts.starts_with(TOK_STR_PRETTY_IFF) {
					ts = &ts[TOK_STR_PRETTY_IFF.len()..];
					res.push(Iff);
				} else {
					res.push(ts[0]);
					ts = &ts[1..];
				}
			}
		}
		*self = Tokens{ inner: res };
		old_len - self.len()
	}
}
impl Deref for Tokens {
	type Target = Vec<Token>;
	fn deref(&self) -> &Vec<Token> {
		&self.inner
	}
}
impl DerefMut for Tokens {
	fn deref_mut(&mut self) -> &mut Vec<Token> {
		&mut self.inner
	}
}
impl Display for Tokens {
	fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
		let mut s = String::with_capacity(self.len() + 16);
		for t in self.iter() {
			if !f.alternate() {
				try!(write!(s, "{}", t));
			} else {
				try!(write!(s, "{:#}", t));
			}
		}
		try!(f.pad(&s));
		Ok(())
	}
}
