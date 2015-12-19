
extern crate gtk;

extern crate logic;

use logic::prelude::*;

fn main() {
	let r = implies(not(and(var('P'), var('Q'))), or(not(var('P')), not(var('Q'))));
	println!("R: {}", r);
}
