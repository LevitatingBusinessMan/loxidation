use crate::scanner::tokens::TokenType;
use crate::compiler;
use crate::compiler::Compiler;

pub enum Precedence {
	None = 0,
	Assignment,  // =
	Or,          // or
	And,         // and
	Equality,    // == !=
	Comparison,  // < > <= >=
	Term,        // + -
	Factor,      // * /
	Unary,       // ! -
	Call,        // . ()
	Primary
}

//Is this the best way to do this? Who cares?
impl Into<u32> for Precedence {
	fn into(self) -> u32 {
		self as u32
	}
}

pub(super) struct ParseRule {
	pub prefix: Option<for<'r> fn(&'r mut compiler::Compiler)>,
	pub infix: Option<for<'r> fn(&'r mut compiler::Compiler) >,
	pub precedence: Precedence
}

macro_rules! parse_rule {
	(both => $prefix:ident,$infix:ident,$precedence:ident) => {
		ParseRule {prefix: Some(Compiler::$prefix), infix: Some(Compiler::$infix), precedence: Precedence::$precedence}
	};
	(prefix => $prefix:ident,$precedence:ident) => {
		ParseRule {prefix: Some(Compiler::$prefix), infix: None, precedence: Precedence::$precedence}
	};
	(infix => $infix:ident,$precedence:ident) => {
		ParseRule {prefix: None, infix: Some(Compiler::$infix), precedence: Precedence::$precedence}
	};
	(none) => {
		ParseRule {prefix: None, infix: None, precedence: Precedence::None}
	};
}

pub(super) fn get_rule(ttype: TokenType) -> ParseRule {
	match ttype {
		TokenType::LEFT_PAREN => parse_rule!(prefix => grouping, None),
		TokenType::MINUS => parse_rule!(both => unary,binary,Term),
		TokenType::PLUS => parse_rule!(infix => binary,Term),
		TokenType::SLASH => parse_rule!(infix => binary,Factor),
		TokenType::ASTERISK => parse_rule!(infix => binary,Factor),
		TokenType::NUMBER => parse_rule!(prefix => number,None),
		_ => parse_rule!(none)
	}
}
