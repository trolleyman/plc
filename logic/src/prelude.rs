pub use ::Formula::*;

pub fn var(c: char) -> Box<::Formula> {
	box Var(c)
}
pub fn not(p: Box<::Formula>) -> Box<::Formula> {
	box Not(p)
}
pub fn and(p: Box<::Formula>, q: Box<::Formula>) -> Box<::Formula> {
	box And(p, q)
}
pub fn or(p: Box<::Formula>, q: Box<::Formula>) -> Box<::Formula> {
	box Or(p, q)
}
pub fn implies(p: Box<::Formula>, q: Box<::Formula>) -> Box<::Formula> {
	box Implies(p, q)
}
pub fn iff(p: Box<::Formula>, q: Box<::Formula>) -> Box<::Formula> {
	box Iff(p, q)
}
