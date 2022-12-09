mod byte;
mod file;

pub use byte::*;
pub use file::*;

use crate::events::Key;

pub fn get_key_char(k: Key) -> char {
	if let Key::Char(key) = k {
		key
	} else {
		todo!()
	}
}
