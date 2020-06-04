#![allow(dead_code)]

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub(crate) enum Type {
	Bool,
	Char,
	Integer(IntegerType),
	FloatingPoint(FloatingPointType),
	String,

	BuildInfo,
	DateTimeUtc,
}

impl Type {
	pub fn is_integer(&self) -> bool {
		match self {
			Type::Integer(_) => true,
			_ => false,
		}
	}

	pub fn is_floating_point(&self) -> bool {
		match self {
			Type::FloatingPoint(_) => true,
			_ => false,
		}
	}
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub(crate) enum IntegerType {
	Unspecified,
	I8,
	U8,
	I16,
	U16,
	I32,
	U32,
	I64,
	U64,
	I128,
	U128,
	ISize,
	USize,
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub(crate) enum FloatingPointType {
	F32,
	F64,
}
