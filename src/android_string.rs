#[derive(Debug, PartialEq)]
pub struct AndroidString {
	name: String, 
	value: String, 
	is_translatable: bool
}

impl AndroidString {
	pub fn new(name: String, value: String, is_translatable: bool) -> AndroidString {
		AndroidString {
			name, 
			value, 
			is_translatable
		}
	}
}
