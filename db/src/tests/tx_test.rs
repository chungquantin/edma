use std::str::from_utf8;

use crate::{adapter::DatastoreAdapter, tx::SimpleTransaction};

pub async fn should_set_key<T>(adapter: impl DatastoreAdapter<T>)
where
	T: SimpleTransaction,
{
	let adapter = adapter.spawn();
	let mut tx = adapter.transaction(true).unwrap();

	let key = "mock key";
	let val = "mock value";

	tx.set(key, val).await.unwrap();
	let res = tx.get(key).await.unwrap();
	match res {
		Some(v) => assert_eq!(val, from_utf8(&v).unwrap()),
		None => panic!("Wrong value"),
	}

	let new_val = "mock value 2";
	tx.set(key, new_val).await.unwrap();
	let res = tx.get(key).await.unwrap();
	match res {
		Some(v) => assert_eq!(new_val, from_utf8(&v).unwrap()),
		None => panic!("Wrong value"),
	}
}

pub async fn should_delete_key<T>(adapter: impl DatastoreAdapter<T>)
where
	T: SimpleTransaction,
{
	let adapter = adapter.spawn();
	let mut tx = adapter.transaction(true).unwrap();

	let key = "mock key";
	let val = "mock value";

	tx.set(key, val).await.unwrap();
	tx.del(key).await.unwrap();
	let res = tx.get(key).await.unwrap();
	assert_eq!(res, None);
}

pub async fn should_put_key<T>(adapter: impl DatastoreAdapter<T>)
where
	T: SimpleTransaction,
{
	let adapter = adapter.spawn();
	let mut tx = adapter.transaction(true).unwrap();

	let key = "mock key";
	let val = "mock value";

	tx.put(key, val).await.unwrap();
	let res = tx.get(key).await.unwrap();
	match res {
		Some(v) => assert_eq!(val, from_utf8(&v).unwrap()),
		None => panic!("Wrong value"),
	}

	assert!(tx.put(key, val).await.is_err());
}
