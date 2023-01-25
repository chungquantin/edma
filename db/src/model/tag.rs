use std::collections::HashMap;

type TagKey = &'static str;
type TagValue = String;
type TagBucketInner = HashMap<TagKey, TagValue>;

#[derive(Clone, Default)]
pub struct TagBucket(TagBucketInner);

#[macro_export]
macro_rules! tag {
	($($key: expr => $value: expr),*) => {{
		#[allow(unused_mut)]
  let mut map = std::collections::HashMap::default();
  $(
   map.insert($key, $value);
  )*
  $crate::TagBucket::new(map)
 }};
}

impl TagBucket {
	pub fn new(map: TagBucketInner) -> TagBucket {
		TagBucket(map)
	}

	pub fn get(&self, key: TagKey) -> Option<TagValue> {
		self.0.get(key).cloned()
	}

	pub fn insert(&mut self, key: TagKey, val: TagValue) {
		self.0.insert(key, val);
	}

	pub fn unchecked_get(&self, key: TagKey) -> TagValue {
		self.0.get(key).unwrap().clone()
	}

	pub fn get_bytes(&self, key: TagKey) -> Option<Vec<u8>> {
		let wrapped_value = self.0.get(key).cloned();
		if let Some(v) = wrapped_value {
			return Some(v.as_bytes().to_vec());
		}
		None
	}
}
