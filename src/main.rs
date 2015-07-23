mod sat;

use sat::SAT;
use sat::clause;

pub fn main() {
	let mut sat = SAT::new();

	let (v1, v2) = (sat.variable("v1"), sat.variable("v2"));

	let clause = (clause::positive(v1), clause::positive(v2), clause::positive(v1));
	let clause2 = (clause::negative(v1), clause::negative(v2), clause::negative(v2));

	sat.add_clause(clause);
	sat.add_clause(clause2);

	let (can_sat, cur_sat) = sat.attempt();

	println!("Satisfiable: {}", can_sat);
	if can_sat {
		sat.print_mapping(cur_sat);
	}
}
