// SPDX-License-Identifier: MPL-2.0

#[derive(Debug, thiserror::Error)]
pub enum Error {
	/// Invalid enum variant when converting from String.
	#[error("Invalid enum variant: {got}, expected something in {expected:?}")]
	InvalidEnum {
		got: String,
		expected: &'static [&'static str],
	},

	#[error("Tried to convert extract a {wanted}, but it was actually {actual}")]
	IncorrectVariant {
		wanted: &'static str,
		actual: &'static str,
	},

	/// A zbus error.
	#[error("zbus error: {0}")]
	Zbus(zbus::Error),

	/// A zbus::fdo error.
	#[error("zbus fdo error: {0}")]
	Fdo(zbus::fdo::Error),
}

impl From<zbus::fdo::Error> for Error {
	fn from(err: zbus::fdo::Error) -> Self {
		match err {
			zbus::fdo::Error::ZBus(err) => Self::Zbus(err),
			_ => Self::Fdo(err),
		}
	}
}

impl From<zbus::Error> for Error {
	fn from(err: zbus::Error) -> Self {
		match err {
			zbus::Error::FDO(err) => Self::Fdo(*err),
			_ => Self::Zbus(err),
		}
	}
}

pub type Result<T> = std::result::Result<T, Error>;
