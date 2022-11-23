use chrono::Utc;

pub fn now() -> i64 {
	Utc::now().timestamp()
}
