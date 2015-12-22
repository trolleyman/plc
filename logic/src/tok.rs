use std::fmt::{self, Write, Display, Formatter};

pub enum Token {
	Num(i64),
	Var(char),
	Not,
	And,
	Or,
	Implies,
	Iff,
	Comma,
}

impl Display for Token {
	fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
		use Token::*;
		use consts::*;
		
		let (not, and, or, implies, iff, comma) = if !f.alternate() {
			(STR_NOT, STR_AND, STR_OR, STR_IF, STR_IFF, STR_COMMA)
		} else {
			(STR_PRETTY_NOT, STR_PRETTY_AND, STR_PRETTY_OR, STR_PRETTY_IF, STR_PRETTY_IFF, STR_PRETTY_COMMA)
		};
		
		match self {
			&Num(ref v) => v.fmt(f),
			&Var(ref c) => c.fmt(f),
			&Not        => f.write_str(not),
			&And        => f.write_str(and),
			&Or         => f.write_str(or),
			&Implies    => f.write_str(implies),
			&Iff        => f.write_str(iff),
			&Comma      => f.write_str(comma),
		}
	}
}
