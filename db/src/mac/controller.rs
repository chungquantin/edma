pub enum Controller {
	LabelController,
	VertexController,
}

impl Controller {
	pub fn get_cf(&self) -> &str {
		match self {
			Controller::LabelController => "labels:v1",
			Controller::VertexController => "vertices:v1",
		}
	}
}

macro_rules! impl_controller {
	(get $datastore_adapter: ty; from $m: ident for $c_name: ident) => {
		pub struct $c_name {
			pub ds: $datastore_adapter,
			pub cf: &'static str,
		}

		impl $c_name {
			pub fn new(ds: DatastoreManager) -> Result<Self, Error> {
				Ok($c_name {
					ds: ds.$m() as $datastore_adapter,
					cf: Controller::$c_name.get_cf(),
				})
			}
		}
	};
}
