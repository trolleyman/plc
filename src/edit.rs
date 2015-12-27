use std::fmt::{self, Write, Debug, Formatter};
use std::mem;
use std::ops::{Deref, DerefMut};

use gtk::signal::Inhibit;
use gdk::EventKey;

use logic::Token;

#[derive(Clone)]
pub struct Lines {
	inner: Vec<Line>
}
impl Lines {
	/// Creates a lines object with one line.
	pub fn new() -> Lines {
		Lines {
			inner: vec![Line::new(0)]
		}
	}
	/// Creates a lines object from a vector.
	pub fn from_vec(v: Vec<Line>) -> Lines {
		Lines {
			inner: v
		}
	}
	
	/// Inserts `nl` at `nl.no` in `lines`, and updates all line numbers in `lines`
	pub fn insert_line(&mut self, nl: Line) {
		let no = nl.no;
		for l in self.iter_mut() {
			if l.no >= no {
				l.no += 1;
			}
		}
		self.insert(no, nl);
	}
	/// Deletes the line at `no` in `lines`, and updates all line numbers in `lines`
	pub fn delete_line(&mut self, no: usize) {
		self.inner.remove(no);
		for l in self.iter_mut() {
			if l.no >= no {
				l.no -= 1;
			}
		}
	}
}
impl Deref for Lines {
	type Target = Vec<Line>;
	fn deref(&self) -> &Vec<Line> {
		&self.inner
	}
}
impl DerefMut for Lines {
	fn deref_mut(&mut self) -> &mut Vec<Line> {
		&mut self.inner
	}
}

#[derive(Clone)]
pub struct Line {
	/// The line number of the proof, starting at 0. It is only visually where everything is incremented
	pub no: usize,
	/// A step of the proof. e.g. (P\^Q)->P. This is a vector of tokens that can be invalid.
	pub step: Vec<Token>,
	/// A token string representing the method of the proof.
	pub method: Vec<Token>,
	/// Line numbers that this depends on. Line numbers start at 0.
	pub deps: Vec<usize>,
}
impl Line {
	/// Constructs an empty line
	pub fn new(no: usize) -> Line {
		Line {
			no: no,
			step: Vec::new(),
			method: Vec::new(),
			deps: Vec::new(),
		}
	}
	/// Constructs a line with the specified tokens in the `step` field.
	pub fn with_step(no: usize, step: Vec<Token>) -> Line {
		Line {
			no: no,
			step: step,
			method: Vec::new(),
			deps: Vec::new(),
		}
	}
	pub fn full(no: usize, step: Vec<Token>, method: Vec<Token>, deps: Vec<usize>) -> Line {
		Line {
			no: no,
			step: step,
			method: method,
			deps: deps,
		}
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
			try!(write!(dep_str, "{}", dep + 1));
			if i < self.deps.len() - 1 {
				dep_str.push_str(", ")
			}
		}
		
		let mut method_str = String::with_capacity(self.method.len() + 8);
		if c.no == self.no && c.col == Col::Method && (c.i <= self.method.len() || self.method.len() == 0) {
			let (a, b) = self.method.split_at(c.i);
			if !f.alternate() {
				for t in a {
					try!(write!(method_str, "{}", t));
				}
				method_str.push('|');
				for t in b {
					try!(write!(method_str, "{}", t));
				}
			} else {
				for t in a {
					try!(write!(method_str, "{:#}", t));
				}
				method_str.push('|');
				for t in b {
					try!(write!(method_str, "{:#}", t));
				}
			}
		} else {
			if !f.alternate() {
				for t in self.method.iter() {
					try!(write!(method_str, "{}", t));
				}
			} else {
				for t in self.method.iter() {
					try!(write!(method_str, "{:#}", t));
				}
			}
		}
		f.pad(&format!("{: >3}. {: <20} {: <10} {{{}}}", self.no + 1, step_str, method_str, dep_str))
	}
	
	/// True if `self.step` and `self.method` are empty
	pub fn is_empty(&self) -> bool {
		self.step.is_empty() && self.method.is_empty()
	}
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Col {
	Step,
	Method,
}
#[derive(Debug, PartialEq, Eq)]
pub struct Cursor {
	/// Line number
	no: usize,
	/// Column
	col: Col,
	/// Index of token/string
	i: usize,
}
impl Cursor {
	pub fn new() -> Cursor {
		Cursor {
			no: 0,
			col: Col::Step,
			i: 0,
		}
	}
	/// Move the cursor to the right. Wrap at end of line. Error at end of text.
	pub fn right(&mut self, lines: &[Line]) -> Result<(), ()> {
		let l = match lines.get(self.no) {
			Some(l) => l,
			None    => { *self = Cursor::new(); return Err(()); },
		};
		self.i += 1;
		match self.col {
			Col::Step => {
				if self.i > l.step.len() {
					self.col = Col::Method;
					self.i = 0;
				}
				Ok(())
			}
			Col::Method => {
				if self.i > l.method.len() {
					if self.no < lines.len() - 1 {
						self.no += 1;
						self.col = Col::Step;
						self.i = 0;
						Ok(())
					} else {
						self.i -= 1;
						Err(())
					}
				} else {
					Ok(())
				}
			}
		}
	}
	/// Move the cursor to the left. Wrap at start of line. Error at start of text.
	pub fn left(&mut self, lines: &[Line]) -> Result<(), ()> {
		let l = match lines.get(self.no) {
			Some(l) => l,
			None    => { *self = Cursor::new(); return Err(()); },
		};
		
		if self.i == 0 {
			match self.col {
				Col::Step => {
					if self.no > 0 {
						self.no -= 1;
						self.col = Col::Method;
						self.i = match lines.get(self.no) {
							Some(l) => l.method.len(),
							None    => { *self = Cursor::new(); return Err(()); },
						};
					}
				}
				Col::Method => {
					self.col = Col::Step;
					self.i = l.step.len();
				}
			}
		} else {
			self.i -= 1;
		}
		Ok(())
	}
	
	/// Returns true if at the end of `Col::Step` or `Col::Method`
	fn is_eol(&self, lines: &Lines) -> bool {
		self.i == match self.col {
			Col::Step   => lines[self.no].step.len(),
			Col::Method => lines[self.no].method.len(),
		}
	}
	
	/// Delete the character in front of the cursor. If on an empty line && at the en, delete line. If no chars in front, error.
	pub fn delete(&mut self, lines: &mut Lines) -> Result<(), ()> {
		if lines[self.no].is_empty() && self.col == Col::Method {
			if lines.len() == 1 {
				return Err(());
			}
			// Delete the line
			lines.delete_line(self.no);
			
			if self.no != 0 { self.no -= 1; };
			self.i = 0;
			
			Ok(())
		} else if self.is_eol(lines) {
			Err(())
		} else {
			// Delete char in front
			match self.col {
				Col::Step   => { let _ = lines[self.no].step.remove(self.i); },
				Col::Method => { let _ = lines[self.no].method.remove(self.i); },
			}
			Ok(())
		}
	}
	/// Add a newline to `lines` if the cursor is at the end of the line.
	pub fn newline(&mut self, lines: &mut Lines) -> Result<(), ()> {
		if !self.is_eol(lines) {
			// Don't place a newline if the cursor isn't at the end of the line.
			return Err(());
		}
		
		lines.insert_line(Line::new(self.no + 1));
		
		self.no += 1;
		self.i = 0;
		
		Ok(())
	}
}

pub struct Editor {
	lines: Lines,
	cursor: Cursor,
}

impl<'a> Editor {
	/// Constructs a new editor
	pub fn new() -> Editor {
		Editor {
			lines: Lines::from_vec(vec![Line::full(0, vec![Token::Char('P')], Token::from_str("Premise"), vec![0]),
						Line::full(1, vec![Token::Not, Token::Not, Token::Char('P')], Token::from_str("¬I 1"), vec![0])]),
			cursor: Cursor::new(),
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
			key::Delete => {
				let _ = self.cursor.delete(&mut self.lines);
			},
			key::Return => {
				let _ = self.cursor.newline(&mut self.lines);
			},
			_ => {
				return Inhibit(false);
			},
		}
		println!(" *** Editor *** - Cursor: {:?} \n{:?}", self.cursor, self);
		Inhibit(true)
	}
}
impl Debug for Editor {
	fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
		for l in self.lines.iter() {
			try!(l.fmt_cursor(f, &self.cursor));
			try!(writeln!(f, ""));
		}
		Ok(())
	}
}
