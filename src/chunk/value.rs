pub type Value = f64;

pub trait ValueMethods {
	fn print(&self) -> String;
}

impl ValueMethods for Value {
	fn print(&self) -> String {
		format!("{}", self).to_owned()
	}
}
