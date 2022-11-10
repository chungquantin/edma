use std::str::from_utf8;

use crate::{adapter::DatastoreAdapter, tx::SimpleTransaction};

pub async fn should_set_key(adapter: impl DatastoreAdapter) {
	let adapter = adapter.spawn();
	let cf = Some("test_suite:v1".into());
	let mut tx = adapter.transaction(true).unwrap();

	let key = "mock key";
	let val = "mock value";

	tx.set(cf.clone(), key, val).await.unwrap();
	assert!(tx.exi(cf.clone(), key).await.unwrap());
	let res = tx.get(cf.clone(), key).await.unwrap();
	match res {
		Some(v) => assert_eq!(val, from_utf8(&v).unwrap()),
		None => panic!("Wrong value"),
	}

	let new_val = "mock value 2";
	tx.set(cf.clone(), key, new_val).await.unwrap();
	assert!(tx.exi(cf.clone(), key).await.unwrap());
	let res = tx.get(cf.clone(), key).await.unwrap();
	match res {
		Some(v) => assert_eq!(new_val, from_utf8(&v).unwrap()),
		None => panic!("Wrong value"),
	}
}

pub async fn should_delete_key(adapter: impl DatastoreAdapter) {
	let adapter = adapter.spawn();
	let cf = Some("test_suite:v1".into());
	let mut tx = adapter.transaction(true).unwrap();

	let key = "mock key";
	let val = "mock value";

	tx.set(cf.clone(), key, val).await.unwrap();
	assert!(tx.exi(cf.clone(), key).await.unwrap());
	tx.del(cf.clone(), key).await.unwrap();
	let res = tx.get(cf.clone(), key).await.unwrap();
	assert_eq!(res, None);
}

pub async fn should_put_key(adapter: impl DatastoreAdapter) {
	let adapter = adapter.spawn();
	let cf = Some("test_suite:v1".into());
	let mut tx = adapter.transaction(true).unwrap();

	let key = "mock key";
	let val = "mock value";

	tx.put(cf.clone(), key, val).await.unwrap();
	assert!(tx.exi(cf.clone(), key).await.unwrap());
	let res = tx.get(cf.clone(), key).await.unwrap();
	match res {
		Some(v) => assert_eq!(val, from_utf8(&v).unwrap()),
		None => panic!("Wrong value"),
	}

	assert!(tx.put(cf.clone(), key, val).await.is_err());
}
