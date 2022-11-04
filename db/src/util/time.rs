use chrono::Utc;

pub fn get_now() -> i64 {
	Utc::now().timestamp()
}
