/// Defines a unit test function.
#[macro_export]
#[cfg(feature = "test-suite")]
macro_rules! define_test {
	($name:ident, $code:expr) => {
		#[tokio::test]
		async fn $name() {
			$crate::tests::$name($code).await;
		}
	};
}

/// Use this macro to enable the entire standard test suite.
#[macro_export]
#[cfg(feature = "test-suite")]
macro_rules! full_adapter_test_impl {
	($code:expr) => {
		#[cfg(test)]
		define_test!(should_delete_key, $code);
		#[cfg(test)]
		define_test!(should_set_key, $code);
		#[cfg(test)]
		define_test!(should_put_key, $code);
	};
}

#[macro_export]
#[cfg(feature = "test-suite")]
macro_rules! full_database_test_impl {
	($test_name: ident, $code:expr) => {
		#[cfg(test)]
		mod $test_name {
			define_test!(vertex_with_property, $code);
			define_test!(vertex_with_many_property, $code);
			define_test!(vertices_iter, $code);
			define_test!(vertex_property, $code);
			define_test!(multiple_new_vertex, $code);
			define_test!(vertex_has_step, $code);
		}
	};
}
