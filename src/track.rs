// SPDX-License-Identifier: MPL-2.0
use std::{
	cmp::Ordering,
	fmt::{self, Display},
	ops::Deref,
};
use zbus::zvariant::{ObjectPath, OwnedObjectPath};

/// A reference to an MPRIS track.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Track(OwnedObjectPath);

impl Track {
	pub fn into_inner(self) -> OwnedObjectPath {
		self.0
	}

	pub fn into_static_path(self) -> ObjectPath<'static> {
		self.0.into_inner().into_owned()
	}
}

impl Deref for Track {
	type Target = OwnedObjectPath;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl<'a> AsRef<ObjectPath<'a>> for Track {
	fn as_ref(&self) -> &ObjectPath<'a> {
		&self.0
	}
}

impl<'a, T> From<T> for Track
where
	T: Into<ObjectPath<'a>>,
{
	fn from(path: T) -> Self {
		Self(path.into().into())
	}
}

impl PartialOrd for Track {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		self.0.as_str().partial_cmp(other.0.as_str())
	}
}

impl Ord for Track {
	fn cmp(&self, other: &Self) -> Ordering {
		self.0.as_str().cmp(other.0.as_str())
	}
}

impl Display for Track {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.0.as_str())
	}
}
