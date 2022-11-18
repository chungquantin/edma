use crate::GValue;

#[derive(Debug, PartialEq, Clone)]
pub struct Bytecode {
	source_instructions: Vec<Instruction>,
	step_instructions: Vec<Instruction>,
}

impl Default for Bytecode {
	fn default() -> Bytecode {
		Bytecode {
			source_instructions: vec![],
			step_instructions: vec![],
		}
	}
}
impl Bytecode {
	pub fn new() -> Bytecode {
		Default::default()
	}

	pub fn add_source(&mut self, source_name: String, args: Vec<GValue>) {
		self.source_instructions.push(Instruction::new(source_name, args));
	}
	pub fn add_step(&mut self, step_name: String, args: Vec<GValue>) {
		self.step_instructions.push(Instruction::new(step_name, args));
	}

	pub fn steps(&self) -> &Vec<Instruction> {
		&self.step_instructions
	}

	pub fn sources(&self) -> &Vec<Instruction> {
		&self.source_instructions
	}
}

lazy_static! {
	pub static ref WRITE_OPERATORS: Vec<&'static str> =
		vec!["addV", "property", "addE", "from", "to", "drop"];
}

#[derive(Debug, PartialEq, Clone)]
pub struct Instruction {
	operator: String,
	args: Vec<GValue>,
}

impl Instruction {
	pub fn new(operator: String, args: Vec<GValue>) -> Instruction {
		Instruction {
			operator,
			args,
		}
	}

	pub fn operator(&self) -> &String {
		&self.operator
	}

	pub fn args(&self) -> &Vec<GValue> {
		&self.args
	}
}
