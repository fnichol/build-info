use anyhow::{anyhow, Result};
use format_buf::format;
use num_bigint::BigInt;

use std::collections::VecDeque;

use build_info_common::{
	BuildInfo, CompilerChannel, CompilerInfo, CrateInfo, DateTime, GitInfo, Identifier, Utc, Version, VersionControl,
};

use super::super::syntax::Suffix;
use super::{Type, Value};

pub(crate) trait Suffixed {
	fn apply_suffix(&self, suffix: &Suffix) -> Result<Value>;
}

impl Suffixed for BuildInfo {
	fn apply_suffix(&self, suffix: &Suffix) -> Result<Value> {
		match suffix {
			Suffix::Field(ref id) if id == "timestamp" => Ok(Value{value: Box::new(self.timestamp), r#type: Type::DateTimeUtc}),
			Suffix::Field(ref id) if id == "profile" => unimplemented!(),
			Suffix::Field(ref id) if id == "crate_info" => unimplemented!(),
			Suffix::Field(ref id) if id == "compiler" => unimplemented!(),
			Suffix::Field(ref id) if id == "version_control" => unimplemented!(),
			_ => Err(anyhow!("Suffix {:#?} is not valid on build_info::BuildInfo", suffix)),
		}
	}
}

impl Suffixed for BigInt {
	fn apply_suffix(&self, suffix: &Suffix) -> Result<Value> {
		Err(anyhow!("Suffix {:#?} is not valid on an integer", suffix))
	}
}

impl Suffixed for String {
	fn apply_suffix(&self, suffix: &Suffix) -> Result<Value> {
		Err(anyhow!("Suffix {:#?} is not valid on a string", suffix))
	}
}

impl Suffixed for DateTime<Utc> {
	fn apply_suffix(&self, suffix: &Suffix) -> Result<Value> {
		Err(anyhow!("Suffix {:#?} is not valid on a DateTime<Utc>", suffix))
	}
}
