pub struct Argument {
	key: &'static String,
	value: Option<&'static String>
}

impl Argument {

	pub fn get_key(&self) -> &'static String {
		return self.key;
	}

	pub fn has_value(&self) -> bool {
		return self.value != None;
	}

	pub fn get_value(&self) -> &'static String {
		assert!( self.has_value(), "Tried to get value of a valueless Argument! key:{}", &self.key );
		return self.value.unwrap();
	}
}

pub struct CommandLine {
	arguments: Vec<Argument>
}

impl CommandLine {
	fn new() -> Self {
		let args: Vec<String> = std::env::args().collect();
		let mut arguments = Vec::new();
		// TODO: MAKE THIS WORK

		return CommandLine { arguments }
	}

	pub fn get_params(&self) -> &Vec<Argument> {
		return &self.arguments;
	}
}

pub fn get_instance() -> CommandLine {
	return CommandLine::new();
}