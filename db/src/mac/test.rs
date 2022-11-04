/// Defines a unit test function.
#[macro_export]
macro_rules! define_test {
	($name:ident, $datastore_adapter:expr) => {
		#[tokio::test]
		async fn $name() {
			let datastore_adapter = $datastore_adapter;
			$crate::tests::$name(datastore_adapter).await;
		}
	};
}

/// Use this macro to enable the entire standard test suite.
#[macro_export]
macro_rules! full_test_impl {
	($code:expr) => {
		#[cfg(test)]
		define_test!(should_delete_key, $code);

		#[cfg(test)]
		define_test!(should_set_key, $code);

		#[cfg(test)]
		define_test!(should_put_key, $code);
	};
}
