use std::vec::Vec;

pub struct ClausePart {
	variable: usize,
	negate: bool
}

impl ClausePart {
	pub fn new(var: usize, negate: bool) -> ClausePart {
		ClausePart{variable: var, negate: negate}
	}
}

pub type Clause = (ClausePart, ClausePart, ClausePart);

pub struct SAT {
	pub variables: Vec<String>,
	pub clauses: Vec<Clause>
}

impl SAT {
	pub fn new() -> SAT {
		SAT{variables: Vec::new(), clauses: Vec::new()}
	}

	pub fn variable(&mut self, name: &str) -> usize {
		self.variables.push(format!("{}", name));
		self.variables.len() - 1
	}

	pub fn add_clause(&mut self, clause: Clause) {
		self.clauses.push(clause);
	}

	fn check_part(part: &ClausePart, cur_sat: &Vec<bool>) -> bool {
		match part.negate {
			false => cur_sat[part.variable],
			true => !cur_sat[part.variable]
		}
	}

	fn check_sat(&self, cur_sat: &Vec<bool>) -> bool {
		for clause in &self.clauses {
			if !(SAT::check_part(&clause.0, cur_sat) && SAT::check_part(&clause.1, cur_sat) && SAT::check_part(&clause.2, cur_sat)) {
				return false;
			}
		}
		true
	}

	fn sat_next(&self, i: usize, cur_sat: &mut Vec<bool>) -> bool {
		
		if i == self.variables.len() {
			return self.check_sat(cur_sat);
		}

		cur_sat[i] = false;
		if self.sat_next(i+1, cur_sat) {
			true
		} else {
			cur_sat[i] = true;
			self.sat_next(i+1, cur_sat)
		}
	}

	pub fn is_sat(&self) -> bool {
		let mut cur_sat = Vec::new();

		for _ in 0..self.variables.len() {
			cur_sat.push(false);
		}

		self.sat_next(0, &mut cur_sat)
	}
}
