use std::fmt::{self, Write, Debug, Formatter};
use std::mem;

use gtk::signal::Inhibit;
use gdk::EventKey;

use logic::Token;

pub struct Line {
	/// The line number of the proof, starting at 1.
	no: usize,
	/// A step of the proof. e.g. (P^Q)->P. This is a vector of tokens that can be invalid.
	step: Vec<Token>,
	/// A string representing the method of the proof. This is guarenteed to be in the format
	/// <methodName> + <space> + <derivedFromLines>
	method: String,
	//
	deps: Vec<usize>,
}
impl Line {
	/// Constructs an empty line
	pub fn new(no: usize) -> Line {
		Line {
			no: no,
			step: vec![Token::Not, Token::Var('P')],
			method: String::new(),
			deps: Vec::new(),
		}
	}
	/// Gets the line number of the proof
	pub fn no(&self) -> usize {
		self.no
	}
	/// Gets the tokens that represent a step of the proof
	pub fn step(&self) -> &[Token] {
		&self.step
	}
	/// Gets a string that represents a method of the proof
	pub fn method(&self) -> &str {
		&self.method
	}
	
	pub fn fmt_cursor(&self, f: &mut Formatter, c: &Cursor) -> Result<(), fmt::Error> {
		let mut step_str = String::with_capacity(self.step.len() + 16);
		if !f.alternate() {
			for (i, t) in self.step.iter().enumerate() {
				if c.no == self.no && c.col == Col::Step && c.i == i {
					step_str.push('|');
				}
				try!(write!(step_str, "{}", t));
			}
		} else {
			for (i, t) in self.step.iter().enumerate() {
				if c.no == self.no && c.col == Col::Step && c.i == i {
					step_str.push('|');
				}
				try!(write!(step_str, "{:#}", t));
			}
		}
		if c.no == self.no && c.col == Col::Step && c.i == self.step.len() {
			step_str.push('|');
		}
		let mut dep_str = String::with_capacity(self.deps.len() * 2);
		for (i, dep) in self.deps.iter().enumerate() {
			try!(write!(dep_str, "{}", dep));
			if i < self.deps.len() - 1 {
				dep_str.push_str(", ")
			}
		}
		f.pad(&format!("{}.\t{}\t{}\t{{{}}}", self.no, step_str, self.method, dep_str))
	}
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Col {
	Step,
	Method,
}
pub struct Cursor {
	/// Line number
	no: usize,
	/// Column
	col: Col,
	/// Index of token/string
	i: usize,
}
impl Cursor {
	pub fn new(no: usize) -> Cursor {
		Cursor {
			no: no,
			col: Col::Step,
			i: 0,
		}
	}
	pub fn right(&mut self, lines: &[Line]) -> Result<(), ()> {
		let l = match lines.get(self.no) {
			Some(l) => l,
			None    => { *self = Cursor::new(0); return Err(()); },
		};
		self.i += 1;
		match self.col {
			Col::Step => {
				if self.i > l.step().len() {
					self.col = Col::Method;
					self.i = 0;
				}
				Ok(())
			}
			Col::Method => {
				if self.i > l.method().len() {
					if self.no < lines.len() {
						self.no += 1;
						self.col = Col::Step;
						self.i = 0;
						Ok(())
					} else {
						Err(())
					}
				} else {
					Ok(())
				}
			}
		}
	}
	pub fn left(&mut self, lines: &[Line]) -> Result<(), ()> {
		let l = match lines.get(self.no) {
			Some(l) => l,
			None    => { *self = Cursor::new(0); return Err(()); },
		};
		
		if self.i == 0 {
			match self.col {
				Col::Step => {
					if self.no > 1 {
						self.no -= 1;
						self.col = Col::Method;
						self.i = match lines.get(self.no) {
							Some(l) => l.method().len(),
							None    => { *self = Cursor::new(0); return Err(()); },
						};
					}
				}
				Col::Method => {
					self.col = Col::Step;
					self.i = l.step().len();
				}
			}
		} else {
			self.i -= 1;
		}
		Ok(())
	}
}

pub struct Editor {
	lines: Vec<Line>,
	cursor: Cursor,
}

impl<'a> Editor {
	/// Constructs a new editor
	pub fn new() -> Editor {
		Editor {
			lines: vec![Line::new(1)],
			cursor: Cursor::new(1),
		}
	}
	/// Gets a ref to the line numbered `v` in the proof.
	pub fn line(&'a self, v: usize) -> Option<&'a Line> {
		self.lines.get(v - 1)
	}
	/// Gets the number of lines in the proof.
	pub fn lines(&self) -> usize {
		self.lines.len()
	}
	/// Handles the input given to it, and whether to pass the input on or not.
	pub fn handle_input(&mut self, e: &EventKey) -> Inhibit {
		use gdk::enums::key;
		use gdk::{keyval_to_unicode, keyval_name};
		
		let c = keyval_to_unicode(e.keyval).unwrap_or(' ');
		let name = keyval_name(e.keyval).unwrap_or(" ".to_string());
		println!("keypress: {0:#08x} : {1} : {2}", e.keyval, c, name);
		
		match unsafe { mem::transmute(e.keyval) } {
			key::Right => {
				let _ = self.cursor.right(&self.lines);
			},
			key::Left => {
				let _ = self.cursor.left(&self.lines);
			},
			_ => {
				return Inhibit(false);
			},
		}
		println!(" *** Editor *** \n{:?}", self);
		Inhibit(true)
	}
}
impl Debug for Editor {
	fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
		for l in self.lines.iter() {
			try!(l.fmt_cursor(f, &self.cursor))
		}
		Ok(())
	}
}
