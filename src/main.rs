mod sat;

use sat::{ClausePart, Clause, SAT};

pub fn main() {
	let mut sat = SAT::new();

	let v1 = sat.variable("v1");
	let v2 = sat.variable("v2");

	let clause = (ClausePart::new(v1, false), ClausePart::new(v2, false), ClausePart::new(v2, false));

	sat.add_clause(clause);

	println!("Satisfiable: {}", sat.is_sat());
}