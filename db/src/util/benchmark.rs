use std::time::Instant;

pub struct BenchmarkTracker {
	fn_name: String,
	start_at: Instant,
}

impl BenchmarkTracker {
	pub fn start(name: &str) -> Self {
		let now = Instant::now();
		BenchmarkTracker {
			fn_name: name.to_string(),
			start_at: now,
		}
	}

	pub fn stop(&mut self) {
		let elapsed_time = self.start_at.elapsed();
		#[cfg(feature = "debug-suite")]
		println!("[{:?}] took {:?} seconds.", self.fn_name, elapsed_time);
	}
}
