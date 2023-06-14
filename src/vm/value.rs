/*
The book uses a union here, with an enum to track the type.
In rust Enum variants can have their own values, which in practice is like bundling a union with an enum.
So using an enum I get pretty much the same behavior, but I am using a safe type
Memory is exactly the same, both are a union and a single byte identifier
*/

#[allow(non_camel_case_types)]
pub type number = f64;

/*STRING?
A string in rust is a Vec<u8>, and a Vec<u8> is just a RawVec<u8> with a length.
So to keep a special struct with it and impl all kinds of methods to manage it is stupid.
*/

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
	BOOL(bool),
	NUMBER(number),
	STRING(String),
	CHAR(char),
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

impl From<String> for Value {
	fn from(value: String) -> Value {
		Value::STRING(value)
	}
}

impl From<char> for Value {
	fn from(value: char) -> Value {
		Value::CHAR(value)
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

impl From<Value> for String {
	fn from(value: Value) -> String {
		match value {
			Value::STRING(string) => string,
			_ => unreachable!()
		}
	}
}

impl From<Value> for char {
	fn from(value: Value) -> char {
		match value {
			Value::CHAR(char) => char,
			_ => unreachable!()
		}
	}
}

//According to https://doc.rust-lang.org/std/convert/trait.From.html
//Should imply ToString, which implies From which implies Into (I think?)
impl std::fmt::Display for Value {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
		write!(f, "{}", match self {
			Value::NUMBER(number) => number.to_string(),
			Value::BOOL(bool) => bool.to_string(),
			Value::STRING(string) => string.clone(),
			Value::CHAR(char) => char.to_string(),
			Value::NIL => "nil".to_owned()
		})
	}
}

impl Value {
	pub fn is_truthy(&self) -> bool {
		return match self {
			Value::BOOL(bool) => *bool,
			Value::NIL => false,
			_ => true
		}
	}

	// Not sure if this is useful in any way
	// Maybe for future types though
	// For now this just works on PartialEq
	pub fn equal(&self, second: Value) -> bool {
		self == &second
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