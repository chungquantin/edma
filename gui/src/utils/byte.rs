#[derive(Clone)]
pub enum LayoutVariant {
	String,
	Int32,
	Int64,
	Uuid,
	Float32,
	Float64,
	Boolean,
	Bytes,
}

impl Default for LayoutVariant {
	fn default() -> Self {
		LayoutVariant::Bytes
	}
}

#[derive(Clone, Default)]
pub struct ByteLayout {
	variant: LayoutVariant,
	from: usize,
	to: usize,
}

#[derive(Clone)]
pub struct LayoutTemplate {
	pub name: String,
	pub layout: Vec<ByteLayout>,
}

impl LayoutTemplate {
	pub fn new(name: &str, layout: Vec<ByteLayout>) -> Self {
		LayoutTemplate {
			name: name.to_string(),
			layout,
		}
	}
}

impl ByteLayout {
	pub fn variant(&self) -> LayoutVariant {
		self.variant.clone()
	}

	pub fn from(&self) -> usize {
		self.from
	}

	pub fn to(&self) -> usize {
		self.to
	}

	pub fn build(&self) -> Self {
		self.clone()
	}

	pub fn with_variant(&mut self, variant: LayoutVariant) -> &mut Self {
		self.variant = variant;
		self
	}

	pub fn range(&mut self, from: usize, to: usize) -> &mut Self {
		self.from = from;
		self.to = to;
		self
	}
}

pub trait FromLayoutVariant {
	fn from_variant(&self, variant: LayoutVariant) -> String;
}

impl FromLayoutVariant for Vec<u8> {
	fn from_variant(&self, variant: LayoutVariant) -> String {
		let default_value = format!("{:?}", self);
		match variant {
			LayoutVariant::String => {
				let c = String::from_utf8(self.clone());
				match c {
					Ok(v) => v,
					Err(_) => format!("Unable to parse: {}", default_value),
				}
			}
			LayoutVariant::Int64 => {
				if self.len() < 8 {
					return format!("Unable to parse: {}", default_value);
				}
				let c = i64::from_be_bytes(self[0..8].try_into().unwrap());
				c.to_string()
			}
			LayoutVariant::Int32 => {
				if self.len() < 4 {
					return format!("Unable to parse: {}", default_value);
				}
				let c = i32::from_be_bytes(self[0..4].try_into().unwrap());
				c.to_string()
			}
			LayoutVariant::Float64 => {
				if self.len() < 8 {
					return format!("Unable to parse: {}", default_value);
				}
				let c = f64::from_be_bytes(self[0..8].try_into().unwrap());
				c.to_string()
			}
			LayoutVariant::Float32 => {
				if self.len() < 4 {
					return format!("Unable to parse: {}", default_value);
				}
				let c = f32::from_be_bytes(self[0..4].try_into().unwrap());
				c.to_string()
			}
			LayoutVariant::Boolean => {
				if self.iter().any(|b| b > &1) {
					return format!("Unable to parse: {}", default_value);
				}
				let cmp_byte = |b: u8| {
					if b != 0 {
						"True".to_string()
					} else {
						"False".to_string()
					}
				};
				if self.len() == 1 {
					return cmp_byte(self[0]);
				}
				let mut value = vec![];
				for b in self.iter() {
					value.push(cmp_byte(*b));
				}
				format!("{:?}", value)
			}
			_ => default_value,
		}
	}
}
