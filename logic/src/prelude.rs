pub fn var(c: char) -> Box<::Formula> {
	box ::Formula::Var(c)
}
pub fn not(p: Box<::Formula>) -> Box<::Formula> {
	box ::Formula::Not(p)
}
pub fn and(p: Box<::Formula>, q: Box<::Formula>) -> Box<::Formula> {
	box ::Formula::And(p, q)
}
pub fn or(p: Box<::Formula>, q: Box<::Formula>) -> Box<::Formula> {
	box ::Formula::Or(p, q)
}
pub fn implies(p: Box<::Formula>, q: Box<::Formula>) -> Box<::Formula> {
	box ::Formula::Implies(p, q)
}
pub fn iff(p: Box<::Formula>, q: Box<::Formula>) -> Box<::Formula> {
	box ::Formula::Iff(p, q)
}
