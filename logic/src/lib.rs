#![feature(box_patterns, box_syntax)]
use std::fmt::{self, Write, Display, Formatter};

use consts::*;
use Formula::*;

pub mod consts;
pub mod prelude;

pub enum Formula {
	Var(char),
	Not(Box<Formula>),
	And(Box<Formula>, Box<Formula>),
	Or(Box<Formula>, Box<Formula>),
	Implies(Box<Formula>, Box<Formula>),
	Iff(Box<Formula>, Box<Formula>),
}
impl Formula {
	//pub fn new(s: &str) -> Box<Formula> {
	//	
	//}
}
impl Display for Formula {
	fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
		fn brackets(p: &Formula, f: &mut Formatter) -> Result<(), fmt::Error> {
			if let &Var(_) = p {
				try!(p.fmt(f));
			} else if let &Not(box Var(_)) = p {
				try!(p.fmt(f));
			} else {
				try!(f.write_char('('));
				try!(p.fmt(f));
				try!(f.write_char(')'));
			}
			Ok(())
		}
		
		match self {
			&Var(ref c) => try!(f.write_char(*c)),
			&Not(ref p) => {
				try!(f.write_str(STR_NOT));
				try!(brackets(&p, f));
			},
			&And(ref p, ref q) => {
				try!(brackets(&p, f));
				try!(f.write_str(STR_AND));
				try!(brackets(&q, f));
			},
			&Or(ref p, ref q) => {
				try!(brackets(&p, f));
				try!(f.write_str(STR_OR));
				try!(brackets(&q, f));
			},
			&Implies(ref p, ref q) => {
				try!(brackets(&p, f));
				try!(f.write_str(STR_IF));
				try!(brackets(&q, f));
			},
			&Iff(ref p, ref q) => {
				try!(brackets(&p, f));
				try!(f.write_str(STR_IFF));
				try!(brackets(&q, f));
			}
		}
		
		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use ::Formula;
	
	fn yn(b: bool) -> &'static str {
		if b { "yes" } else { "no" }
	}

	#[test]
	fn test_formula_display() {
		fn test<'a>(f: &'a Formula, s: &'a str) {
			let f_s = format!("{}", f);
			println!("[test_formula_display] {} == {} ? ... {}", f_s, s, yn(f_s == s));
			assert_eq!(f_s, s);
		}
		use ::{var, not, and, or, implies, iff};
		
		test(&*implies(not(and(var('P'), var('Q'))), or(not(var('P')), not(var('Q')))), "(¬(P^Q))->(¬Pv¬Q)");//"(¬(P∧Q))→(¬P∨¬Q)");
		test(&*iff(or(var('P'), var('Q')), or(var('Q'), var('P'))), "(PvQ)<->(QvP)");
	}
}
