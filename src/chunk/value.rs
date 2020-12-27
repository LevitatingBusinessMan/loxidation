/*
The book uses a union here, with an enum to track the type.
In rust Enum variants can have their own values, which in practice is like bundling a union with an enum.
So using an enum I get pretty much the same behavior, but I am using a safe type
Memory is exactly the same, both are a union and a single byte identifier
*/

#[allow(non_camel_case_types)]
pub type number = f64;

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
	BOOL(bool),
	NUMBER(number),
	NIL
}

impl From<bool> for Value {
	fn from(value: bool) -> Value {
		Value::BOOL(value)
	}
}

impl From<number> for Value {
	fn from(value: number) -> Value {
		Value::NUMBER(value)
	}
}

impl From<Value> for number {
	fn from(value: Value) -> number {
		match value {
			Value::NUMBER(num) => num,
			_ => unreachable!()
		}
	}
}

impl From<Value> for bool {
	fn from(value: Value) -> bool {
		match value {
			Value::BOOL(bool) => bool,
			_ => unreachable!()
		}
	}
}

//According to https://doc.rust-lang.org/std/convert/trait.From.html
//Into types should be auto generated? Sure k then

impl ToString for Value {
	fn to_string(&self) -> String {
		return match self {
			Value::NUMBER(number) => number.to_string(),
			Value::BOOL(bool) => bool.to_string(),
			Value::NIL => "nil".to_owned(),
		}
	}
}

/* UNION IMPLEMENTATION
enum ValueType {
	BOOL,
	NIL,
	NUMBER
}

#[allow(non_camel_case_types)]
type number = f64;

union ValueUnion {
	bool: bool,
	number: number
}

struct Value {
	vtype: ValueType,
	union: ValueUnion
}

impl From<bool> for Value {
	fn from(value: bool) -> Value {
		Value {
			vtype: ValueType::BOOL,
			union: ValueUnion {
				bool: value
			}
		}
	}
}

impl From<number> for Value {
	fn from(value: number) -> Value {
		Value {
			vtype: ValueType::BOOL,
			union: ValueUnion {
				number: value
			}
		}
	}
}


impl Into<bool> for Value {
	fn into(self) -> bool {
		self.union.bool
	}
}

impl Into<number> for Value {
	fn into(self) -> number {
		self.union.number
	}
}


impl Value {
	fn print(&self) -> String {
		return match self.vtype {
			ValueType::BOOL => <Self as Into<bool>>::into(*self).to_string(),
			ValueType::NUMBER => <Self as Into<number>>::into(*self).to_string(),
			ValueType::NIL => "nil".to_owned(),
		}
	}
}
*/