// SPDX-License-Identifier: MPL-2.0
use crate::{
	bindings::playlist::PlaylistsProxy,
	error::{Error, Result},
};
use std::{
	fmt::{self, Display},
	ops::Deref,
	str::FromStr,
};
use zbus::{names::OwnedBusName, Connection};

pub struct Playlists {
	proxy: PlaylistsProxy<'static>,
}

impl Playlists {
	/// Creates a new instance of the `org.mpris.MediaPlayer2.Playlists` interface.
	pub async fn new(connection: &Connection, name: OwnedBusName) -> Result<Self> {
		PlaylistsProxy::builder(connection)
			.destination(name)?
			.build()
			.await
			.map(Self::from)
			.map_err(Error::from)
	}
}

impl Deref for Playlists {
	type Target = PlaylistsProxy<'static>;

	fn deref(&self) -> &Self::Target {
		&self.proxy
	}
}

impl From<PlaylistsProxy<'static>> for Playlists {
	fn from(proxy: PlaylistsProxy<'static>) -> Self {
		Self { proxy }
	}
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum PlaylistOrdering {
	/// Alphabetical ordering by name, ascending.
	Alphabetical,
	/// Ordering by creation date, oldest first.
	CreationDate,
	/// Ordering by last modified date, oldest first.
	ModifiedDate,
	/// Ordering by date of last playback, oldest first.
	LastPlayDate,
	/// A user-defined ordering.
	UserDefined,
}

impl FromStr for PlaylistOrdering {
	type Err = Error;

	fn from_str(s: &str) -> Result<Self> {
		match s.to_lowercase().trim() {
			"alphabetical" => Ok(Self::Alphabetical),
			"created" => Ok(Self::CreationDate),
			"modified" => Ok(Self::ModifiedDate),
			"played" => Ok(Self::LastPlayDate),
			"user" => Ok(Self::UserDefined),
			_ => Err(Error::InvalidEnum {
				got: s.to_string(),
				expected: &["Alphabetical", "Created", "Modified", "Played", "User"],
			}),
		}
	}
}

impl Display for PlaylistOrdering {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(
			f,
			"{}",
			match self {
				Self::Alphabetical => "Alphabetical",
				Self::CreationDate => "Created",
				Self::ModifiedDate => "Modified",
				Self::LastPlayDate => "Played",
				Self::UserDefined => "User",
			}
		)
	}
}
