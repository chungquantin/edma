use uuid::Uuid;

#[derive(Clone, Debug)]
pub enum LayoutVariant {
	String,
	Int32,
	Int64,
	UuidV4,
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

macro_rules! impl_convert_string {
	($(($code:ident, $str:expr)),*) => {
impl LayoutVariant {
	pub fn from_string(s: &str) -> LayoutVariant {
		match s {
		$(
			$str => LayoutVariant::$code,
		)*
		_ => unimplemented!(),
	}
	}
	pub fn to_string(&self) -> String {
		match self {
		$(
				LayoutVariant::$code => $str,
			)*
	}.to_string()
	}
}
	};
}

impl_convert_string!(
	(String, "String"),
	(Int32, "Int32"),
	(Int64, "Int64"),
	(UuidV4, "UuidV4"),
	(Float32, "Float32"),
	(Float64, "Float64"),
	(Boolean, "Boolean"),
	(Bytes, "Bytes")
);

#[derive(Clone, Debug)]
pub struct ByteLayout {
	pub variant: LayoutVariant,
	pub name: String,
	pub from: usize,
	pub to: usize,
}

impl Default for ByteLayout {
	fn default() -> Self {
		Self {
			variant: Default::default(),
			name: "*".to_string(),
			from: usize::MIN,
			to: usize::MAX,
		}
	}
}

#[derive(Clone, Debug, Default)]
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

	pub fn set_name(&mut self, name: &str) {
		self.name = name.to_string();
	}

	pub fn push_layout(&mut self, layout: ByteLayout) {
		self.layout.push(layout);
	}
}

impl ByteLayout {
	pub fn build(&self) -> Self {
		self.clone()
	}

	pub fn with_variant(&mut self, variant: LayoutVariant) -> &mut Self {
		self.variant = variant;
		self
	}

	pub fn with_name(&mut self, name: String) -> &mut Self {
		self.name = name;
		self
	}

	pub fn with_range(&mut self, from: usize, to: usize) -> &mut Self {
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
			LayoutVariant::UuidV4 => {
				let mut vec: [u8; 16] = Default::default();
				vec.copy_from_slice(&self[0..16]);
				let uuid = Uuid::from_bytes(vec);
				uuid.to_string()
			}
			_ => default_value,
		}
	}
}
