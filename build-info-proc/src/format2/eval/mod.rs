use super::syntax::{AtomicExpr, Expr};
use super::types::Type;

use std::any::Any;
use std::fmt::Debug;

mod suffixed;
use suffixed::Suffixed;

pub(crate) trait ValueTrait: Any + Debug + Suffixed {}

impl<T: Any + Debug + Suffixed> ValueTrait for T {}

#[derive(Debug)]
pub(crate) struct Value {
	pub(crate) value: Box<dyn ValueTrait>,
	pub(crate) r#type: Type,
}

pub(crate) trait Eval {
	fn eval(&self) -> Value;
}

impl Eval for AtomicExpr {
	fn eval(&self) -> Value {
		match self {
			AtomicExpr::LitInt(value, r#type, _) => Value {
				value: Box::new(value.clone()),
				r#type: Type::Integer(*r#type),
			},
			AtomicExpr::LitStr(value, _) => Value {
				value: Box::new(value.clone()),
				r#type: Type::String,
			},
			AtomicExpr::BuildInfo(_) => Value {
				value: Box::new(crate::deserialize_build_info()),
				r#type: Type::BuildInfo,
			},
			AtomicExpr::Parenthesized(expr, _) => expr.eval(),
		}
	}
}

impl Eval for Expr {
	fn eval(&self) -> Value {
		let value = self.atom.eval();

		for _suffix in &self.suffixes {
			unimplemented!();
		}

		value
	}
}
