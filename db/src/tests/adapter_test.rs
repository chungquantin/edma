use std::str::from_utf8;

use crate::{
	constant::{ColumnFamily, COLUMN_FAMILIES},
	tag, DatastoreAdapter, SimpleTransaction,
};

pub async fn should_set_key(adapter: impl DatastoreAdapter) {
	let adapter = adapter.spawn();
	let cf_name = COLUMN_FAMILIES.get(&ColumnFamily::TestSuite).unwrap();
	let mut tx = adapter.transaction(true).await.unwrap();
	let tags = tag!("column_family" => cf_name.clone());

	let key = "mock key";
	let val = "mock value";

	tx.set(key, val, tags.clone()).await.unwrap();
	assert!(tx.exi(key, tags.clone()).await.unwrap());
	let res = tx.get(key, tags.clone()).await.unwrap();
	match res {
		Some(v) => assert_eq!(val, from_utf8(&v).unwrap()),
		None => panic!("Wrong value"),
	}

	let new_val = "mock value 2";
	tx.set(key, new_val, tags.clone()).await.unwrap();
	assert!(tx.exi(key, tags.clone()).await.unwrap());
	let res = tx.get(key, tags.clone()).await.unwrap();
	match res {
		Some(v) => assert_eq!(new_val, from_utf8(&v).unwrap()),
		None => panic!("Wrong value"),
	}
}

pub async fn should_delete_key(adapter: impl DatastoreAdapter) {
	let adapter = adapter.spawn();
	let tags = tag!("column_family" => "test_suite:v1".to_string());
	let mut tx = adapter.transaction(true).await.unwrap();

	let key = "mock key";
	let val = "mock value";

	tx.set(key, val, tags.clone()).await.unwrap();
	assert!(tx.exi(key, tags.clone()).await.unwrap());
	tx.del(key, tags.clone()).await.unwrap();
	let res = tx.get(key, tags.clone()).await.unwrap();
	assert_eq!(res, None);
}

pub async fn should_put_key(adapter: impl DatastoreAdapter) {
	let adapter = adapter.spawn();
	let tags = tag!("column_family" => "test_suite:v1".to_string());
	let mut tx = adapter.transaction(true).await.unwrap();

	let key = "mock key";
	let val = "mock value";

	tx.put(key, val, tags.clone()).await.unwrap();
	assert!(tx.exi(key, tags.clone()).await.unwrap());
	let res = tx.get(key, tags.clone()).await.unwrap();
	match res {
		Some(v) => assert_eq!(val, from_utf8(&v).unwrap()),
		None => panic!("Wrong value"),
	}

	assert!(tx.put(key, val, tags.clone()).await.is_err());
}
