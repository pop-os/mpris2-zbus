// SPDX-License-Identifier: MPL-2.0
use crate::{
	bindings::{media_player::MediaPlayer2Proxy, player::PlayerProxy},
	error::{Error, Result},
	handle_optional,
	media_player::MediaPlayer,
	metadata::Metadata,
	track::Track,
};
use std::{
	fmt::{self, Display},
	ops::Deref,
	str::FromStr,
};
use time::Duration;
use zbus::{names::OwnedBusName, Connection};

#[derive(Debug, Clone)]
pub struct Player {
	proxy: PlayerProxy<'static>,
}

impl Player {
	/// Creates a new instance of the `org.mpris.MediaPlayer2.Player` interface.
	pub async fn new(connection: &Connection, name: OwnedBusName) -> Result<Self> {
		PlayerProxy::builder(connection)
			.destination(name)?
			.build()
			.await
			.map(Self::from)
			.map_err(Error::from)
	}

	/// Returns this player's `org.mpris.MediaPlayer2` instance
	pub async fn media_player(&self) -> Result<MediaPlayer> {
		let proxy = MediaPlayer2Proxy::builder(self.proxy.connection())
			.destination(self.proxy.destination().to_owned())?
			.build()
			.await?;
		Ok(proxy.into())
	}

	/// Seeks the specified duration.
	pub async fn seek(&self, duration: Duration) -> Result<bool> {
		if self.proxy.can_seek().await? {
			self.proxy
				.seek(duration.whole_microseconds() as i64)
				.await?;
			Ok(true)
		} else {
			Ok(false)
		}
	}

	/// Sets the current track position.
	///
	/// If `track` does not match the id of the currently-playing track, the call is ignored as "stale".
	pub async fn set_position(&self, track: &Track, position: Duration) -> Result<()> {
		self.proxy
			.set_position(track, position.whole_microseconds() as i64)
			.await
			.map_err(Error::from)
	}

	/// How far into the current track the player is.
	///
	/// Not all players support this, and it will return None if this is the case.
	pub async fn position(&self) -> Result<Option<Duration>> {
		handle_optional(self.proxy.position().await.map(Duration::microseconds))
	}

	/// Gets the current playback status of the player.
	pub async fn playback_status(&self) -> Result<PlaybackStatus> {
		self.proxy
			.playback_status()
			.await
			.map_err(Error::from)
			.and_then(|status| PlaybackStatus::from_str(&status))
	}

	/// Returns the current rate of playback.
	///
	/// Not all players support this, and it will return None if this is the case.
	pub async fn rate(&self) -> Result<Option<f64>> {
		handle_optional(self.proxy.rate().await)
	}

	/// Returns the minimum supported rate for the player.
	///
	/// Not all players support this, and it will return None if this is the case.
	pub async fn minimum_rate(&self) -> Result<Option<f64>> {
		handle_optional(self.proxy.minimum_rate().await)
	}

	/// Returns the minimum supported rate for the player.
	///
	/// Not all players support this, and it will return None if this is the case.
	pub async fn maximum_rate(&self) -> Result<Option<f64>> {
		handle_optional(self.proxy.maximum_rate().await)
	}

	/// Returns the range of playback rates available for the player.
	///
	/// Not all players support this, and it will return None if this is the case.
	pub async fn available_rates(&self) -> Result<Option<std::ops::RangeInclusive<f64>>> {
		let minimum = match self.minimum_rate().await? {
			Some(min) => min,
			None => return Ok(None),
		};
		let maximum = match self.maximum_rate().await? {
			Some(max) => max,
			None => return Ok(None),
		};
		Ok(Some(minimum..=maximum))
	}

	/// Returns the metadata for the player.
	pub async fn metadata(&self) -> Result<Metadata> {
		self.proxy
			.metadata()
			.await
			.map(|metadata| metadata.into())
			.map_err(Error::from)
	}
}

impl Deref for Player {
	type Target = PlayerProxy<'static>;

	fn deref(&self) -> &Self::Target {
		&self.proxy
	}
}

impl From<PlayerProxy<'static>> for Player {
	fn from(proxy: PlayerProxy<'static>) -> Self {
		Self { proxy }
	}
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum PlaybackStatus {
	Playing,
	Paused,
	Stopped,
}

impl FromStr for PlaybackStatus {
	type Err = Error;

	fn from_str(s: &str) -> Result<Self> {
		match s.to_lowercase().trim() {
			"playing" => Ok(Self::Playing),
			"paused" => Ok(Self::Paused),
			"stopped" => Ok(Self::Stopped),
			_ => Err(Error::InvalidEnum {
				got: s.to_string(),
				expected: &["Playing", "Paused", "Stopped"],
			}),
		}
	}
}

impl Display for PlaybackStatus {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(
			f,
			"{}",
			match self {
				Self::Playing => "Playing",
				Self::Paused => "Paused",
				Self::Stopped => "Stopped",
			}
		)
	}
}
