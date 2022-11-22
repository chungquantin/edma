use crate::structure::T;

pub enum LabelType {
	Str(String),
	Bool(bool),
	T(T),
}

impl LabelType {
	pub fn bytes(&self) -> Vec<u8> {
		match self {
			LabelType::Str(v) => v.as_bytes().to_vec(),
			_ => unimplemented!(),
		}
	}

	pub fn bytes_len(&self) -> usize {
		self.bytes().len()
	}
}

pub struct Labels(pub Vec<LabelType>);

impl From<&str> for Labels {
	fn from(param: &str) -> Labels {
		Labels(vec![LabelType::Str(String::from(param))])
	}
}

impl From<String> for Labels {
	fn from(param: String) -> Labels {
		Labels(vec![LabelType::Str(param)])
	}
}

impl From<T> for Labels {
	fn from(param: T) -> Labels {
		Labels(vec![LabelType::T(param)])
	}
}

impl From<()> for Labels {
	fn from(_: ()) -> Labels {
		Labels(vec![])
	}
}
impl From<Vec<&str>> for Labels {
	fn from(param: Vec<&str>) -> Labels {
		Labels(param.into_iter().map(|val| LabelType::Str(String::from(val))).collect())
	}
}
impl From<Vec<String>> for Labels {
	fn from(param: Vec<String>) -> Labels {
		Labels(param.into_iter().map(LabelType::Str).collect())
	}
}

impl From<bool> for Labels {
	fn from(param: bool) -> Labels {
		Labels(vec![LabelType::Bool(param)])
	}
}

impl From<(bool, Vec<&str>)> for Labels {
	fn from(param: (bool, Vec<&str>)) -> Labels {
		let mut out: Vec<LabelType> = vec![LabelType::Bool(param.0)];
		out.append(&mut Into::<Labels>::into(param.1).0.drain(..).collect());
		Labels(out)
	}
}

impl From<(bool, T, Vec<&str>)> for Labels {
	fn from(param: (bool, T, Vec<&str>)) -> Labels {
		let mut out: Vec<LabelType> = vec![LabelType::Bool(param.0)];
		out.append(&mut Into::<Labels>::into(param.1).0.drain(..).collect());
		out.append(&mut Into::<Labels>::into(param.2).0.drain(..).collect());
		Labels(out)
	}
}

impl From<(T, Vec<&str>)> for Labels {
	fn from(param: (T, Vec<&str>)) -> Labels {
		let mut out: Vec<LabelType> = vec![LabelType::T(param.0)];
		out.append(&mut Into::<Labels>::into(param.1).0.drain(..).collect());
		Labels(out)
	}
}

macro_rules! impl_into_labels_str {
	($n:expr) => {
		impl From<[&str; $n]> for Labels {
			fn from(param: [&str; $n]) -> Labels {
				Labels(param.iter().map(|s| LabelType::Str(String::from(*s))).collect())
			}
		}
	};
}

impl_into_labels_str!(1);
impl_into_labels_str!(2);
impl_into_labels_str!(3);
impl_into_labels_str!(4);
impl_into_labels_str!(5);
impl_into_labels_str!(6);
impl_into_labels_str!(7);
impl_into_labels_str!(8);
impl_into_labels_str!(9);
impl_into_labels_str!(10);

macro_rules! impl_into_labels_string {
	($n:expr) => {
		impl From<[String; $n]> for Labels {
			fn from(param: [String; $n]) -> Labels {
				Labels(param.iter().map(|val| LabelType::Str(val.clone())).collect())
			}
		}
	};
}

impl_into_labels_string!(1);
impl_into_labels_string!(2);
impl_into_labels_string!(3);
impl_into_labels_string!(4);
impl_into_labels_string!(5);
impl_into_labels_string!(6);
impl_into_labels_string!(7);
impl_into_labels_string!(8);
impl_into_labels_string!(9);
impl_into_labels_string!(10);
