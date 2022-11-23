#[doc(hidden)]
#[macro_export]
macro_rules! vertex {
    ({ id => $id:expr,label => $label:expr, properties => {$($key:expr => $({ value => $p_value:expr}),*),*}}) => {{

        #[allow(unused_mut)]
        let mut properties  = ::std::collections::HashMap::<String,$crate::Property>::new();
            $(
                $(
                    let p = $crate::Property::new($key,$p_value);
                    properties.insert($key.into(), p);
                )*
            )*
        let v = $crate::Vertex::new($id.into(), $label, properties);
        v
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! edge {
    ({
       id => $id:expr,
       label => $label:expr,
       inV => {  id => $inVId:expr, label => $inVLabel:expr},
       outV => {  id => $outVId:expr, label => $outVLabel:expr} ,
       properties => {$($key:expr => $value:expr),*}}) => {{

           #[allow(unused_mut)]
        let mut properties  = ::std::collections::HashMap::<String,$crate::Property>::new();
            $(
                let p = Property::new($key.into(),$value.into());
                properties.insert($key.into(),p);
            )*
        $crate::Edge::new($id.into(), $label, $inVId.into(),$inVLabel,$outVId.into(),$outVLabel,properties)
    }};
}

// #[cfg(feature = "test-suite")]
#[cfg(test)]
mod test {
	use std::collections::HashMap;

	use crate::Property;
	use crate::Vertex;

	#[test]
	fn vertex_macro() {
		let v1 = vertex!({
			id => 1,
			label => "Person",
			properties => {

			}
		});

		assert_eq!(v1, Vertex::new(1.into(), "Person", HashMap::new()));
	}

	#[test]
	fn vertex_macro_with_properties() {
		let v1 = vertex!({
			id => 1,
			label => "Person",
			properties => {
				"name" => { value => "Enrico"}
			}
		});

		let mut properties = HashMap::new();
		properties.insert(String::from("name"), Property::new("name", "Enrico"));

		assert_eq!(v1, Vertex::new(1.into(), "Person", properties));
	}
}
