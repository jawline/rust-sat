mod sat;

use sat::SAT;
use sat::clause::*;

pub fn main() {
	let mut sat = SAT::new();

	let v1 = sat.variable("v1");
	let v2 = sat.variable("v2");

	let clause = (basic(v1), basic(v2), with_negate(v2, true));

	sat.add_clause(clause);

	println!("Satisfiable: {}", sat.is_sat());
}